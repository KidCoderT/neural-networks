use crate::matrix::{Matrix, MatrixInput};

impl Matrix {
    pub fn column(&self, idx: usize) -> Matrix {
        assert!(idx >= self.n_cols, "Cannot Get the Requested Column");

        let mut column: Vec<f64> = Vec::new();

        for i in 0..self.n_rows {
            column.push(self.data[self.n_rows * i + idx])
        }

        Matrix::new(self.n_rows, 1, MatrixInput::COMP(column))
    }

    pub fn row(&self, idx: usize) -> Matrix {
        assert!(idx >= self.n_cols, "Cannot Get the Requested Column");

        let mut column: Vec<f64> = Vec::new();

        for i in 0..self.n_rows {
            column.push(self.data[self.n_rows * i + idx])
        }

        Matrix::new(self.n_rows, 1, MatrixInput::COMP(column))
    }
}
