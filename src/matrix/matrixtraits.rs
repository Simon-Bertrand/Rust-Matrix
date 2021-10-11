use crate::matrix::Matrix;
use num_rational::Ratio;

pub trait MatrixTrait{}



macro_rules! sub_impl {
    ($($t:ty)*) => ($(
        impl MatrixTrait for Matrix<$t> {}
    )*)
}
sub_impl! {usize u8 u16 u32 u64 i8 i16 i32 i64 i128 Ratio<i32> Ratio<i64> f32 f64 bool}



pub trait MatrixNum{}
macro_rules! sub_impl {
    ($($t:ty)*) => ($(
        impl MatrixNum for Matrix<$t> {}
    )*)
}
sub_impl! {usize u8 u16 u32 u64 i8 i16 i32 i64 i128 Ratio<i32> Ratio<i64> f32 f64}

pub trait MatrixSignedNum{}
macro_rules! sub_impl {
    ($($t:ty)*) => ($(
        impl MatrixSignedNum for Matrix<$t> {}
    )*)
}
sub_impl! {i8 i16 i32 i64 i128 Ratio<i32> Ratio<i64> f32 f64}




