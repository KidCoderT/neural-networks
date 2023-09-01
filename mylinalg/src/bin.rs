use mylinalg::*;

fn main() {
    let mut identity_matrix = Matrix::new_rnd_square_matrix(5, 1.0, 30.0);
    let new_mat = identity_matrix * 5.0;
    println!("{}", new_mat);
}
