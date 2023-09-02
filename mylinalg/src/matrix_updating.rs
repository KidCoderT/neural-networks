use crate::matrix::Matrix;
use rayon::prelude::*;

// TODO: ADD RAYON

impl Matrix {
    pub fn set(&mut self, pos: (usize, usize), value: f64) {
        assert!(pos.1 < self.n_cols, "Incorrect Column Value");
        assert!(pos.0 < self.n_rows, "Incorrect Row Value");

        self.data[self.n_rows * pos.0 + pos.1] = value
    }

    pub fn set_all(&mut self, value: f64) {
        self.data.par_iter_mut().for_each(|element| {
            *element = value;
        });
    }

    pub fn set_diagonal(&mut self, value: f64) {
        assert!(self.is_square(), "only possible for square matrices");
        let size = self.n_rows;

        self.data.par_iter_mut().enumerate().for_each(|(idx, element)| {
            if idx % (size + 1) == 0 {
                *element = value;
            }
        });
    }
}
