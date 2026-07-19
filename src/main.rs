use std::{
    env,
    ffi::OsString,
    io::{self, stderr},
    path::PathBuf,
};

use anyhow::bail;

use rust_java::{StartType, run};

struct Opts {
    jar: Option<PathBuf>,
    main_class: Option<PathBuf>,
    args: Vec<String>,
    class_path: Vec<PathBuf>,
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

    let start_type = if let Some(main_class) = &opts.main_class {
        StartType::Class(main_class)
    } else {
        StartType::Jar(opts.jar.as_ref().unwrap())
    };

    let class_path = if opts.jar.is_some() {
        Vec::new()
    } else {
        opts.class_path.iter().map(PathBuf::as_path).collect()
    };

    run(io::stdout(), start_type, &opts.args, &class_path).await?;

    Ok(())
}

fn parse_args() -> anyhow::Result<Opts> {
    parse_args_from(env::args().skip(1), env::var_os("CLASSPATH"))
}

fn parse_args_from<I>(args: I, environment_class_path: Option<OsString>) -> anyhow::Result<Opts>
where
    I: IntoIterator<Item = String>,
{
    let mut args = args.into_iter();
    let mut class_path = environment_class_path
        .map(|value| env::split_paths(&value).collect())
        .unwrap_or_else(|| vec![PathBuf::from(".")]);

    while let Some(argument) = args.next() {
        if argument == "-cp" || argument == "-classpath" {
            let Some(value) = args.next() else {
                bail!("Missing class path after {argument}");
            };
            class_path = env::split_paths(&value).collect();
        } else if argument == "-jar" {
            let Some(jar) = args.next() else {
                bail!("Missing jar file after -jar");
            };
            return Ok(Opts {
                jar: Some(jar.into()),
                main_class: None,
                args: args.collect(),
                class_path,
            });
        } else {
            return Ok(Opts {
                jar: None,
                main_class: Some(argument.into()),
                args: args.collect(),
                class_path,
            });
        }
    }

    bail!("No class or -jar specified")
}

#[cfg(test)]
mod tests {
    use std::{env, ffi::OsString, path::PathBuf};

    use super::parse_args_from;

    #[test]
    fn classpath_options_override_environment_and_preserve_application_args() {
        let first_class_path = env::join_paths(["first", "second"]).unwrap().into_string().unwrap();
        let last_class_path = env::join_paths(["third", "fourth"]).unwrap().into_string().unwrap();
        let opts = parse_args_from(
            vec![
                "-cp".into(),
                first_class_path,
                "-classpath".into(),
                last_class_path,
                "Main".into(),
                "-cp".into(),
                "application-value".into(),
            ],
            Some(OsString::from("environment")),
        )
        .unwrap();

        assert_eq!(opts.class_path, vec![PathBuf::from("third"), PathBuf::from("fourth")]);
        assert_eq!(opts.main_class, Some(PathBuf::from("Main")));
        assert_eq!(opts.args, vec!["-cp", "application-value"]);
    }

    #[test]
    fn classpath_uses_environment_then_current_directory() {
        let environment_class_path = env::join_paths(["environment", "lib"]).unwrap();
        let opts = parse_args_from(["Main"].into_iter().map(String::from), Some(environment_class_path)).unwrap();
        assert_eq!(opts.class_path, vec![PathBuf::from("environment"), PathBuf::from("lib")]);

        let opts = parse_args_from(["Main"].into_iter().map(String::from), None).unwrap();
        assert_eq!(opts.class_path, vec![PathBuf::from(".")]);
    }

    #[test]
    fn classpath_preserves_explicit_empty_entries() {
        let class_path = env::join_paths(["", "classes", "", ""]).unwrap().into_string().unwrap();
        let opts = parse_args_from(vec!["-cp".into(), class_path, "Main".into()], None).unwrap();
        assert_eq!(
            opts.class_path,
            vec![PathBuf::from(""), PathBuf::from("classes"), PathBuf::from(""), PathBuf::from("")]
        );
    }

    #[test]
    fn jar_target_consumes_launcher_options_before_application_args() {
        let opts = parse_args_from(
            ["-cp", "ignored", "-jar", "app.jar", "-classpath", "application-value"]
                .into_iter()
                .map(String::from),
            None,
        )
        .unwrap();

        assert_eq!(opts.class_path, vec![PathBuf::from("ignored")]);
        assert_eq!(opts.jar, Some(PathBuf::from("app.jar")));
        assert_eq!(opts.args, vec!["-classpath", "application-value"]);
    }

    #[test]
    fn classpath_option_requires_a_value_and_launch_target() {
        let error = parse_args_from(["-cp"].into_iter().map(String::from), None).err().unwrap();
        assert_eq!(error.to_string(), "Missing class path after -cp");

        let error = parse_args_from(["-classpath", "classes"].into_iter().map(String::from), None)
            .err()
            .unwrap();
        assert_eq!(error.to_string(), "No class or -jar specified");
    }
}
