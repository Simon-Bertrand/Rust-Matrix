#![allow(dead_code)]
#![allow(unused_variables)]




include!("mod.rs");

use crate::matrix::constructors::Constructors;
//use num_rational::Rational32;

use matrix::math::Functions;



fn main() {

let mat_a = &Matrix::<f32>::fill(3,3,1.0);
(2.0 * mat_a).show();





}