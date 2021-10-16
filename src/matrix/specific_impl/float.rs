#![allow(dead_code)]

use crate::matrix::*;



pub trait MatrixFloats{
    fn exp(&self) -> Self;
    fn sin(&self) -> Self ;
    fn cos(&self) -> Self ;
    fn log(&self, base : usize) -> Self;
    fn powi_ew(&self, powi : i32) -> Self;
    fn powf_ew(&self, powf : f32) -> Self;
    fn round(&mut self) -> &mut Self;
}


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

macro_rules! sub_impl {
    ($($t:ty)*) => ($(
        impl MatrixFloats for Matrix<$t> {
            fn exp(&self) -> Matrix<$t> {
                core_apply(&self, | v | v.exp())
            }
            fn sin(&self) -> Matrix<$t> {
                core_apply(&self, | v | v.sin())
            }
            fn cos(&self) -> Matrix<$t> {
                core_apply(&self, | v | v.cos())
            }
            fn log(&self, base : usize) -> Matrix<$t> {
                core_apply(&self, | v | v.log(base as $t))
            }
            fn powi_ew(&self, powi : i32) -> Matrix<$t> {
                core_apply(&self, | v | v.powi(powi))
            }
            fn powf_ew(&self, powf : f32) -> Matrix<$t> {
                core_apply(&self, | v | v.powf(powf as $t))
            }
            fn round(&mut self) -> &mut Self{
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
sub_impl! {f32 f64}





impl<T : Copy> Matrix<T> {
    fn apply(&self, f: impl Fn(T)->T) -> Matrix<T> {
        core_apply(&self, f)
    }
}
