pub struct Matrix {
    pub n_rows: usize,
    pub n_cols: usize,
    pub data: Vec<f64>,
}

pub enum MatrixInput {
    SIMPLE(Vec<Vec<f64>>),
    COMP(Vec<f64>),
}

impl Matrix {
    pub fn is_square(&self) -> bool {
        self.n_rows == self.n_cols
    }

    pub fn new(n_rows: usize, n_cols: usize, data: MatrixInput) -> Self {
        assert_ne!(n_rows, 0, "matrix row size cannot be 0");
        assert_ne!(n_cols, 0, "matrix column size cannot be 0");

        let mut proper_data: Vec<f64> = Vec::new();

        match data {
            MatrixInput::SIMPLE(vec_format) => {
                assert_eq!(
                    vec_format.len(),
                    n_rows,
                    "the data size and the given data size for the row number does not match"
                );

                for row in vec_format.iter() {
                    assert_eq!(
                        row.len(),
                        n_cols,
                        "the data size and the given data size for the column number does not match"
                    );

                    proper_data.append(&mut row.clone())
                }
            }
            MatrixInput::COMP(comp_format) => {
                assert_eq!(comp_format.len(), n_rows * n_cols);
                proper_data = comp_format
            }
        }

        Matrix {
            n_rows,
            n_cols,
            data: proper_data,
        }
    }

    pub fn new_square_matrix(size: usize) -> Self {
        Matrix::new(size, size, MatrixInput::COMP(vec![0.0; size * size]))
    }

    pub fn identity_matrix(size: usize) -> Self {
        let vector: Vec<f64> = (0..(size * size))
            .map(|idx| if idx % (size + 1) == 0 { 1.0 } else { 0.0 })
            .collect();

        Matrix::new(size, size, MatrixInput::COMP(vector))
    }
}
