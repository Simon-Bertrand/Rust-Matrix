#![allow(dead_code)]
#![allow(unused_variables)]




include!("mod.rs");
use crate::matrix::constructors::Constructors;

fn main() {
  let mut matrix = Matrix::<i32>::fill_diagonal(5,3);
  matrix.clone_to_ratio().dot(&matrix.invert()).show();
}