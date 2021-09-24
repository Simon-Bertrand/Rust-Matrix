#![allow(dead_code)]
#![allow(unused_variables)]




include!("mod.rs");
use crate::matrix::constructors::Constructors;

fn main() {
    let n=5;
    let m = 5;

    
    let fill = &Matrix::fill(5,5,false);
    let ones = &Matrix::fill(5,5,1);
    let zeros = &Matrix::fill(5,5,0);

    let eye = &Matrix::<i32>::fill_diagonal(5,5);
    for el in eye.col_iter(0) {
        println!("{}", el);
    }
    println!("{}", eye);

    let mut eyes =Matrix::fill(5,5,0);
    println!("{}", &eyes);

    *eyes.get_mut(1,1) =1;
    println!("{}", (ones.dot(eye)));

    Matrix::<f64>::fill_tri(5,5.0,-2).flatten().show();
}
