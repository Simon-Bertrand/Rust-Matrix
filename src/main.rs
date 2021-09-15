#![allow(dead_code)]
#![allow(unused_variables)]




include!("mod.rs");
use crate::matrix::constructors::Constructors;

fn main() {
    let n=5;
    let m = 5;

    
    let fill = &Matrix::fill(5,5,3.0);
    let ones = &Matrix::fill(5,5,1.0);
    let zeros = &Matrix::fill(5,5,0);

    println!("{}",5.0*ones/ fill);
    println!("{}", fill);

}
