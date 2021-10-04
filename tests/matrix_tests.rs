use numru;
use crate::numru::matrix::Matrix;

#[test]
fn concatenation_both_axes() {
    let mut matrix_a: Matrix<f64> = Matrix::<f64> {values: vec![1.0, 2.0, 3.0, 4.0], shape:(2,2)};
    let right_part: Matrix<f64> = Matrix::<f64> {values: vec![3.0,4.0], shape:(2,1)};
    let down_part: Matrix<f64> = Matrix::<f64> {values: vec![5.0,6.0,7.0], shape:(1,3)};
    let result: Matrix<f64> = Matrix::<f64> {values: vec![1.0,2.0, 3.0, 3.0, 4.0, 4.0, 5.0, 6.0, 7.0], shape:(3,3)};
    assert_eq!(*matrix_a.concat(&right_part).concat(&down_part), result, "Testing concatenation on both axes.");

}
#[test]
fn matrixe_inverse() {
    let mat_a: Matrix<f64> = Matrix::<f64> {values: vec![1.0/3.0, 2.0/3.0, -2.0/3.0, -2.0/3.0, 2.0/3.0, 1.0/3.0, 2.0/3.0, 1.0/3.0, 2.0/3.0], shape:(3,3)};
    let id: Matrix<f64> = Matrix::<f64>::fill_diagonal(3,1.0);
    assert_eq!(mat_a.dot(&mat_a.invert()).round(), id, "Testing inverting");
}
#[test]
fn dot_product() {
    let mat_a: Matrix<f64> = Matrix::<f64> {values: vec![1.0/3.0, 2.0/3.0, -2.0/3.0, -2.0/3.0, 2.0/3.0, 1.0/3.0, 2.0/3.0, 1.0/3.0, 2.0/3.0], shape:(3,3)};
    let mat_b: Matrix<f64> = Matrix::<f64> {values: vec![1.0/3.0, -2.0/3.0, 2.0/3.0, 2.0/3.0, 2.0/3.0, 1.0/3.0, -2.0/3.0, 1.0/3.0, 2.0/3.0], shape:(3,3)};
    let id: Matrix<f64> =Matrix::fill_diagonal(3,1.0);
    assert_eq!(mat_a.dot(&mat_b), id, "Testing A*B = Id");
}
#[test]
fn reglin_normal() {
    let ans = Matrix::<f64> {values: vec![2.0740558793576986,25.87379024570168], shape:(2,1)};
    assert_eq!(Matrix::<f64>::from_csv("./reg_data.csv").reglin_normal(), ans, "Testing the linear regression");
}
#[test]
fn resolve_system() {
    let mat_a: Matrix<i32> = Matrix::<i32> {values: vec![1, 2, 3, 4, 5, 6, 7, 8, 0], shape:(3,3)};
    let b: Matrix<i32> = Matrix::<i32> {values: vec![6, 15, 15], shape:(3,1)};
    let sol: Matrix<i32> =Matrix::<i32> {values: vec![1, 1, 1], shape:(3,1)};
    assert_eq!(mat_a.clone_to_ratio().dot(&mat_a.resolve_system(&b)), b, "Testing A.x = b");
    assert_eq!(mat_a.resolve_system(&b), sol , "Testing x = sol");
}
#[test]
fn determinant() {
    let matrix_a: Matrix<f64> = Matrix::<f64> {values: vec![1.0,0.0,6.0,3.0,4.0,15.0,5.0,6.0,21.0], shape:(3,3)};
    assert_eq!(matrix_a.det(), -18.0, "Testing det(A) = -18");
}

#[test]
fn lu_decomposition(){
    let mat_a: &mut Matrix<f64> = &mut Matrix::<f64> {values: vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 0.0], shape:(3,3)};
    let (p, l, u,_) = mat_a.lu_decomposition();
    assert_eq!(p.dot(&mat_a),l.dot(&u), "PA = LU");
}
