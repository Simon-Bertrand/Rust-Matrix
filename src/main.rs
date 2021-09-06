#![allow(dead_code)]
#![allow(unused_variables)]




include!("mod.rs");
use crate::matrix::constructors::Constructors;

fn main() {
    let n=5;
    let m = 5;

    /*
    let binary = &Matrix::rand_binary(n,m);
    let id = &Matrix::eye(m,"i32");
    let tri = &Matrix::tri(m, 0);
    let linstep = &Matrix::linstep(m, n, 1);
    
    tri.show();

    tri.col(1).show();
    */
    println!("{}", <Matrix<i32> as Constructors<i32>>::fill(n,m, 0 as i32));

}
