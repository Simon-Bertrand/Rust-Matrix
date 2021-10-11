#![allow(dead_code)]

use crate::matrix::*;

use num_traits::Float;

pub struct Functions<T> { v : T}


pub trait MatrixFloats{}
macro_rules! sub_impl {
    ($($t:ty)*) => ($(
        impl MatrixFloats for Matrix<$t> {}
    )*)
}
sub_impl! {f32 f64}




fn core_apply<T : Copy,U>(this : &Matrix<T>, f : impl Fn(T) -> U) -> Matrix<U> {
    Matrix::<U> {
        values : {
            let mut r : Vec<U> = Vec::with_capacity(this.length());
            for i in 0..this.length(){
                r.push(f(this.values[i]));
            }
        r},
        shape:this.shape,
    }
}

impl<T : Copy> Matrix<T> {
    fn apply(&self, f: impl Fn(T)->T) -> Matrix<T> {
        core_apply(&self, f)
    }
}

impl<T : Copy + Float> Functions<T>{
    pub fn exp(this : &Matrix<T>) -> Matrix<T> {
        core_apply(this, | v | v.exp())
    }
    pub fn sin(this : &Matrix<T>) -> Matrix<T> {
        core_apply(this, | v | v.sin())
    }
    pub fn cos(this : &Matrix<T>) -> Matrix<T> {
        core_apply(this, | v | v.cos())
    }
    pub fn log(this : &Matrix<T>, base : T) -> Matrix<T> {
        core_apply(this, | v | v.log(base))
    }
    pub fn powi_ew(this : &Matrix<T>, powi : i32) -> Matrix<T> {
        core_apply(this, | v | v.powi(powi))
    }
    pub fn powf_ew(this : &Matrix<T>, powf : T) -> Matrix<T> {
        core_apply(this, | v | v.powf(powf))
    }

}




macro_rules! sub_impl {
    ($($t:ty)*) => ($(
        impl Matrix<$t> {
            pub fn round(mut self) -> Self{
                for el in self.values.iter_mut() {
                    if el.abs() < 1e-10 {
                        *el = 0.0;
                    }
                    else {
                        *el = (*el as $t * 1e10).round()*1e-10;
                    }   
                }
                self
            }
        }
)*)
}
sub_impl! { f32 f64 }
