use matrix_ops::*;
use matrix::Matrix;

fn main() {
    let matrix = MatrixOps(Matrix(vec![vec![8, 1], vec![9, 1]]));
    let matrix_2 = MatrixOps(Matrix(vec![vec![1, 1], vec![1, 1]]));
    println!("{:?}", matrix + matrix_2);

    let matrix = MatrixOps(Matrix(vec![vec![1, 3], vec![2, 5]]));
    let matrix_2 = MatrixOps(Matrix(vec![vec![3, 1], vec![1, 1]]));
    println!("{:?}", matrix - matrix_2);

    let matrix = MatrixOps(Matrix(vec![vec![1, 1], vec![1, 1]]));
    let matrix_2 = MatrixOps(Matrix(vec![vec![1, 1, 3], vec![1, 1]]));
    println!("{:?}", matrix - matrix_2);

    let matrix = MatrixOps(Matrix(vec![vec![1, 3], vec![9, 1]]));
    let matrix_2 = MatrixOps(Matrix(vec![vec![1, 1, 3], vec![1, 1]]));
    println!("{:?}", matrix + matrix_2);
}
