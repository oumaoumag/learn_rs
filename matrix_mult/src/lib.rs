use std::ops::Mul;

#[derive(Debug, Clone)]
pub struct Matrix<T>(pub Vec<Vec<T>>);

impl<T> Matrix<T> {
    pub fn number_of_rows(&self) -> usize {
        self.0.len()
    }

    pub fn number_of_cols(&self) -> usize {
        if self.0.is_empty() {
            0
        } else {
            self.0[0].len()
        }
    }

    pub fn row(&self, n: usize) -> Vec<T>
    where
        T: Clone,
    {
        self.0[n].clone()
    }

    pub fn col(&self, n: usize) -> Vec<T>
    where
        T: Clone,
    {
        self.0.iter().map(|row| row[n].clone()).collect()
    }
}

impl<T> Mul for Matrix<T>
where
    T: Clone + std::ops::Add<Output = T> + std::ops::Mul<Output = T> + Default,
{
    type Output = Option<Matrix<T>>;

    fn mul(self, rhs: Matrix<T>) -> Option<Matrix<T>> {
        let m = self.number_of_rows();
        let n = self.number_of_cols();
        let p = rhs.number_of_rows();
        let q = rhs.number_of_cols();

        if n != p {
            return None;
        }

        let mut result = vec![vec![T::default(); q]; m];
        for i in 0..m {
            for j in 0..q {
                let mut sum = T::default();
                for k in 0..n {
                    sum = sum + self.0[i][k].clone() * rhs.0[k][j].clone();
                }
                result[i][j] = sum;
            }
        }
        Some(Matrix(result))
    }
}