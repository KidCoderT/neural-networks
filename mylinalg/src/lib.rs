use rand::Rng;

pub struct Matrix {
    pub n_rows: usize,
    pub n_cols: usize,
    pub data: Vec<f64>,
}

pub enum Array {
    SIMPLE(Vec<Vec<f64>>),
    COMP(Vec<f64>),
}

impl Matrix {
    pub fn is_square(&self) -> bool {
        self.n_rows == self.n_cols
    }

    pub fn new(n_rows: usize, n_cols: usize, data: Array) -> Self {
        assert_ne!(n_rows, 0, "matrix row size cannot be 0");
        assert_ne!(n_cols, 0, "matrix column size cannot be 0");

        let mut proper_data: Vec<f64> = Vec::new();

        match data {
            Array::SIMPLE(vec_format) => {
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
            Array::COMP(comp_format) => {
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

    pub fn new_rnd(n_rows: usize, n_cols: usize, min: f64, max: f64) -> Self {
        let mut thread = rand::thread_rng();

        let vector: Vec<f64> = (0..(n_rows * n_cols))
            .map(|_| thread.gen_range(min..=max))
            .collect();

        Matrix::new(n_rows, n_cols, Array::COMP(vector))
    }

    pub fn new_square_matrix(size: usize) -> Self {
        Matrix::new(size, size, Array::COMP(vec![0.0; size * size]))
    }

    pub fn new_rnd_square_matrix(size: usize, min: f64, max: f64) -> Self {
        let mut thread = rand::thread_rng();

        let vector: Vec<f64> = (0..(size * size))
            .map(|_| thread.gen_range(min..=max))
            .collect();

        Matrix::new(size, size, Array::COMP(vector))
    }
}
