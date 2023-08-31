use mylib::*;

fn main() {
    let vec_matrix = Matrix::new_square_matrix(8);
    let arr_matrix = Matrix::new_rnd(1, 2, 0.0, 10.0);
    println!("{:?}, {:?}", vec_matrix.data, arr_matrix.data);
    println!("{:?}, {:?}", vec_matrix.is_square(), arr_matrix.is_square());
}
