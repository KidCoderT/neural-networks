use std::fmt;

use crate::matrix::Matrix;

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "rows: {}, columns: {}", self.n_rows, self.n_cols)?;
        for row in 0..self.n_rows {
            write!(f, "[ ")?;
            for col in 0..self.n_cols {
                let val = self.data[row * self.n_cols + col];
                write!(f, "{:5.2} ", val)?;
            }
            writeln!(f, " ]")?;
        }
        Ok(())
    }
}
