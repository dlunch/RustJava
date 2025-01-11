use alloc::{boxed::Box, collections::BTreeMap, string::String, sync::Arc, vec::Vec};
use core::mem::forget;
use java_constants::FieldAccessFlags;

use bytemuck::cast_slice;
use hashbrown::{hash_set::Entry, HashMap, HashSet};
use parking_lot::Mutex;

use crate::{class_loader::Class, thread::JvmThread, ClassInstance, JavaValue, Jvm};

// XXX java/util/Vector, java/util/HashMap internal..
type RustVector = Arc<Mutex<Vec<Box<dyn ClassInstance>>>>;
type RustHashMap = Arc<Mutex<HashMap<i32, Vec<(Box<dyn ClassInstance>, Box<dyn ClassInstance>)>>>>;

pub fn determine_garbage(
    jvm: &Jvm,
    threads: &BTreeMap<u64, JvmThread>,
    all_class_instances: &HashSet<Box<dyn ClassInstance>>,
    classes: &BTreeMap<String, Class>,
) -> Vec<Box<dyn ClassInstance>> {
    let mut reachable_objects = HashSet::new();

    classes.values().for_each(|x| {
        find_reachable_objects(jvm, &x.java_class(), &mut reachable_objects);
        find_static_reachable_objects(jvm, x, &mut reachable_objects);
    });

    threads
        .iter()
        .flat_map(|(_, thread)| thread.iter_frame().flat_map(|stack| stack.local_variables()))
        .for_each(|x| {
            find_reachable_objects(jvm, x, &mut reachable_objects);
        });

    // HACK we should test if class loader is in use
    for class_instance in all_class_instances.iter() {
        if jvm.is_instance(&**class_instance, "java/lang/ClassLoader") {
            find_reachable_objects(jvm, class_instance, &mut reachable_objects);
        }
    }

    all_class_instances.difference(&reachable_objects).cloned().collect()
}

fn find_static_reachable_objects(jvm: &Jvm, class: &Class, reachable_objects: &mut HashSet<Box<dyn ClassInstance>>) {
    let fields = class.definition.fields();
    for field in fields {
        if !field.access_flags().contains(FieldAccessFlags::STATIC) {
            continue;
        }

        let descriptor = field.descriptor();
        let value = class.definition.get_static_field(&*field).unwrap();

        if descriptor.starts_with("L") && descriptor.ends_with(";") {
            if let JavaValue::Object(Some(value)) = value {
                find_reachable_objects(jvm, &value, reachable_objects);
            }
        } else if descriptor.starts_with("[") {
            if let JavaValue::Object(Some(value)) = value {
                reachable_objects.insert(value.clone());

                let array = value.as_array_instance().unwrap();
                let values = array.load(0, array.length()).unwrap();

                for value in values {
                    if let JavaValue::Object(Some(value)) = value {
                        find_reachable_objects(jvm, &value, reachable_objects);
                    }
                }
            }
        }
    }
}

#[allow(clippy::borrowed_box)]
fn find_reachable_objects(jvm: &Jvm, object: &Box<dyn ClassInstance>, reachable_objects: &mut HashSet<Box<dyn ClassInstance>>) {
    let entry = reachable_objects.entry(object.clone());
    if let Entry::Occupied(_) = entry {
        return;
    }
    entry.insert();

    // XXX we have to deal with java value wrapped inside rust type e.g. java.util.Vector, java.util.Hashtable
    if jvm.is_instance(&**object, "java/util/Vector") {
        let members = vector_members(jvm, &**object);
        for member in members {
            find_reachable_objects(jvm, &member, reachable_objects);
        }
    } else if jvm.is_instance(&**object, "java/util/Hashtable") {
        let members = hashtable_members(jvm, &**object);
        for member in members {
            find_reachable_objects(jvm, &member, reachable_objects);
        }
    }

    let fields = object.class_definition().fields();
    for field in fields {
        if field.access_flags().contains(FieldAccessFlags::STATIC) {
            continue;
        }

        let descriptor = field.descriptor();

        if !descriptor.starts_with("L") && !descriptor.starts_with("[") {
            continue;
        }

        let value = object.get_field(&*field).unwrap();

        if descriptor.starts_with("L") && descriptor.ends_with(";") {
            if let JavaValue::Object(Some(value)) = value {
                find_reachable_objects(jvm, &value, reachable_objects);
            }
        } else if descriptor.starts_with("[") {
            if let JavaValue::Object(Some(value)) = value {
                reachable_objects.insert(value.clone());

                let array = value.as_array_instance().unwrap();
                let values = array.load(0, array.length()).unwrap();

                for value in values {
                    if let JavaValue::Object(Some(value)) = value {
                        find_reachable_objects(jvm, &value, reachable_objects);
                    }
                }
            }
        }
    }
}

// Same as Jvm's one but without async
fn get_rust_object_field<T: Clone>(jvm: &Jvm, object: &dyn ClassInstance, field_name: &str) -> T {
    let field = jvm.find_field(&*object.class_definition(), field_name, "[B").unwrap().unwrap();
    let value = object.get_field(&*field).unwrap();
    let buf: Vec<i8> = match value {
        JavaValue::Object(Some(value)) => {
            let value_array = value.as_array_instance().unwrap();

            value_array.load(0, value_array.length()).unwrap().into_iter().map(|x| x.into()).collect()
        }
        _ => panic!("Invalid field type"),
    };

    let rust_raw = usize::from_le_bytes(cast_slice(&buf).try_into().unwrap());

    let rust = unsafe { Box::from_raw(rust_raw as *mut T) };
    let result = (*rust).clone();

    forget(rust); // do not drop box as we still have it in java memory

    result
}

fn vector_members(jvm: &Jvm, vector: &dyn ClassInstance) -> Vec<Box<dyn ClassInstance>> {
    let rust_vector: RustVector = get_rust_object_field(jvm, vector, "raw");

    let rust_vector = rust_vector.lock();
    rust_vector.iter().cloned().collect()
}

fn hashtable_members(jvm: &Jvm, hashtable: &dyn ClassInstance) -> Vec<Box<dyn ClassInstance>> {
    let rust_hashmap: RustHashMap = get_rust_object_field(jvm, hashtable, "raw");

    let rust_hashmap = rust_hashmap.lock();
    rust_hashmap.iter().flat_map(|(_, v)| v.iter().map(|x| x.1.clone())).collect()
}
