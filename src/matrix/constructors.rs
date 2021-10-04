use crate::matrix::*;


use rand::Rng;
use std::vec;
use num_traits::Zero;
trait Display {
    fn show(&self);
}

pub trait Constructors<T> {
    fn new(shape1 : i32, shape2 : i32) -> Self;
    fn fill(shape1 : i32, shape2 : i32, fill_value :T) -> Self;
}


impl<T : Clone + Zero> Constructors<T> for Matrix<T> {
    fn new(shape1 : i32, shape2 : i32) -> Matrix<T> {
        let zero : T = Zero::zero();
        return Matrix::fill(shape1,shape2, zero);
    }
    fn fill(shape1 : i32, shape2 : i32, fill_value : T) -> Matrix<T> {
        Matrix::<T>{values : vec![fill_value; (shape1*shape2) as usize], shape:(shape1,shape2)}
    }

}

use std::mem;
impl<T : Clone + Copy + Zero> Matrix<T> {
    pub fn fill_diagonal(shape : i32, value : T) -> Matrix<T>{
        let mut result = Matrix::new(shape,shape);
        for k in 0..shape {*result.get_mut(k,k) =value;}
        result
    }
    pub fn fill_tri(shape : i32, value : T, offset:i32) -> Matrix<T>{
        let mut result = Matrix::new(shape,shape);
        for i in 0..shape {
            for j in 0..shape {
                if i - (offset+1) < j {
                    *result.get_mut(i,j) = value;
                }
            }
        }
        result
    }
}

macro_rules! sub_impl {
    ($($t:ty)*) => ($(
        impl Matrix<$t> {
            pub fn rand_binary(shape1 : i32, shape2: i32) -> Matrix<$t> {
                let mut result = Matrix::fill(shape1,shape2,0 as $t);
                let mut rng = rand::thread_rng();
                for mut_el in result.values.iter_mut() {
                    *mut_el = rng.gen_range(0..=1) as $t;
                } result
            }
            
        }
    )*)
}

sub_impl! { usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 f32 f64 }



