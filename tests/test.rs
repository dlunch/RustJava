use jvm::{Class, ClassLoader, Jvm};

struct FileClassLoader {
    class_name: String,
    data: Vec<u8>,
}

impl FileClassLoader {
    pub fn new(class_name: &str, data: &[u8]) -> Self {
        Self {
            class_name: class_name.to_string(),
            data: data.to_vec(),
        }
    }
}

impl ClassLoader for FileClassLoader {
    fn load(&mut self, class_name: &str) -> anyhow::Result<Option<Class>> {
        if class_name == self.class_name {
            let class = Class::from_classfile(&self.data)?;

            Ok(Some(class))
        } else {
            Ok(None)
        }
    }
}

#[test]
fn test_hello() -> anyhow::Result<()> {
    let hello = include_bytes!("./test_data/Hello.class");

    let mut jvm = Jvm::new();
    jvm.add_class_loader(FileClassLoader::new("Hello", hello));

    let class = jvm.resolve_class("Hello")?.unwrap();

    assert_eq!(class.name, "Hello");
    assert_eq!(class.methods.len(), 2);
    assert!(class.method("<init>", "()V").is_some());
    assert!(class.method("main", "([Ljava/lang/String;)V").is_some());

    Ok(())
}
