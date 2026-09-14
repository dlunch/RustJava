# RustJava

Embeddable jvm and java runtime implementation, targetting running on webassembly

## Interpreter Benchmark

```sh
cargo test --release --test test_interpreter_performance -- --ignored --nocapture
```

`test-data/src/InterpreterLoops.java` performs the same calculation in three ways:
inline arithmetic, calls to Java methods, and calls to the Rust-backed `Math` methods.
The benchmark warms each path, then measures seven runs of 100,000 iterations,
rotating their order and checking every result against a Rust calculation.
Class loading and initialization are excluded from the timings.
Set `BENCH_ITERATIONS` and `BENCH_SAMPLES` to change the workload.
Timings cover these loops on the current host, not overall application performance.

To rebuild the fixture after changing its source:

```sh
javac --release 8 -d test-data test-data/src/InterpreterLoops.java
```
