use alloc::collections::BTreeMap;

use java_constants::MethodAccessFlags;

use crate::{AttributeInfo, ClassFileError, ClassInfo, ConstantPoolReference, constant_pool::ConstantPoolItem};

enum MemberKind {
    Field,
    Method,
}

pub(crate) fn validate_class(class: &ClassInfo) -> Result<(), ClassFileError> {
    if !is_internal_class_name(&class.this_class)
        || class.super_class.as_ref().is_some_and(|name| !is_internal_class_name(name))
        || class.interfaces.iter().any(|name| !is_internal_class_name(name))
        || !validate_constant_pool(&class.constant_pool)
    {
        return Err(ClassFileError::InvalidFormat);
    }

    for field in &class.fields {
        if !is_field_descriptor(&field.descriptor) {
            return Err(ClassFileError::InvalidFormat);
        }

        let constant_values = field
            .attributes
            .iter()
            .filter_map(|attribute| match attribute {
                AttributeInfo::ConstantValue(value) => Some(value),
                _ => None,
            })
            .collect::<alloc::vec::Vec<_>>();
        if constant_values.len() > 1
            || constant_values.first().is_some_and(|value| {
                !matches!(
                    (field.descriptor.as_str(), *value),
                    ("Z" | "B" | "C" | "S" | "I", ConstantPoolReference::Integer(_))
                        | ("J", ConstantPoolReference::Long(_))
                        | ("F", ConstantPoolReference::Float(_))
                        | ("D", ConstantPoolReference::Double(_))
                        | ("Ljava/lang/String;", ConstantPoolReference::String(_))
                )
            })
        {
            return Err(ClassFileError::InvalidFormat);
        }
    }

    for method in &class.methods {
        if !is_method_descriptor(&method.descriptor) {
            return Err(ClassFileError::InvalidFormat);
        }

        let code_attributes = method
            .attributes
            .iter()
            .filter(|attribute| matches!(attribute, AttributeInfo::Code(_)))
            .count();
        if method.access_flags.intersects(MethodAccessFlags::ABSTRACT | MethodAccessFlags::NATIVE) {
            if code_attributes != 0 {
                return Err(ClassFileError::InvalidFormat);
            }
        } else if code_attributes != 1 {
            return Err(ClassFileError::InvalidFormat);
        }
    }

    Ok(())
}

fn validate_constant_pool(constant_pool: &BTreeMap<u16, ConstantPoolItem>) -> bool {
    constant_pool.values().all(|item| match item {
        ConstantPoolItem::Class { name_index } => constant_pool
            .get(name_index)
            .and_then(ConstantPoolItem::utf8)
            .is_some_and(|name| is_class_constant_name(&name)),
        ConstantPoolItem::String { string_index } => constant_pool.get(string_index).and_then(ConstantPoolItem::utf8).is_some(),
        ConstantPoolItem::Fieldref {
            class_index,
            name_and_type_index,
        } => validate_member_reference(constant_pool, *class_index, *name_and_type_index, MemberKind::Field),
        ConstantPoolItem::Methodref {
            class_index,
            name_and_type_index,
        }
        | ConstantPoolItem::InterfaceMethodref {
            class_index,
            name_and_type_index,
        } => validate_member_reference(constant_pool, *class_index, *name_and_type_index, MemberKind::Method),
        ConstantPoolItem::NameAndType {
            name_index,
            descriptor_index,
        } => {
            let name = constant_pool.get(name_index).and_then(ConstantPoolItem::utf8);
            let descriptor = constant_pool.get(descriptor_index).and_then(ConstantPoolItem::utf8);
            name.is_some_and(|name| !name.is_empty())
                && descriptor.is_some_and(|descriptor| is_field_descriptor(&descriptor) || is_method_descriptor(&descriptor))
        }
        _ => true,
    })
}

