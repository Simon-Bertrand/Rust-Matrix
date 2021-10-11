#![allow(dead_code)]
#![allow(unused_variables)]


use rust_matrix::matrix::*;
use rust_matrix::matrix::constructors::Constructors;


//use num_rational::Rational32;




fn main() {

let mat_a = &Matrix::<f32>::fill(3,3,1.0);
(2.0 * mat_a + 5.0).show();





}