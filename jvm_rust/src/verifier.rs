use classfile::{AttributeInfo, ClassInfo, ConstantPoolReference, Opcode};
use jvm::JavaType;

use crate::ClassDefinitionError;

pub(crate) fn verify(class: &ClassInfo) -> Result<(), ClassDefinitionError> {
    for method in &class.methods {
        for attribute in &method.attributes {
            let AttributeInfo::Code(code) = attribute else {
                continue;
            };
            for opcode in code.code.values() {
                match opcode {
                    Opcode::Multianewarray(ConstantPoolReference::Class(name), dimensions) => {
                        let Some(mut r#type) = JavaType::try_parse(name) else {
                            return Err(ClassDefinitionError::Verification);
                        };
                        let mut available_dimensions = 0;
                        while let JavaType::Array(element) = r#type {
                            available_dimensions += 1;
                            r#type = *element;
                        }
                        if available_dimensions < *dimensions as usize {
                            return Err(ClassDefinitionError::Verification);
                        }
                    }
                    Opcode::Invokedynamic(_) => return Err(ClassDefinitionError::UnsupportedFeature("invokedynamic")),
                    _ => {}
                }
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use classfile::{AttributeInfo, ClassInfo, Opcode};

    use crate::{ClassDefinitionError, verifier::verify};

    #[test]
    fn rejects_multianewarray_dimensions_larger_than_the_array_type() {
        let mut class = ClassInfo::parse(include_bytes!("../../test_data/MultiArray.class")).unwrap();
        let mut changed = false;
        for method in &mut class.methods {
            for attribute in &mut method.attributes {
                let AttributeInfo::Code(code) = attribute else {
                    continue;
                };
                for opcode in code.code.values_mut() {
                    if let Opcode::Multianewarray(_, dimensions) = opcode {
                        *dimensions = u8::MAX;
                        changed = true;
                    }
                }
            }
        }

        assert!(changed);
        assert_eq!(verify(&class), Err(ClassDefinitionError::Verification));
    }
}
