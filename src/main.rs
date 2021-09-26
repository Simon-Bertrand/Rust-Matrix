#![allow(dead_code)]
#![allow(unused_variables)]




include!("mod.rs");
use crate::matrix::constructors::Constructors;

fn main() {
    let mat_a: Matrix<f64> = Matrix::<f64> {values: vec![1.0/3.0, 2.0/3.0, -2.0/3.0, -2.0/3.0, 2.0/3.0, 1.0/3.0, 2.0/3.0, 1.0/3.0, 2.0/3.0], shape:(3,3)};
    let id: Matrix<f64> = Matrix::<f64>::fill_diagonal(3,1.0);



    mat_a.dot(&mat_a.invert()).show();
}