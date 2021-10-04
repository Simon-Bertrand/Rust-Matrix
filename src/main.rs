#![allow(dead_code)]
#![allow(unused_variables)]




include!("mod.rs");

fn main() {

  Matrix::<f64>::from_csv("./reg_data.csv").reglin_normal().show();
}