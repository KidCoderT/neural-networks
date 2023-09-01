use std::cmp;

use crate::matrix::Matrix;

impl cmp::PartialEq for Matrix {
    fn eq(&self, other: &Self) -> bool {
        if self.n_rows != other.n_rows || self.n_cols != other.n_cols {
            return false;
        }

        for i in 0..self.data.len() {
            if (self.data[i] - other.data[i]).abs() > std::f64::EPSILON {
                return false;
            }
        }
        true
    }
}
