use bench_helper::bench_cmd;
use bench_helper::invocation_program;
use bench_helper::tif;
use bench_helper::CommandUnderTest;
use criterion::criterion_group;
use criterion::criterion_main;
use criterion::BenchmarkId;
use criterion::Criterion;
use paste::paste;

use benches::generate_bench;

generate_bench!(invocation,  "lox-rs", "interpreter_main", invocation_program!, [1000]);

criterion_group! {
    name = invocation_benchs;
    config = Criterion::default().sample_size(20);
    targets = invocation_bench_fn,
}

criterion_main!(invocation_benchs);