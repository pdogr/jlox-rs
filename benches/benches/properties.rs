use bench_helper::bench_cmd;
use bench_helper::properties_program;
use bench_helper::tif;
use bench_helper::CommandUnderTest;
use criterion::criterion_group;
use criterion::criterion_main;
use criterion::BenchmarkId;
use criterion::Criterion;
use paste::paste;

use benches::generate_bench;

generate_bench!(properties,  "lox-rs", "interpreter_main", properties_program!, [10,100]);

criterion_group! {
    name = properties_benchs;
    config = Criterion::default().sample_size(20);
    targets = properties_bench_fn,
}

criterion_main!(properties_benchs);
