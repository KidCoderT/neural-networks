use criterion::{criterion_group, criterion_main, Criterion};
use mylinalg::Matrix;

fn column_benchmark(c: &mut Criterion) {
    let matrix = Matrix::new_rnd(1000, 1000, 0.0, 1000.0);
    c.bench_function("column", |b| b.iter(|| matrix.column(0)));
}

fn row_benchmark(c: &mut Criterion) {
    let matrix = Matrix::new_rnd(1000, 1000, 0.0, 1000.0);
    c.bench_function("row", |b| b.iter(|| matrix.row(0)));
}

fn set_all_benchmark(c: &mut Criterion) {
    let mut matrix = Matrix::new_rnd(1000, 1000, 0.0, 1000.0);
    c.bench_function("set_all", |b| b.iter(|| matrix.set_all(0.0)));
}

fn set_diagonals_benchmark(c: &mut Criterion) {
    let mut matrix = Matrix::new_rnd(1000, 1000, 0.0, 1000.0);
    c.bench_function("set_diagonals", |b| b.iter(|| matrix.set_diagonal(0.0)));
}

fn mul_row_benchmark(c: &mut Criterion) {
    let mut matrix = Matrix::new_rnd(1000, 1000, 0.0, 1000.0);
    c.bench_function("multiply row", |b| b.iter(|| matrix.scale_row(5, 40.0)));
}

fn mul_col_benchmark(c: &mut Criterion) {
    let mut matrix = Matrix::new_rnd(1000, 1000, 0.0, 1000.0);
    c.bench_function("multiply col", |b| b.iter(|| matrix.scale_col(5, 40.0)));
}

// TODO: BENCHMARK OPS

criterion_group!(
    benches,
    column_benchmark,
    row_benchmark,
    set_all_benchmark,
    set_diagonals_benchmark,
    mul_row_benchmark,
    mul_col_benchmark
);
criterion_main!(benches);
