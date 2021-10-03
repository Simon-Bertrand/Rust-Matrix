#![allow(dead_code)]
#![allow(unused_variables)]




include!("mod.rs");
use crate::matrix::constructors::Constructors;

fn main() {
  let mut matrix = Matrix::<f64>::fill_diagonal(5,3.0);
  matrix.invert().show();
}