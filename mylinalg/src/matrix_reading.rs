use crate::matrix::{Matrix, MatrixInput};

// TODO: ADD RAYON

impl Matrix {
    pub fn column(&self, idx: usize) -> Matrix {
        assert!(idx < self.n_cols, "Cannot Get the Requested Column");
        let column: Vec<f64> = (0..self.n_rows)
            .map(|row| self.data[self.n_rows * row + idx])
            .collect();
        Matrix::new(self.n_rows, 1, MatrixInput::COMP(column))
    }

    pub fn row(&self, idx: usize) -> Matrix {
        assert!(idx < self.n_rows, "Cannot Get the Requested Row");
        let row = self.data[(idx * self.n_rows)..((idx + 1) * self.n_rows)].to_vec();
        Matrix::new(1, self.n_cols, MatrixInput::COMP(row))
    }
}
