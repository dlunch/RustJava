use alloc::{boxed::Box, collections::BTreeMap, string::String, vec::Vec};
use jvm_types::FieldAccessFlags;

use hashbrown::HashSet;

use crate::{ClassDefinition, ClassInstance, Field, JavaValue, Jvm, class_loader::Class, thread::JvmThread};

pub fn determine_garbage(
    jvm: &Jvm,
    threads: &BTreeMap<u64, JvmThread>,
    global_references: &BTreeMap<u64, Box<dyn ClassInstance>>,
    all_class_instances: &HashSet<Box<dyn ClassInstance>>,
    classes: &BTreeMap<String, Class>,
    interned_strings: &[Box<dyn ClassInstance>],
    root_identities: &HashSet<usize>,
) -> anyhow::Result<Vec<Box<dyn ClassInstance>>> {
    let mut reachable_objects = HashSet::new();

    for class in classes.values() {
        find_reachable_objects(jvm, &class.java_class(), &mut reachable_objects)?;
        find_static_reachable_objects(jvm, class, &mut reachable_objects)?;
    }

    for object in threads
        .values()
        .flat_map(|thread| thread.iter_frame().flat_map(|stack| stack.local_variables()))
    {
        find_reachable_objects(jvm, object, &mut reachable_objects)?;
    }

    for object in threads.values().filter_map(|thread| thread.java_thread()) {
        find_reachable_objects(jvm, object, &mut reachable_objects)?;
    }

    for object in global_references.values() {
        find_reachable_objects(jvm, object, &mut reachable_objects)?;
    }

    for object in interned_strings {
        find_reachable_objects(jvm, object, &mut reachable_objects)?;
    }

    for object in all_class_instances.iter().filter(|object| root_identities.contains(&object.identity())) {
        find_reachable_objects(jvm, object, &mut reachable_objects)?;
    }

    Ok(all_class_instances
        .iter()
        .filter(|object| !reachable_objects.contains(&object.identity()))
        .cloned()
        .collect())
}

fn find_static_reachable_objects(jvm: &Jvm, class: &Class, reachable_objects: &mut HashSet<usize>) -> anyhow::Result<()> {
    let fields = find_all_fields(jvm, &*class.definition);
    for field in fields {
        if !field.access_flags().contains(FieldAccessFlags::STATIC) {
            continue;
        }

        let descriptor = field.descriptor();

        if (descriptor.starts_with('L') && descriptor.ends_with(';')) || descriptor.starts_with('[') {
            let value = class.definition.get_static_field(&*field)?;
            if let JavaValue::Object(Some(value)) = value {
                find_reachable_objects(jvm, &value, reachable_objects)?;
            }
        }
    }
    Ok(())
}

#[allow(clippy::borrowed_box)]
fn find_reachable_objects(jvm: &Jvm, object: &Box<dyn ClassInstance>, reachable_objects: &mut HashSet<usize>) -> anyhow::Result<()> {
    if !reachable_objects.insert(object.identity()) {
        return Ok(());
    }

    let class_definition = object.class_definition();
    let name = class_definition.name();
    if name.starts_with('[') {
        if name.starts_with("[L") || name.starts_with("[[") {
            // is object array
            let array = object.as_array_instance().unwrap();
            let values = array.load(0, array.length())?;

            for value in values {
                if let JavaValue::Object(Some(value)) = value {
                    find_reachable_objects(jvm, &value, reachable_objects)?;
                }
            }
        }
        // do nothing for primitive arrays
    } else {
        let fields = find_all_fields(jvm, &*class_definition);
        for field in fields {
            if field.access_flags().contains(FieldAccessFlags::STATIC) {
                continue;
            }

            let descriptor = field.descriptor();

            if (descriptor.starts_with('L') && descriptor.ends_with(';')) || descriptor.starts_with('[') {
                let value = object.get_field(&*field)?;
                if let JavaValue::Object(Some(value)) = value {
                    find_reachable_objects(jvm, &value, reachable_objects)?;
                }
            }
        }
    }
    Ok(())
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
