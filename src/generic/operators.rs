use std::ops::Mul;
use std::ops::Add;
use std::ops::Sub;
use std::ops::Div;
use std::cmp::PartialEq;

use crate::generic::*;


impl<T : AllowedTypes + Mul<Output = T>> Mul<GenericNum<T>> for GenericNum<T> 
{type Output = GenericNum<T>; fn mul(self, rhs: &GenericNum<T>) -> GenericNum<T>{GenericNum::<T> {v:rhs.v*self.v}}}

macro_rules! sub_impl {
    ($($t1:ty)*) => ($(
        impl Mul<GenericNum<($t)>> for GenericNum<($t)> {
            type Output = Matrix<$t>;
            fn mul(self, rhs: &Matrix<$t>) -> Matrix<$t>{
                rhs*(self)
            }
        }
    )*)
}

sub_impl! { (f64,i32) (usize,i32) }
