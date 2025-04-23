use lalgebra_scalar::Scalar;

#[derive(Debug, Clone, PartialEq)]
pub struct Matrix<T>(pub Vec<Vec<T>>);

impl<T: Scalar<Item = T>> Matrix<T> {
    pub fn new() -> Matrix<T> {
        // Create a 1x1 matrix with a zero value
        Matrix(vec![vec![T::zero()]])
    }

    pub fn zero(row: usize, col: usize) -> Matrix<T> {
        // Create a matrix of size row x col filled with zeros
        let mut matrix = Vec::with_capacity(row);
        for _ in 0..row {
            let row_vec = vec![T::zero(); col];
            matrix.push(row_vec);
        }
        Matrix(matrix)
    }

    pub fn identity(n: usize) -> Matrix<T> {
        // Create a matrix of size n x n
        // Fill the diagonal with ones and the rest with zeros
        let mut matrix = Vec::with_capacity(n);
        for i in 0..n {
            let mut row_vec = vec![T::zero(); n];
            row_vec[i] = T::one(); // Set diagonal element to one
            matrix.push(row_vec);
        }
        Matrix(matrix)
    }
}