use std::{
    env, fs,
    io::{self},
    path::Path,
};

use futures_executor::block_on;

use rust_java::{create_jvm, load_class_file, run_java_main};

pub fn main() -> anyhow::Result<()> {
    let args = env::args().collect::<Vec<_>>();
    let filename = &args[1];
    let main_class_name = &args[2];
    let args = &args[3..];

    block_on(async {
        let mut jvm = create_jvm(io::stdout()).await?;

        let data = fs::read(filename)?;

        let filename = Path::new(filename).file_name().unwrap().to_string_lossy();
        let class_name = filename.strip_suffix(".class").unwrap();

        load_class_file(&mut jvm, class_name, &data).await?;

        run_java_main(&mut jvm, main_class_name, args).await?;

        Ok(())
    })
}
