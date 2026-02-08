use alloc::{boxed::Box, collections::BTreeMap, string::String, vec::Vec};
use java_constants::FieldAccessFlags;

use hashbrown::{HashSet, hash_set::Entry};

use crate::{ClassDefinition, ClassInstance, Field, JavaValue, Jvm, class_loader::Class, thread::JvmThread};

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

    all_class_instances.difference(&reachable_objects).cloned().collect()
}

fn find_static_reachable_objects(jvm: &Jvm, class: &Class, reachable_objects: &mut HashSet<Box<dyn ClassInstance>>) {
    let fields = find_all_fields(jvm, &*class.definition);
    for field in fields {
        if !field.access_flags().contains(FieldAccessFlags::STATIC) {
            continue;
        }

        let descriptor = field.descriptor();

        if (descriptor.starts_with('L') && descriptor.ends_with(';')) || descriptor.starts_with('[') {
            let value = class.definition.get_static_field(&*field).unwrap();
            if let JavaValue::Object(Some(value)) = value {
                find_reachable_objects(jvm, &value, reachable_objects);
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

    let name = object.class_definition().name();
    if name.starts_with('[') {
        if name.starts_with("[L") || name.starts_with("[[") {
            // is object array
            let array = object.as_array_instance().unwrap();
            let values = array.load(0, array.length()).unwrap();

            for value in values {
                if let JavaValue::Object(Some(value)) = value {
                    find_reachable_objects(jvm, &value, reachable_objects);
                }
            }
        }
        // do nothing for primitive arrays
    } else {
        let fields = find_all_fields(jvm, &*object.class_definition());
        for field in fields {
            if field.access_flags().contains(FieldAccessFlags::STATIC) {
                continue;
            }

            let descriptor = field.descriptor();

            if (descriptor.starts_with('L') && descriptor.ends_with(';')) || descriptor.starts_with('[') {
                let value = object.get_field(&*field).unwrap();
                if let JavaValue::Object(Some(value)) = value {
                    find_reachable_objects(jvm, &value, reachable_objects);
                }
            }
        }
    }
}

fn find_all_fields(jvm: &Jvm, class_definition: &dyn ClassDefinition) -> Vec<Box<dyn Field>> {
    let result = class_definition.fields();
    let super_class_name = class_definition.super_class_name();

    if let Some(x) = super_class_name {
        let super_class = jvm.get_class(&x).unwrap();
        let super_fields = find_all_fields(jvm, &*super_class.definition);
        result.into_iter().chain(super_fields).collect()
    } else {
        result
    }
}
