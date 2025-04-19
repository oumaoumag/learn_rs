use std::ops::{Add, Sub, Mul, Div};

pub trait Scalar: Add<Output = Self> + Sub<Output = Self> + Mul<Output = Self> + Div<Output = Self> + Sized + Copy {
    type Item;
    fn zero() -> Self::Item;
    fn one() -> Self::Item;
}

impl Scalar for u32 {
    type Item = Self;
    
    fn zero() -> Self::Item {
        0
    }
    
    fn one() -> Self::Item {
        1
    }
}

impl Scalar for u64 {
    type Item = Self;
    
    fn zero() -> Self::Item {
        0
    }
    
    fn one() -> Self::Item {
        1
    }
}

impl Scalar for i32 {
    type Item = Self;
    
    fn zero() -> Self::Item {
        0
    }
    
    fn one() -> Self::Item {
        1
    }
}

impl Scalar for i64 {
    type Item = Self;
    
    fn zero() -> Self::Item {
        0
    }
    
    fn one() -> Self::Item {
        1
    }
}

impl Scalar for f32 {
    type Item = Self;
    
    fn zero() -> Self::Item {
        0.0
    }
    
    fn one() -> Self::Item {
        1.0
    }
}

impl Scalar for f64 {
    type Item = Self;
    
    fn zero() -> Self::Item {
        0.0
    }
    
    fn one() -> Self::Item {
        1.0
    }
}