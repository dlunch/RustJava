use std::{
    env, fs,
    io::{self},
    path::Path,
};

use futures_executor::block_on;

use rust_java::{create_jvm, load_class_file, run_java_main};

// TODO move logics into lib
pub fn main() -> anyhow::Result<()> {
    let args = env::args().collect::<Vec<_>>();
    let filename = &args[1];
    let main_class_name = &args[2];
    let args = &args[3..];

    block_on(async {
        let jvm = create_jvm(io::stdout()).await?;

        let data = fs::read(filename)?;

        // TODO remove filename parsing and parse class name in ClassPathClassLoader
        let filename = Path::new(filename).file_name().unwrap().to_string_lossy();
        let class_name = filename.strip_suffix(".class").unwrap();

        load_class_file(&jvm, class_name, &data).await?;

        run_java_main(&jvm, main_class_name, args).await?;

        Ok(())
    })
}
