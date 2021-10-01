#![allow(dead_code)]
#![allow(unused_variables)]




include!("mod.rs");
use crate::matrix::constructors::Constructors;

fn main() {
let fraction = Fraction::new(5,2);
println!("{}", fraction);
}