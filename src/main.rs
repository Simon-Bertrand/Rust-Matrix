#![allow(dead_code)]
#![allow(unused_variables)]




include!("mod.rs");
use crate::matrix::constructors::Constructors;

fn main() {
    let n=5;
    let m = 5;

    
    let fill = &Matrix::fill(5,5,false);
    let ones = &Matrix::fill(5,5,1);
    let zeros = &Matrix::fill(5,5,0);

    let permut: Matrix<f64> = Matrix::<f64> {values: vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 0.0], shape:(3,3)};
    let b: Matrix<f64> = Matrix::<f64> {values: vec![6.0, 15.0, 15.0], shape:(3,1)};
    let sol: Matrix<f64> = Matrix::<f64> {values: vec![1.0, 1.0, 1.0], shape:(3,1)};

    permut.show();
    permut.dot(&sol).show();

    permut.resolve_system(&b);
 
    





}
