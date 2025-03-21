use jni::{InitArgsBuilder, JavaVM, objects::{JObject, JValue}};
use std::{path::PathBuf, sync::Arc, time::Instant};
use crate::config::*;
use crate::packages::*;

pub fn native_runner(files: Vec<PathBuf>, conf: &CONFIG, t: PathType) -> Result<(), Box<dyn std::error::Error>>
{
    let prefix = match t
    {
        PathType::CLASS => &conf.bin,
        PathType::TESTS => &conf.test,
        PathType::SRC => &conf.src,
    };
    let classpath_opt = format!("-Djava.class.path={}", conf.classpath);
    let mut args = InitArgsBuilder::new()
        .version(conf.jvm_version)
        .option(&classpath_opt);
    for opt in &conf.jvm_options
    {
        args = args.option(opt);
    }
    let jvm_args = args.build()?;
    let jvm = Arc::new(JavaVM::new(jvm_args)?);
    let mut classes: Vec<String> = Vec::new();
    for file in files
    {
        classes.push(
            file.strip_prefix(prefix)?
                .with_extension("")
                .to_string_lossy()
                .replace(".java", "")
        );
    }
    for chunk in classes.chunks(conf.threads)
    {
        if chunk.is_empty()
        {
            continue;
        }
        let handles: Vec<_> = chunk.iter().map(|class|
            {
            let class = class.clone();
            let jvm = Arc::clone(&jvm);
            let run_args = conf.run_args.clone();
            std::thread::spawn(move ||
                {
                println!("\x1b[34m[RUNNING]\x1b[0m {}", class);
                let now = Instant::now();
                let mut env = jvm.attach_current_thread().unwrap();
                let cls = env.find_class(&class).unwrap();
                let string_cls = env.find_class("java/lang/String").unwrap();
                let args_array = env.new_object_array(run_args.len() as i32, string_cls, JObject::null()).unwrap();
                for (i, arg) in run_args.iter().enumerate()
                {
                    let jstr = env.new_string(arg).unwrap();
                    env.set_object_array_element(&args_array, i as i32, jstr).unwrap();
                }
                let arg_obj = args_array.into();
                match env.call_static_method(
                    cls,
                    "main",
                    "([Ljava/lang/String;)V",
                    &[JValue::Object(&arg_obj)]
                )
                {
                    Ok(_) =>
                    {
                        let elapsed = now.elapsed();
                        println!("\x1b[32m[SUCCESSFUL]\x1b[0m {} ({}ms)", class, elapsed.as_millis());
                    }
                    Err(e) =>
                    {
                        let elapsed = now.elapsed();
                        println!("\x1b[31m[FAILED]\x1b[0m {} ({}ms)", class, elapsed.as_millis());
                        eprintln!("  ↳ JNI error: {e}");
                        if env.exception_check().unwrap_or(false)
                        {
                            let exc = env.exception_occurred().unwrap();
                            env.exception_clear().unwrap();
                            let _ = env.call_method(
                                exc,
                                "printStackTrace",
                                "()V",
                                &[]
                            );
                        }
                    }
                }
            })
        }).collect();
        for h in handles
        {
            h.join().expect("Thread failed");
        }
    }

    Ok(())
}
