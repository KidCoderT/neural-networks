use crate::matrix::Matrix;

// TODO: ADD RAYON

impl Matrix {
    pub fn set(&mut self, pos: (usize, usize), value: f64) {
        assert!(pos.1 < self.n_cols, "Incorrect Column Value");
        assert!(pos.0 < self.n_rows, "Incorrect Row Value");

        self.data[self.n_rows * pos.0 + pos.1] = value
    }

    pub fn set_all(&mut self, value: f64) {
        for element in self.data.iter_mut() {
            *element = value;
        }
    }

    pub fn set_diagonal(&mut self, value: f64) {
        assert!(self.is_square(), "only possible for square matrices");
        let size = self.n_rows;

        for (idx, element) in self.data.iter_mut().enumerate() {
            if idx % (size + 1) == 0 {
                *element = value;
            }
        }
    }
}
