use crate::RomanDigit::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum RomanDigit {
	Nulla,
	I,
	V,
	X,
	L,
	C,
	D,
	M,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RomanNumber(pub Vec<RomanDigit>);

impl From<u32> for RomanDigit {
    fn from(num: u32) -> Self {
        match num {
            0 => RomanDigit::Nulla,
            1 => RomanDigit::I,
            5 => RomanDigit::V,
            10 => RomanDigit::X,
            50 => RomanDigit::L,
            100 => RomanDigit::C,
            500 => RomanDigit::D,
            1000 => RomanDigit::M,
            _ => panic!("Invalid Roman digit value"),
        }
    }
}

impl From<u32> for RomanNumber {
    fn from(mut num: u32) -> Self {
        // Special case for 0
        if num == 0 {
            return RomanNumber(vec![Nulla]);
        }

        let mut result = Vec::new();
        
        // Handle thousands (M)
        while num >= 1000 {
            result.push(M);
            num -= 1000;
        }
        
        // Handle hundreds (C, D, CM)
        if num >= 900 {
            result.push(C);
            result.push(M);
            num -= 900;
        } else if num >= 500 {
            result.push(D);
            num -= 500;
            
            // Add remaining hundreds
            while num >= 100 {
                result.push(C);
                num -= 100;
            }
        } else if num >= 400 {
            result.push(C);
            result.push(D);
            num -= 400;
        } else {
            // Add hundreds
            while num >= 100 {
                result.push(C);
                num -= 100;
            }
        }
        
        // Handle tens (X, L, XC)
        if num >= 90 {
            result.push(X);
            result.push(C);
            num -= 90;
        } else if num >= 50 {
            result.push(L);
            num -= 50;
            
            // Add remaining tens
            while num >= 10 {
                result.push(X);
                num -= 10;
            }
        } else if num >= 40 {
            result.push(X);
            result.push(L);
            num -= 40;
        } else {
            // Add tens
            while num >= 10 {
                result.push(X);
                num -= 10;
            }
        }
        
        // Handle ones (I, V, IX)
        if num >= 9 {
            result.push(I);
            result.push(X);
            num -= 9;
        } else if num >= 5 {
            result.push(V);
            num -= 5;
            
            // Add remaining ones
            while num >= 1 {
                result.push(I);
                num -= 1;
            }
        } else if num >= 4 {
            result.push(I);
            result.push(V);
            num -= 4;
        } else {
            // Add ones
            while num >= 1 {
                result.push(I);
                num -= 1;
            }
        }
        
        RomanNumber(result)
    }
}