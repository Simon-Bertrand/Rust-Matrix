use numru;
use crate::numru::matrix::Matrix;

#[test]
fn dot_product() {
    let mat_a: Matrix<f64> = Matrix::<f64> {values: vec![1.0/3.0, 2.0/3.0, -2.0/3.0, -2.0/3.0, 2.0/3.0, 1.0/3.0, 2.0/3.0, 1.0/3.0, 2.0/3.0], shape:(3,3)};
    let mat_b: Matrix<f64> = Matrix::<f64> {values: vec![1.0/3.0, -2.0/3.0, 2.0/3.0, 2.0/3.0, 2.0/3.0, 1.0/3.0, -2.0/3.0, 1.0/3.0, 2.0/3.0], shape:(3,3)};
    let id: Matrix<f64> =Matrix::fill_diagonal(3,1.0);
    assert_eq!(mat_a.dot(&mat_b), id, "Testing A*B = Id");
}
#[test]
fn resolve_system() {
    let mat_a: Matrix<f64> = Matrix::<f64> {values: vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 0.0], shape:(3,3)};
    let b: Matrix<f64> = Matrix::<f64> {values: vec![6.0, 15.0, 15.0], shape:(3,1)};
    let sol: Matrix<f64> =Matrix::<f64> {values: vec![1.0, 1.0, 1.0], shape:(3,1)};
    assert_eq!(mat_a.dot(&mat_a.resolve_system(&b)), b, "Testing A.x = b");
    assert_eq!(mat_a.resolve_system(&b), sol, "Testing x = sol");
}

