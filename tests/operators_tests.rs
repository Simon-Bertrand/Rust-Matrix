use rust_matrix::matrix::*;
use crate::constructors::Constructors;

use rust_matrix::matrix::specific_impl::boolean::MatrixBool;



#[test]
fn matrix_addition_and_sub_with_reals() {
    let mat_twos: &Matrix<f64> =&Matrix::fill(3,3,2.0);
    let mat_ones: &Matrix<f64> = &Matrix::fill(3,3,1.0);
    assert_eq!(mat_twos - 1.0, *mat_ones, "Testing Mat_of_two - 1 = Mat_of_ones");
    assert_eq!(*mat_twos, mat_ones + 1.0, "Testing Mat_of_two = Mat_of_ones + 1");
    
}
#[test]
fn matrix_mul_and_div_with_reals() {
    let mat_ones: &Matrix<f64> =&Matrix::fill(3,3,1.0);
    let mat_thirds: &Matrix<f64> =&Matrix::fill(3,3,1.0/3.0);
    assert_eq!(mat_ones/3.0, *mat_thirds, "Testing Mat_of_ones / 3 = Mat_of_thirds");
    assert_eq!(*mat_ones, 3.0*mat_thirds, "Testing Mat_of_ones =  3 *Mat_of_thirds");
}

#[test]
fn matrix_comparison() {
    let mat_a: Matrix<i32> = Matrix::<i32> {values: vec![1,2,3,4,5,6,7,8,9], shape:(3,3)};
    assert_eq!(mat_a.compare_greater(&2, false), mat_a.compare_lower(&2, true).not(), "Testing A>2 = not(A<=2)");
    assert_eq!( mat_a.compare_greater(&2, true), mat_a.compare_lower(&2, false).not(), "Testing A>=2 = not(A<2)");

}