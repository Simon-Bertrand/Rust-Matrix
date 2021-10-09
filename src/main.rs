#![allow(dead_code)]
#![allow(unused_variables)]




include!("mod.rs");

//use crate::matrix::constructors::Constructors;
//use num_rational::Ratio;
fn main() {

  
    let mat_a: Matrix<i32> = Matrix::<i32> {values: vec![1, 2, 3, 4, 5, 6, 7, 8, 0], shape:(3,3)};
    *mat_a.col(0).get_mut(0,0) = 5;
    mat_a.col(0).show();
    println!("{}", mat_a.col(0))
}