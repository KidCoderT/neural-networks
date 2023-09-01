use rand::Rng;

use crate::matrix::{Matrix, MatrixInput};

impl Matrix {
    pub fn new_rnd(n_rows: usize, n_cols: usize, min: f64, max: f64) -> Self {
        let mut thread = rand::thread_rng();

        let vector: Vec<f64> = (0..(n_rows * n_cols))
            .map(|_| thread.gen_range(min..=max))
            .collect();

        Matrix::new(n_rows, n_cols, MatrixInput::COMP(vector))
    }

    pub fn new_rnd_square_matrix(size: usize, min: f64, max: f64) -> Self {
        let mut thread = rand::thread_rng();

        let vector: Vec<f64> = (0..(size * size))
            .map(|_| thread.gen_range(min..=max))
            .collect();

        Matrix::new(size, size, MatrixInput::COMP(vector))
    }
}
