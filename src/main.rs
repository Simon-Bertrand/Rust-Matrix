#![allow(dead_code)]
#![allow(unused_variables)]




include!("mod.rs");

//use crate::matrix::constructors::Constructors;
//use num_rational::Ratio;
fn main() {

  
    let matrix_a: Matrix<i32> = Matrix::<i32> {values: vec![1, 2, 3, 4], shape:(2,2)};
    
    matrix_a.show();
    for el in matrix_a.row_iter(1) {
        println!("{}", el);
    }

}