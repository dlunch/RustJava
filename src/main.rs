use std::{
    env,
    io::{self, stderr},
    path::{Path, PathBuf},
};

use anyhow::bail;

use rust_java::{StartType, run};

struct Opts {
    jar: Option<PathBuf>,
    main_class: Option<PathBuf>,
    args: Vec<String>,
}

pub fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_writer(stderr)
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    #[cfg(not(target_arch = "wasm32"))]
    let runtime = tokio::runtime::Runtime::new().unwrap();
    #[cfg(target_arch = "wasm32")]
    let runtime = tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap();

    runtime.block_on(async_main())
}

pub async fn async_main() -> anyhow::Result<()> {
    let opts = parse_args()?;

    let start_type = if opts.main_class.is_some() {
        StartType::Class(opts.main_class.as_ref().unwrap())
    } else {
        StartType::Jar(opts.jar.as_ref().unwrap())
    };

    run(io::stdout(), start_type, &opts.args, &[Path::new(".")]).await?;

    Ok(())
}

fn parse_args() -> anyhow::Result<Opts> {
    let mut args = env::args().skip(1); // skip program name
    let mut jar = None;
    let mut main_class = None;
    let mut rest_args = Vec::new();

    if let Some(first) = args.next() {
        if first == "-jar" {
            // java -jar foo.jar [args...]
            if let Some(jar_path) = args.next() {
                jar = Some(jar_path.into());
                rest_args.extend(args);
            } else {
                bail!("Missing jar file after -jar");
            }
        } else {
            // java MainClass [args...]
            main_class = Some(first.into());
            rest_args.extend(args);
        }
    } else {
        bail!("No class or -jar specified");
    }

    Ok(Opts {
        jar,
        main_class,
        args: rest_args,
    })
}
