use std::ops::{Add, Sub};

#[derive(Debug, Clone, PartialEq)]
pub struct Matrix(pub Vec<Vec<i32>>);

impl Add for Matrix {
    type Output = Option<Matrix>;

    fn add(self, rhs: Matrix) -> Self::Output {
        if self.0.len() != rhs.0.len() || self.0.iter().zip(&rhs.0).any(|(a, b)| a.len() != b.len()) {
            return None;
        }

        let result = self.0.into_iter().zip(rhs.0.into_iter()).map(|(row_a, row_b)| {
            row_a.into_iter().zip(row_b.into_iter()).map(|(a, b)| a + b).collect()
        }).collect();

        Some(Matrix(result))
    }
}

impl Sub for Matrix {
    type Output = Option<Matrix>;

    fn sub(self, rhs: Matrix) -> Self::Output {
        if self.0.len() != rhs.0.len() || self.0.iter().zip(&rhs.0).any(|(a, b)| a.len() != b.len()) {
            return None;
        }

        let result = self.0.into_iter().zip(rhs.0.into_iter()).map(|(row_a, row_b)| {
            row_a.into_iter().zip(row_b.into_iter()).map(|(a, b)| a - b).collect()
        }).collect();

        Some(Matrix(result))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let matrix = Matrix(vec![vec![1, 2], vec![3, 4]]);
        let matrix_2 = Matrix(vec![vec![1, 1], vec![1, 1]]);
        assert_eq!(matrix + matrix_2, Some(Matrix(vec![vec![2, 3], vec![4, 5]])));
    }

    #[test]
    fn test_sub() {
        let matrix = Matrix(vec![vec![1, 2], vec![3, 4]]);
        let matrix_2 = Matrix(vec![vec![1, 1], vec![1, 1]]);
        assert_eq!(matrix - matrix_2, Some(Matrix(vec![vec![0, 1], vec![2, 3]])));
    }
}