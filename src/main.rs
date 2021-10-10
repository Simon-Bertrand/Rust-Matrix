#![allow(dead_code)]
#![allow(unused_variables)]




include!("mod.rs");

//use crate::matrix::constructors::Constructors;
//use num_rational::Rational32;
fn main() {

    let mut mat_a: Matrix<f64> = Matrix::<f64> {values: vec![-1.0, 1.0, 2.0, -2.0 ], shape:(2,2)};

    let mut mat_b: Matrix<f64> = Matrix::<f64> {values: vec![1.0, 2.0, 3.0, 4.0 ], shape:(2,2)};

    (&mat_a-&mat_b).show();


}