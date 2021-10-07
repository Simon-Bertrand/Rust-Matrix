#![allow(dead_code)]
#![allow(unused_variables)]




include!("mod.rs");

use crate::matrix::constructors::Constructors;
use num_rational::Ratio;
fn main() {

    let mat_a: Matrix<i32> = Matrix::<i32> {values: vec![1, 5, 3, 4, 5, 6, 7, 8, 0], shape:(3,3)};

    let ans1: Matrix<i32> = Matrix::<i32> {values: vec![6], shape:(1,1)};
    let ans2: Matrix<Ratio<i32>> = Matrix::<Ratio<i32>> {values: vec![Ratio::from(5)], shape:(1,1)};
    let ans3: Matrix<Ratio<i32>> = Matrix::<Ratio<i32>> {values: vec![Ratio::from(13)], shape:(1,1)};

    
    mat_a.show();

    mat_a.sum(true).min(true).show();

    mat_a.norm(true).min(true).show();

    mat_a.max(false).norm(true).show();


}