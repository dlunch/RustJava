use alloc::{boxed::Box, format, string::String as RustString, sync::Arc, vec, vec::Vec};

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use jvm::{runtime::JavaLangString, ClassInstance, ClassInstanceRef, Jvm, Result};

use crate::{
    classes::java::{
        io::{PrintStream, PrintWriter},
        lang::String,
    },
    RuntimeClassProto, RuntimeContext,
};

// class java.lang.Throwable
pub struct Throwable;

impl Throwable {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/lang/Throwable",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new("<init>", "(Ljava/lang/String;)V", Self::init_with_message, Default::default()),
                JavaMethodProto::new("toString", "()Ljava/lang/String;", Self::to_string, Default::default()),
                JavaMethodProto::new(
                    "fillInStackTrace",
                    "()Ljava/lang/Throwable;",
                    Self::fill_in_stack_trace,
                    Default::default(),
                ),
                JavaMethodProto::new("printStackTrace", "()V", Self::print_stack_trace, Default::default()),
                JavaMethodProto::new(
                    "printStackTrace",
                    "(Ljava/io/PrintStream;)V",
                    Self::print_stack_trace_to_print_stream,
                    Default::default(),
                ),
                JavaMethodProto::new(
                    "printStackTrace",
                    "(Ljava/io/PrintWriter;)V",
                    Self::print_stack_trace_to_print_writer,
                    Default::default(),
                ),
            ],
            fields: vec![
                JavaFieldProto::new("detailMessage", "Ljava/lang/String;", Default::default()),
                JavaFieldProto::new("stackTrace", "[B", Default::default()),
            ],
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.lang.Throwable::<init>({:?})", &this);

        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;

        let _: ClassInstanceRef<Self> = jvm.invoke_virtual(&this, "fillInStackTrace", "()Ljava/lang/Throwable;", ()).await?;

        Ok(())
    }

    async fn init_with_message(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, message: ClassInstanceRef<String>) -> Result<()> {
        tracing::debug!("java.lang.Throwable::<init>({:?}, {:?})", &this, &message);

        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;

        jvm.put_field(&mut this, "detailMessage", "Ljava/lang/String;", message).await?;

        let _: ClassInstanceRef<Self> = jvm.invoke_virtual(&this, "fillInStackTrace", "()Ljava/lang/Throwable;", ()).await?;

        Ok(())
    }

    async fn fill_in_stack_trace(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Self>> {
        tracing::debug!("java.lang.Throwable::fillInStackTrace({:?})", &this);

        let stack_trace = Arc::new(jvm.stack_trace().await);

        jvm.put_rust_object_field(&mut this, "stackTrace", stack_trace).await?;

        Ok(this)
    }

    async fn print_stack_trace(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.lang.Throwable::printStackTrace({:?})", &this);

        let err: ClassInstanceRef<PrintStream> = jvm.get_static_field("java/lang/System", "err", "Ljava/io/PrintStream;").await?;

        let _: () = jvm.invoke_virtual(&this, "printStackTrace", "(Ljava/io/PrintStream;)V", (err,)).await?;

        Ok(())
    }

    async fn print_stack_trace_to_print_stream(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        stream: ClassInstanceRef<PrintStream>,
    ) -> Result<()> {
        tracing::debug!("java.lang.Throwable::printStackTrace({:?}, {:?})", &this, &stream);

        Self::do_print_stack_trace(jvm, this, stream.into()).await?;

        Ok(())
    }

    async fn print_stack_trace_to_print_writer(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        writer: ClassInstanceRef<PrintWriter>,
    ) -> Result<()> {
        tracing::debug!("java.lang.Throwable::printStackTrace({:?}, {:?})", &this, &writer);

        Self::do_print_stack_trace(jvm, this, writer.into()).await?;

        Ok(())
    }

    async fn to_string(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        tracing::debug!("java.lang.Throwable::toString({:?})", &this);

        let class = jvm.invoke_virtual(&this, "getClass", "()Ljava/lang/Class;", ()).await?;
        let class_name = jvm.invoke_virtual(&class, "getName", "()Ljava/lang/String;", ()).await?;

        let message: ClassInstanceRef<String> = jvm.get_field(&this, "detailMessage", "Ljava/lang/String;").await?;

        let class_name = JavaLangString::to_rust_string(jvm, &class_name).await?;
        let message = if message.is_null() {
            class_name
        } else {
            let message = JavaLangString::to_rust_string(jvm, &message).await?;
            if message.is_empty() {
                class_name
            } else {
                format!("{}: {}", class_name, message)
            }
        };

        let message = JavaLangString::from_rust_string(jvm, &message).await?;

        Ok(message.into())
    }

    async fn do_print_stack_trace(jvm: &Jvm, this: ClassInstanceRef<Self>, stream_or_writer: Box<dyn ClassInstance>) -> Result<()> {
        let stack_trace: Arc<Vec<RustString>> = jvm.get_rust_object_field(&this, "stackTrace").await?;

        // TODO we can call println(Ljava/lang/Object;)V
        let string: ClassInstanceRef<String> = jvm.invoke_virtual(&this, "toString", "()Ljava/lang/String;", ()).await?;
        let _: () = jvm
            .invoke_virtual(&stream_or_writer, "println", "(Ljava/lang/String;)V", (string,))
            .await?;

        for line in stack_trace.iter() {
            let line = format!("\tat {}", line);
            let line = JavaLangString::from_rust_string(jvm, &line).await?;
            let _: () = jvm.invoke_virtual(&stream_or_writer, "println", "(Ljava/lang/String;)V", (line,)).await?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use alloc::boxed::Box;
    use jvm::{runtime::JavaLangString, ClassInstance, ClassInstanceRef, JavaError, Result};

    use crate::{classes::java::lang::String, test::test_jvm};

    #[tokio::test]
    async fn test_to_string() -> Result<()> {
        let jvm = test_jvm().await?;

        let message = JavaLangString::from_rust_string(&jvm, "test message").await?;

        let throwable = jvm.new_class("java/lang/Throwable", "(Ljava/lang/String;)V", (message,)).await?;
        let to_string = jvm.invoke_virtual(&throwable, "toString", "()Ljava/lang/String;", ()).await?;

        let result = JavaLangString::to_rust_string(&jvm, &to_string).await?;

        assert_eq!(result, "java/lang/Throwable: test message");

        Ok(())
    }

    #[tokio::test]
    async fn test_stacktrace() -> Result<()> {
        let jvm = test_jvm().await?;

        // get exception by creating invalid url
        let url_string = JavaLangString::from_rust_string(&jvm, "invalid://invalid").await?;
        let url: Result<Box<dyn ClassInstance>> = jvm.new_class("java/net/URL", "(Ljava/lang/String;)V", (url_string,)).await;

        let exception = if let JavaError::JavaException(exception) = url.err().unwrap() {
            exception
        } else {
            panic!("expected JavaException");
        };

        let string_writer = jvm.new_class("java/io/StringWriter", "()V", ()).await?;
        let print_writer = jvm
            .new_class("java/io/PrintWriter", "(Ljava/io/Writer;)V", (string_writer.clone(),))
            .await?;

        let _: () = jvm
            .invoke_virtual(&exception, "printStackTrace", "(Ljava/io/PrintWriter;)V", (print_writer,))
            .await?;

        let result: ClassInstanceRef<String> = jvm.invoke_virtual(&string_writer, "toString", "()Ljava/lang/String;", ()).await?;
        let result = JavaLangString::to_rust_string(&jvm, &result).await?;

        assert_eq!(
            result,
            "\
                java/net/MalformedURLException: unknown protocol: invalid\n\
                    \tat java/net/URL.<init>(Ljava/net/URL;Ljava/lang/String;Ljava/net/URLStreamHandler;)V\n\
                    \tat java/net/URL.<init>(Ljava/net/URL;Ljava/lang/String;)V\n\
                    \tat java/net/URL.<init>(Ljava/lang/String;)V\n\
            "
        );

        Ok(())
    }
}
