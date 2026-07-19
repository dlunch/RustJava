use java_runtime::classes::java::lang::String;
use jvm::{ClassInstanceRef, JavaChar, Result, runtime::JavaLangString};

use test_utils::test_jvm;

#[tokio::test]
async fn test_platform_separators() -> Result<()> {
    let jvm = test_jvm().await?;

    let separator_char: JavaChar = jvm.get_static_field("java/io/File", "separatorChar", "C").await?;
    let separator: ClassInstanceRef<String> = jvm.get_static_field("java/io/File", "separator", "Ljava/lang/String;").await?;
    let path_separator_char: JavaChar = jvm.get_static_field("java/io/File", "pathSeparatorChar", "C").await?;
    let path_separator: ClassInstanceRef<String> = jvm.get_static_field("java/io/File", "pathSeparator", "Ljava/lang/String;").await?;

    if cfg!(windows) {
        assert_eq!(separator_char, '\\' as JavaChar);
        assert_eq!(JavaLangString::to_rust_string(&jvm, &separator).await?, "\\");
        assert_eq!(path_separator_char, ';' as JavaChar);
        assert_eq!(JavaLangString::to_rust_string(&jvm, &path_separator).await?, ";");
    } else {
        assert_eq!(separator_char, '/' as JavaChar);
        assert_eq!(JavaLangString::to_rust_string(&jvm, &separator).await?, "/");
        assert_eq!(path_separator_char, ':' as JavaChar);
        assert_eq!(JavaLangString::to_rust_string(&jvm, &path_separator).await?, ":");
    }

    Ok(())
}
