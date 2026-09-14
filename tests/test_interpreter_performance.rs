use std::{env, time::Instant};

use jvm::Result;
use jvm_bytecode::ClassDefinitionImpl;
use test_utils::test_jvm;

// cargo test --release --test test_interpreter_performance -- --ignored --nocapture
#[tokio::test]
#[ignore = "manual interpreter benchmark"]
async fn interpreter_loops() -> Result<()> {
    let count: i32 = env::var("BENCH_ITERATIONS").unwrap_or_else(|_| "100000".into()).parse().unwrap();
    let samples: usize = env::var("BENCH_SAMPLES").unwrap_or_else(|_| "7".into()).parse().unwrap();
    assert!(count > 0 && samples > 0);

    let jvm = test_jvm().await?;
    let class = ClassDefinitionImpl::from_classfile(include_bytes!("../test-data/InterpreterLoops.class")).unwrap();
    jvm.register_class(Box::new(class), None).await?;
    let expected = (0..count).fold(0i32, |sum, i| {
        sum.wrapping_add(i.wrapping_mul(31).wrapping_sub(count).wrapping_abs().clamp(7, 10000))
    });
    let methods = ["pure", "javaCalls", "runtimeCalls"];
    let mut timings = [Vec::new(), Vec::new(), Vec::new()];

    // Warm each path before timing so class loading and initialization are excluded.
    for method in methods {
        let result: i32 = jvm.invoke_static("InterpreterLoops", method, "(I)I", (count,)).await?;
        assert_eq!(result, expected, "{method}");
    }

    for sample in 0..samples {
        for offset in 0..methods.len() {
            let index = (sample + offset) % methods.len();
            let start = Instant::now();
            let result: i32 = jvm.invoke_static("InterpreterLoops", methods[index], "(I)I", (count,)).await?;
            timings[index].push(start.elapsed());
            assert_eq!(result, expected, "{} sample {sample}", methods[index]);
        }
    }

    println!("iterations={count}, samples={samples}, checksum={expected}");
    for (method, mut times) in methods.into_iter().zip(timings) {
        times.sort();
        println!(
            "{method}: median={:.3} ms, min={:.3} ms, max={:.3} ms",
            (times[(samples - 1) / 2] + times[samples / 2]).as_secs_f64() * 500.0,
            times[0].as_secs_f64() * 1000.0,
            times[samples - 1].as_secs_f64() * 1000.0,
        );
    }
    Ok(())
}
