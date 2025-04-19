use std::ops::{Add, Mul};
use std::fmt::Debug;
use lalgebra_scalar::Scalar;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Vector<T: Scalar>(pub Vec<T>);

impl<T: Scalar> Add for Vector<T> {
    type Output = Option<Self>;

    fn add(self, other: Self) -> Self::Output {
        if self.0.len() != other.0.len() {
            return None;
        }
        
        let mut result = Vec::with_capacity(self.0.len());
        for i in 0..self.0.len() {
            result.push(self.0[i] + other.0[i]);
        }
        
        Some(Vector(result))
    }
}

impl<T: Scalar<Item = T>> Vector<T>
where
    T: Add<Output = T> + Mul<Output = T>,
{
    pub fn new() -> Self {
        Vector(Vec::new())
    }

    pub fn dot(&self, other: &Self) -> Option<T> {
        if self.0.len() != other.0.len() {
            return None;
        }
        
        let mut result = T::zero();
        for i in 0..self.0.len() {
            let product = self.0[i] * other.0[i];
            result = result + product;
        }
        
        Some(result)
    }
}