use std::ops::{Add, Sub};
use matrix::Matrix;
use lalgebra_scalar::Scalar;

// Create a wrapper type for Matrix
#[derive(Debug, Clone, PartialEq)]
pub struct MatrixOps<T>(pub Matrix<T>);

// Implement conversion from Matrix to MatrixOps
impl<T> From<Matrix<T>> for MatrixOps<T> {
    fn from(matrix: Matrix<T>) -> Self {
        MatrixOps(matrix)
    }
}

// Implement conversion from MatrixOps to Matrix
impl<T> From<MatrixOps<T>> for Matrix<T> {
    fn from(matrix_ops: MatrixOps<T>) -> Self {
        matrix_ops.0
    }
}


impl<T: Scalar<Item = T> + Clone> Add for MatrixOps<T> {
    type Output = Option<Self>;

    fn add(self, other: Self) -> Self::Output {
        // Check if matrices have the same dimensions
        if self.0.0.len() != other.0.0.len() {
            return None;
        }

        for i in 0..self.0.0.len() {
            if self.0.0[i].len() != other.0.0[i].len() {
                return None;
            }
        }

        // Perform addition
        let mut result = Vec::with_capacity(self.0.0.len());
        for i in 0..self.0.0.len() {
            let mut row = Vec::with_capacity(self.0.0[i].len());
            for j in 0..self.0.0[i].len() {
                row.push(self.0.0[i][j] + other.0.0[i][j]);
            }
            result.push(row);
        }

        Some(MatrixOps(Matrix(result)))
    }
}

impl<T: Scalar<Item = T> + Clone> Sub for MatrixOps<T> {
    type Output = Option<Self>;

    fn sub(self, other: Self) -> Self::Output {
        // Check if matrices have the same dimensions
        if self.0.0.len() != other.0.0.len() {
            return None;
        }

        for i in 0..self.0.0.len() {
            if self.0.0[i].len() != other.0.0[i].len() {
                return None;
            }
        }

        // Perform subtraction
        let mut result = Vec::with_capacity(self.0.0.len());
        for i in 0..self.0.0.len() {
            let mut row = Vec::with_capacity(self.0.0[i].len());
            for j in 0..self.0.0[i].len() {
                row.push(self.0.0[i][j] - other.0.0[i][j]);
            }
            result.push(row);
        }

        Some(MatrixOps(Matrix(result)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matrix_addition() {
        let matrix1 = MatrixOps(Matrix(vec![vec![8, 1], vec![9, 1]]));
        let matrix2 = MatrixOps(Matrix(vec![vec![1, 1], vec![1, 1]]));
        let expected = MatrixOps(Matrix(vec![vec![9, 2], vec![10, 2]]));

        assert_eq!(matrix1 + matrix2, Some(expected));
    }

    #[test]
    fn test_matrix_subtraction() {
        let matrix1 = MatrixOps(Matrix(vec![vec![1, 3], vec![2, 5]]));
        let matrix2 = MatrixOps(Matrix(vec![vec![3, 1], vec![1, 1]]));
        let expected = MatrixOps(Matrix(vec![vec![-2, 2], vec![1, 4]]));

        assert_eq!(matrix1 - matrix2, Some(expected));
    }

    #[test]
    fn test_matrix_addition_different_dimensions() {
        let matrix1 = MatrixOps(Matrix(vec![vec![1, 1], vec![1, 1]]));
        let matrix2 = MatrixOps(Matrix(vec![vec![1, 1, 3], vec![1, 1]]));

        assert_eq!(matrix1 + matrix2, None);
    }

    #[test]
    fn test_matrix_subtraction_different_dimensions() {
        let matrix1 = MatrixOps(Matrix(vec![vec![1, 3], vec![9, 1]]));
        let matrix2 = MatrixOps(Matrix(vec![vec![1, 1, 3], vec![1, 1]]));

        assert_eq!(matrix1 - matrix2, None);
    }
}