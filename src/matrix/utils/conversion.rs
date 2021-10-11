use num_traits::Zero;
use num_traits::AsPrimitive;
use std::convert::From;

use crate::matrix::* ;


fn core_convert<T : AsPrimitive<U>, U: Copy + Zero>(this : &Matrix<T>) -> Matrix<U> 
where U : 'static {
    Matrix::<U> {
        values : {
            let mut r : Vec<U> = Vec::with_capacity(this.length());
            for i in 0..this.length() {r.push(AsPrimitive::as_(this.values[i]))}         

        r},
        shape: this.shape,
    }
}

macro_rules! impl_as_primitive {
    ($T: ty => $( $U: ty ),* ) => {
        $(
            impl From<&Matrix<$U>> for Matrix<$T> {
                fn from(item : &Matrix<$U>) -> Matrix<$T> {
                    core_convert::<$U,$T>(item)
                }
            }
        )*
    };
}



impl_as_primitive!(u8 => u8, i8, u16, i16, u32, i32, u64, isize, usize, i64, f32, f64);
impl_as_primitive!(i8 => u8, i8, u16, i16, u32, i32, u64, isize, usize, i64, f32, f64);
impl_as_primitive!(u16 => u8, i8, u16, i16, u32, i32, u64, isize, usize, i64, f32, f64);
impl_as_primitive!(i16 => u8, i8, u16, i16, u32, i32, u64, isize, usize, i64, f32, f64);
impl_as_primitive!(u32 => u8, i8, u16, i16, u32, i32, u64, isize, usize, i64, f32, f64);
impl_as_primitive!(i32 => u8, i8, u16, i16, u32, i32, u64, isize, usize, i64, f32, f64);
impl_as_primitive!(u64 => u8, i8, u16, i16, u32, i32, u64, isize, usize, i64, f32, f64);
impl_as_primitive!(i64 => u8, i8, u16, i16, u32, i32, u64, isize, usize, i64, f32, f64);
impl_as_primitive!(usize => u8, i8, u16, i16, u32, i32, u64, isize, usize, i64, f32, f64);
impl_as_primitive!(isize => u8, i8, u16, i16, u32, i32, u64, isize, usize, i64, f32, f64);
impl_as_primitive!(f32 => u8, i8, u16, i16, u32, i32, u64, isize, usize, i64, f32, f64);
impl_as_primitive!(f64 => u8, i8, u16, i16, u32, i32, u64, isize, usize, i64, f32, f64);


