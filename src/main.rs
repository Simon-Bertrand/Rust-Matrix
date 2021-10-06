#![allow(dead_code)]
#![allow(unused_variables)]




include!("mod.rs");

use crate::matrix::constructors::Constructors;

fn main() {

    let mat_a: Matrix<i32> = Matrix::<i32> {values: vec![1, 2, 3, 4, 5, 6, 7, 8, 0], shape:(3,3)};
    let ans1: Matrix<i32> = Matrix::<i32> {values: vec![3, 6, 8], shape:(3,1)};
    let ans2: Matrix<i32> = Matrix::<i32> {values: vec![7, 8, 6], shape:(1,3)};

    mat_a.show();

    mat_a.min(false).show();


}