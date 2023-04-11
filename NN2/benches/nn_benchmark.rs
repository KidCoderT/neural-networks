use criterion::{criterion_group, criterion_main, Criterion};
use nn2::nn::benchmark::{emtpy_new_nn, test_old, train_new_nn};

pub fn nn_benchmarks(c: &mut Criterion) {
    let mut untrained_nn = c.benchmark_group("untrained");

    untrained_nn.bench_function("OLD", |b| b.iter(|| test_old([[0f32; 2]; 2], [0f32; 2])));
    untrained_nn.bench_function("NEW - 1", |b| {
        b.iter(|| train_new_nn([[0f32; 2]; 2], [0f32; 2]))
    });
    untrained_nn.bench_function("NEW - 2", |b| b.iter(|| emtpy_new_nn()));

    untrained_nn.finish();

    let mut trained_nn = c.benchmark_group("trained");

    trained_nn.bench_function("OLD", |b| {
        b.iter(|| {
            test_old(
                [[0.31386864, -0.6496351], [0.5474453, -0.021897793]],
                [-0.44525546, 0.0],
            )
        })
    });

    trained_nn.bench_function("NEW", |b| {
        b.iter(|| {
            train_new_nn(
                [[0.31386864, -0.6496351], [0.5474453, -0.021897793]],
                [-0.44525546, 0.0],
            )
        })
    });

    trained_nn.finish();
}

criterion_group!(benches, nn_benchmarks);
criterion_main!(benches);
