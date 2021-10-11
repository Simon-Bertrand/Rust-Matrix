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


impl Matrix<bool> {
    pub fn and(&self, m : &Matrix<bool>) -> Matrix<bool> {
        core_add_mul(&self, m,true)
    }
    pub fn or(&self, m : &Matrix<bool>) -> Matrix<bool> {
        core_add_mul(&self, m,false)
    }

    pub fn not(&self) -> Matrix<bool> {
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





impl<T : Copy + Zero + PartialEq + std::fmt::Display> Matrix<T> {
    pub fn select(&self, mask : &Matrix<bool>)  -> Matrix<T> {
        if self.shape == mask.shape {
            let mut nb_el : usize = 0;
            Matrix::<T> {
                values : {
                    let mut r : Vec<T> = Vec::with_capacity(self.length());
                    for (i,el) in self.values.iter().enumerate() {
                        if *mask.values.get(i).unwrap() {
                            r.push(*el);
                            nb_el += 1;
                        }
                    }
                r},
                shape: (1,nb_el)
            }
        }
        else {
            exception::raise_exception(
                &"select",
                &mut String::from("The mask shape needs to be the same as self matrix."),
                String::from("Choose matrix A and B such as A.shape==B.shape"),
                100,
                10002);
                panic!();
        }
        
    }
}




pub struct MatrixMaskIter<'a, T> {
    data: &'a Matrix<T>,
    mask : &'a Matrix<bool>,
    index : usize,
}

impl<'a, T> Iterator for MatrixMaskIter<'a, T>{ 
    type Item = &'a T;
    fn next(&mut self) ->Option<Self::Item> {
     while self.index < self.data.length() {
         if *self.mask.values.get(self.index).unwrap() {
            self.index += 1;
            return Some(&self.data.values[self.index-1])
         }
         else {
            self.index += 1;
         }
     }
     None
    }
}


impl<'a,T> Matrix<T> {
    pub fn select_iter(&'a self, mask : &'a Matrix<bool>) -> MatrixMaskIter<'a, T> {
        if  mask.shape != self.shape {
            exception::raise_exception(
                &"select_iter",
                &mut String::from("The mask shape needs to be the same as self matrix."),
                String::from("Choose matrix A and B such as A.shape==B.shape"),
                100,
                10003);
                panic!();
        }
        else {
            MatrixMaskIter::<'a, T> { data: &self, mask: &mask, index : 0}
        }
    }

}

/*
impl<T : Mul + Copy + Zero> Mul<&Matrix<T>> for &Matrix<bool>
{
    type Output = Matrix<T>;
    fn mul(self, rhs: &Matrix<T>) -> Matrix<T> {
        Matrix::<T>{ 
            values: {
                let mut r: Vec<T> = Vec::with_capacity(&self.shape.0*(&self).shape.1); 
                for (i,el) in (&self).values.iter().enumerate() {
                    if *el {
                        r.push(rhs.values[i]);
                    }
                    else {
                        r.push(Zero::zero());
                    }
                }
                r},
            shape:self.shape}    
        }
}
*/