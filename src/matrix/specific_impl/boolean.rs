use std::cmp::PartialEq;
use num_traits::Zero;

use crate::matrix::*;
use crate::matrix::utils::exception;





fn core_add_mul(this : &Matrix<bool>, m : &Matrix<bool>, add: bool) -> Matrix<bool> {
    if this.shape == m.shape {
        Matrix::<bool> {
            values : { 
                let mut r : Vec<bool> = Vec::with_capacity(this.length());
                    if add {
                        for i in 0..this.shape.0 {
                            for j in 0..this.shape.1 {
                                r.push(*this.get(i,j) || *m.get(i,j));
                            }
                        }
                    }
                    else {
                        for i in 0..this.shape.0 {
                            for j in 0..this.shape.1 {
                                r.push(*this.get(i,j) && *m.get(i,j));
                            }
                        }
                    }
                r},
            shape : this.shape
        }
    }
    else {
        exception::raise_exception(
            &"core_add_mul",
            &mut String::from("Shapes are to the same to compute element-wise mul or add."),
            String::from("Choose matrix A and B such as A.shape==B.shape"),
            100,
            10001);
            panic!();
    }  
}




pub trait MatrixBool{
    fn and(&self, m : &Matrix<bool>) -> Matrix<bool>;
    fn or(&self, m : &Matrix<bool>) -> Matrix<bool>;
    fn not(&self) -> Matrix<bool>;
}

impl MatrixBool for Matrix<bool> {
    fn and(&self, m : &Matrix<bool>) -> Matrix<bool> {
        core_add_mul(&self, m,true)
    }

    fn or(&self, m : &Matrix<bool>) -> Matrix<bool> {
        core_add_mul(&self, m,false)
    }
    fn not(&self) -> Matrix<bool> {
        Matrix::<bool> {
            values : { 
                let mut r : Vec<bool> = Vec::with_capacity(self.length());
                for el in self.values.iter() {
                    r.push(!el);
                }
            r},
            shape : self.shape
        }
    }
}





