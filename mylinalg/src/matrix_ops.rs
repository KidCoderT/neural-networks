use std::ops::{Mul, MulAssign};
use crate::matrix::Matrix;

// TODO: ADD RAYON

impl Matrix {
    pub fn scale_row(&mut self, idx: usize, scaler: f64) {
        assert!(idx < self.n_rows, "Row not Found");
        ((idx * self.n_rows)..((idx + 1) * self.n_rows)).for_each(|idx| {
            self.data[idx] *= scaler;
        });
    }

    pub fn scale_col(&mut self, idx: usize, scaler: f64) {
        assert!(idx < self.n_cols, "Column not Found");
        (0..self.n_rows).for_each(|row| {
            self.data[self.n_rows * row + idx] *= scaler;
        });
    }

    // TODO: ADD Methods
    // pub fn add_scaled_row_to_row(&mut self, )
    // pub fn add_scaled_row_to_row_n(self, to: usize, from: usize, scale: f64) -> Matrix {
    //     self.to
    // }
}

impl Mul<f64> for Matrix {
    type Output = Matrix;

    fn mul(self, scalar: f64) -> Matrix {
        let mut result = self.clone();
        for element in result.data.iter_mut() {
            *element *= scalar; 
        }
        result
    }
}

impl MulAssign<f64> for Matrix {
    fn mul_assign(&mut self, scalar: f64) {
        for element in self.data.iter_mut() {
            *element *= scalar;
        }
    }
}