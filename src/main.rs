#![allow(dead_code)]
#![allow(unused_variables)]




include!("mod.rs");

use crate::matrix::constructors::Constructors;
use num_rational::Ratio;
fn main() {

    let mut mat_a: Matrix<i32> = Matrix::<i32> {values: vec![1, 2, 3, 4, 5, 6, 7, 8], shape:(2,4)};
    mat_a.show();
    mat_a.transpose().show();
    mat_a.show();
}