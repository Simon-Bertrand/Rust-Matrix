use numru;
use crate::numru::matrix::Matrix;

use num_rational::Rational32;

#[test]
fn concatenation_both_axes() {
    let mut matrix_a: Matrix<f64> = Matrix::<f64> {values: vec![1.0, 2.0, 3.0, 4.0], shape:(2,2)};
    let right_part: Matrix<f64> = Matrix::<f64> {values: vec![3.0,4.0], shape:(2,1)};
    let down_part: Matrix<f64> = Matrix::<f64> {values: vec![5.0,6.0,7.0], shape:(1,3)};
    let result: Matrix<f64> = Matrix::<f64> {values: vec![1.0,2.0, 3.0, 3.0, 4.0, 4.0, 5.0, 6.0, 7.0], shape:(3,3)};
    assert_eq!(*matrix_a.concat(&right_part).concat(&down_part), result, "Testing concatenation on both axes.");

}

#[test]
fn dot_product() {
    let mat_a: Matrix<f64> = Matrix::<f64> {values: vec![1.0/3.0, 2.0/3.0, -2.0/3.0, -2.0/3.0, 2.0/3.0, 1.0/3.0, 2.0/3.0, 1.0/3.0, 2.0/3.0], shape:(3,3)};
    let mat_b: Matrix<f64> = Matrix::<f64> {values: vec![1.0/3.0, -2.0/3.0, 2.0/3.0, 2.0/3.0, 2.0/3.0, 1.0/3.0, -2.0/3.0, 1.0/3.0, 2.0/3.0], shape:(3,3)};
    let id: Matrix<f64> =Matrix::fill_diagonal(3,1.0);
    assert_eq!(mat_a.dot(&mat_b), id, "Testing A*B = Id");
}
