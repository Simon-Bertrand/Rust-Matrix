use rust_matrix::matrix::*;
use crate::math::decomposition::LUDecomposition;


use num_rational::Rational32;


#[test]
fn resolve_system_int() {
    let mat_a: Matrix<i32> = Matrix::<i32> {values: vec![1, 2, 3, 4, 5, 6, 7, 8, 0], shape:(3,3)};
    let b: Matrix<i32> = Matrix::<i32> {values: vec![6, 15, 15], shape:(3,1)};
    let sol: Matrix<i32> =Matrix::<i32> {values: vec![1, 1, 1], shape:(3,1)};
    assert_eq!(mat_a.clone_to_ratio().dot(&mat_a.resolve_system(&b)), b, "Testing A.x = b");
    assert_eq!(mat_a.resolve_system(&b), sol , "Testing x = sol");
}
#[test]
fn determinant_int() {
    let matrix_a: Matrix<i32> = Matrix::<i32> {values: vec![1,0,6,3,4,15,5,6,21], shape:(3,3)};
    assert_eq!(matrix_a.det(), Rational32::from(-18), "Testing det(A) = -18");
}

#[test]
fn lu_decomposition_int(){
    let mat_a: &mut Matrix<i32> = &mut Matrix::<i32> {values: vec![1, 2, 3, 4, 5, 6, 7, 8, 0], shape:(3,3)};
    let (p, l, u,_) = mat_a.lu_decomposition();
    assert_eq!(p.dot(&mat_a.clone_to_ratio()),l.dot(&u), "PA = LU");
}
