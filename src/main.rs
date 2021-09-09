#![allow(dead_code)]
#![allow(unused_variables)]




include!("mod.rs");
use crate::matrix::constructors::Constructors;

fn main() {
    let n=5;
    let m = 5;

    
    let fill = &Matrix::fill(5,5,5);

    println!("{}", fill);
    println!("{}", fill*5);

}
