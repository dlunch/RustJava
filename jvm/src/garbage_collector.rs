use alloc::{boxed::Box, collections::BTreeMap, sync::Arc, vec::Vec};
use core::mem::forget;

use bytemuck::cast_slice;
use hashbrown::{hash_set::Entry, HashMap, HashSet};
use parking_lot::Mutex;

use crate::{thread::JvmThread, ClassInstance, JavaType, JavaValue, Jvm};

// XXX java/util/Vector, java/util/HashMap internal..
type RustVector = Arc<Mutex<Vec<Box<dyn ClassInstance>>>>;
type RustHashMap = Arc<Mutex<HashMap<i32, Vec<(Box<dyn ClassInstance>, Box<dyn ClassInstance>)>>>>;

pub fn determine_garbage(
    jvm: &Jvm,
    threads: &BTreeMap<u64, JvmThread>,
    all_class_instances: &HashSet<Box<dyn ClassInstance>>,
    classes: Vec<Box<dyn ClassInstance>>,
) -> Vec<Box<dyn ClassInstance>> {
    let mut reachable_objects = classes.into_iter().collect::<HashSet<_>>();

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

#[allow(clippy::borrowed_box)]
fn find_reachable_objects(jvm: &Jvm, object: &Box<dyn ClassInstance>, reachable_objects: &mut HashSet<Box<dyn ClassInstance>>) {
    let entry = reachable_objects.entry(object.clone());
    if let Entry::Occupied(_) = entry {
        return;
    }
    entry.insert();

    let fields = object.class_definition().fields();

    for field in fields {
        match field.r#type() {
            JavaType::Class(_) => {
                let value = object.get_field(&*field).unwrap();
                if let JavaValue::Object(Some(value)) = value {
                    find_reachable_objects(jvm, &value, reachable_objects);

                    // XXX we have to deal with java value wrapped inside rust type e.g. java.util.Vector, java.util.Hashtable
                    if jvm.is_instance(&*value, "java/util/Vector") {
                        let members = vector_members(&*value);
                        assert!(members.len() == 1);
                        for member in members {
                            find_reachable_objects(jvm, &member, reachable_objects);
                        }
                    } else if jvm.is_instance(&*value, "java/util/Hashtable") {
                        let members = hashtable_members(&*value);
                        for member in members {
                            find_reachable_objects(jvm, &member, reachable_objects);
                        }
                    }
                }
            }
            JavaType::Array(_) => {
                let value = object.get_field(&*field).unwrap();
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
            _ => {}
        }
    }
}

// Same as Jvm's one but without async
fn get_rust_object_field<T: Clone>(object: &dyn ClassInstance, field_name: &str) -> T {
    let field = object.class_definition().field(field_name, "Ljava/lang/Object;", true).unwrap();
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

fn vector_members(vector: &dyn ClassInstance) -> Vec<Box<dyn ClassInstance>> {
    let rust_vector: RustVector = get_rust_object_field(vector, "raw");

    let rust_vector = rust_vector.lock();
    rust_vector.iter().cloned().collect()
}

fn hashtable_members(hashtable: &dyn ClassInstance) -> Vec<Box<dyn ClassInstance>> {
    let rust_hashmap: RustHashMap = get_rust_object_field(hashtable, "raw");

    let rust_hashmap = rust_hashmap.lock();
    rust_hashmap.iter().flat_map(|(_, v)| v.iter().map(|x| x.1.clone())).collect()
}