fn validate_member_reference(constant_pool: &BTreeMap<u16, ConstantPoolItem>, class_index: u16, name_and_type_index: u16, kind: MemberKind) -> bool {
    let class_name = constant_pool
        .get(&class_index)
        .and_then(ConstantPoolItem::class_name_index)
        .and_then(|index| constant_pool.get(&index))
        .and_then(ConstantPoolItem::utf8);
    let name_and_type = constant_pool.get(&name_and_type_index).and_then(ConstantPoolItem::name_and_type);
    let Some((name_index, descriptor_index)) = name_and_type else {
        return false;
    };
    let name = constant_pool.get(&name_index).and_then(ConstantPoolItem::utf8);
    let descriptor = constant_pool.get(&descriptor_index).and_then(ConstantPoolItem::utf8);

    class_name.is_some_and(|name| is_class_constant_name(&name))
        && name.is_some_and(|name| !name.is_empty())
        && descriptor.is_some_and(|descriptor| match kind {
            MemberKind::Field => is_field_descriptor(&descriptor),
            MemberKind::Method => is_method_descriptor(&descriptor),
        })
}

fn is_internal_class_name(name: &str) -> bool {
    !name.is_empty() && !name.starts_with('[') && !name.contains(['.', ';', '['])
}

fn is_class_constant_name(name: &str) -> bool {
    is_internal_class_name(name) || array_dimensions(name).is_some()
}

fn is_field_descriptor(descriptor: &str) -> bool {
    let mut cursor = 0;
    parse_field_type(descriptor.as_bytes(), &mut cursor) && cursor == descriptor.len()
}

fn is_method_descriptor(descriptor: &str) -> bool {
    let bytes = descriptor.as_bytes();
    if bytes.first() != Some(&b'(') {
        return false;
    }

    let mut cursor = 1;
    while bytes.get(cursor).is_some_and(|byte| *byte != b')') {
        if !parse_field_type(bytes, &mut cursor) {
            return false;
        }
    }
    if bytes.get(cursor) != Some(&b')') {
        return false;
    }
    cursor += 1;

    if bytes.get(cursor) == Some(&b'V') {
        cursor += 1;
    } else if !parse_field_type(bytes, &mut cursor) {
        return false;
    }

    cursor == bytes.len()
}

fn array_dimensions(descriptor: &str) -> Option<usize> {
    let bytes = descriptor.as_bytes();
    let dimensions = bytes.iter().take_while(|byte| **byte == b'[').count();
    if dimensions == 0 || dimensions > u8::MAX as usize {
        return None;
    }

    let mut cursor = 0;
    if parse_field_type(bytes, &mut cursor) && cursor == bytes.len() {
        Some(dimensions)
    } else {
        None
    }
}

fn parse_field_type(bytes: &[u8], cursor: &mut usize) -> bool {
    let mut dimensions = 0;
    while bytes.get(*cursor) == Some(&b'[') {
        dimensions += 1;
        if dimensions > u8::MAX as usize {
            return false;
        }
        *cursor += 1;
    }

    match bytes.get(*cursor) {
        Some(b'B' | b'C' | b'D' | b'F' | b'I' | b'J' | b'S' | b'Z') => {
            *cursor += 1;
            true
        }
        Some(b'L') => {
            let name_start = *cursor + 1;
            let Some(relative_end) = bytes[name_start..].iter().position(|byte| *byte == b';') else {
                return false;
            };
            let name_end = name_start + relative_end;
            if name_end == name_start || bytes[name_start..name_end].iter().any(|byte| matches!(byte, b'.' | b'[' | b';')) {
                return false;
            }
            *cursor = name_end + 1;
            true
        }
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::{array_dimensions, is_field_descriptor, is_method_descriptor};

    #[test]
    fn validates_field_and_method_descriptors() {
        assert!(is_field_descriptor("Ljava/lang/String;"));
        assert!(is_field_descriptor("[[I"));
        assert!(!is_field_descriptor("V"));
        assert!(!is_field_descriptor("[V"));
        assert!(!is_field_descriptor("Igarbage"));

        assert!(is_method_descriptor("([Ljava/lang/String;I)V"));
        assert!(!is_method_descriptor("(V)V"));
        assert!(!is_method_descriptor("(I"));
        assert!(!is_method_descriptor("()"));
    }

    #[test]
    fn counts_valid_array_dimensions() {
        assert_eq!(array_dimensions("[[Ljava/lang/String;"), Some(2));
        assert_eq!(array_dimensions("java/lang/String"), None);
        assert_eq!(array_dimensions("[V"), None);
    }
}
