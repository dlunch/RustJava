use alloc::{boxed::Box, collections::BTreeMap, string::String, vec, vec::Vec};
use jvm_types::FieldAccessFlags;

use dyn_clone::clone_box;
use hashbrown::{HashMap, HashSet};

use crate::{ClassDefinition, ClassInstance, Field, JavaValue, class_loader::Class, thread::JvmThread};

pub fn determine_garbage(
    threads: &BTreeMap<u64, JvmThread>,
    global_references: &BTreeMap<u64, Box<dyn ClassInstance>>,
    all_class_instances: &HashSet<Box<dyn ClassInstance>>,
    classes: &BTreeMap<String, Class>,
    interned_strings: &[Box<dyn ClassInstance>],
    root_identities: &HashSet<usize>,
) -> anyhow::Result<Vec<Box<dyn ClassInstance>>> {
    let mut marker = Marker {
        registry: classes,
        reachable_objects: HashSet::new(),
        classes: HashMap::new(),
        references: Vec::new(),
    };

    for class in classes.values() {
        marker.mark(class.java_class().as_ref())?;
        marker.mark_static_fields(class)?;
    }

    for object in threads
        .values()
        .flat_map(|thread| thread.iter_frame().flat_map(|stack| stack.local_variables()))
    {
        marker.mark(object.as_ref())?;
    }

    for object in threads.values().filter_map(|thread| thread.java_thread()) {
        marker.mark(object)?;
    }

    for object in global_references.values() {
        marker.mark(object.as_ref())?;
    }

    for object in interned_strings {
        marker.mark(object.as_ref())?;
    }

    for object in all_class_instances.iter().filter(|object| root_identities.contains(&object.identity())) {
        marker.mark(object.as_ref())?;
    }

    // java.lang.ref: a weak reference is cleared once its referent is no longer strongly reachable
    if let Some(reference_class) = classes.get("java/lang/ref/Reference") {
        let referent = reference_class.definition.field("referent", "Ljava/lang/Object;", false).unwrap();
        for mut object in marker.references {
            if all_class_instances.contains(&object)
                && let JavaValue::Object(Some(value)) = object.get_field(&*referent)?
                && !marker.reachable_objects.contains(&value.identity())
            {
                object.put_field(&*referent, JavaValue::Object(None))?;
            }
        }
    }

    Ok(all_class_instances
        .iter()
        .filter(|object| !marker.reachable_objects.contains(&object.identity()))
        .cloned()
        .collect())
}

// What marking needs to know about a class, resolved once per collection
struct MarkedClass {
    reference_fields: Vec<Box<dyn Field>>,
    is_reference: bool,
}

struct Marker<'a> {
    registry: &'a BTreeMap<String, Class>,
    reachable_objects: HashSet<usize>,
    classes: HashMap<String, MarkedClass>,
    // reachable java.lang.ref.Reference instances
    references: Vec<Box<dyn ClassInstance>>,
}

impl Marker<'_> {
    // inherited static fields belong to the superclass, which is marked on its own
    fn mark_static_fields(&mut self, class: &Class) -> anyhow::Result<()> {
        for field in class.definition.fields() {
            if field.access_flags().contains(FieldAccessFlags::STATIC)
                && is_reference_type(&field.descriptor())
                && let JavaValue::Object(Some(value)) = class.definition.get_static_field(&*field)?
            {
                self.mark(value.as_ref())?;
            }
        }
        Ok(())
    }

    fn mark(&mut self, root: &dyn ClassInstance) -> anyhow::Result<()> {
        if !self.reachable_objects.insert(root.identity()) {
            return Ok(());
        }

        // An explicit stack, since a chain of references can be longer than the native stack is deep
        let mut pending = vec![clone_box(root)];
        while let Some(object) = pending.pop() {
            let mut mark_value = |value| {
                if let JavaValue::Object(Some(value)) = value
                    && self.reachable_objects.insert(value.identity())
                {
                    pending.push(value);
                }
            };

            let class_definition = object.class_definition();
            let name = class_definition.name();
            if let Some(element_type) = name.strip_prefix('[') {
                if is_reference_type(element_type) {
                    let array = object.as_array_instance().unwrap();
                    array.load(0, array.length())?.into_iter().for_each(mark_value);
                }
                continue;
            }

            if !self.classes.contains_key(&*name) {
                let class = MarkedClass::new(&*class_definition, self.registry);
                self.classes.insert(name.clone().into_owned(), class);
            }
            let class = &self.classes[&*name];
            if class.is_reference {
                self.references.push(object.clone());
            }
            for field in &class.reference_fields {
                mark_value(object.get_field(&**field)?);
            }
        }
        Ok(())
    }
}

fn is_reference_type(descriptor: &str) -> bool {
    (descriptor.starts_with('L') && descriptor.ends_with(';')) || descriptor.starts_with('[')
}

impl MarkedClass {
    fn new(definition: &dyn ClassDefinition, registry: &BTreeMap<String, Class>) -> Self {
        let mut reference_fields = Vec::new();
        let mut is_reference = false;
        let mut class = definition;
        loop {
            let declares_referent = class.name() == "java/lang/ref/Reference";
            is_reference |= declares_referent;
            reference_fields.extend(class.fields().into_iter().filter(|field| {
                !field.access_flags().contains(FieldAccessFlags::STATIC)
                    && is_reference_type(&field.descriptor())
                    && !(declares_referent && field.name() == "referent")
            }));
            let Some(parent) = class.super_class_name() else { break };
            // GC holds the registry read lock. Reacquiring it can deadlock if a class loader is waiting to write.
            class = &*registry[&parent].definition;
        }
        Self {
            reference_fields,
            is_reference,
        }
    }
}
