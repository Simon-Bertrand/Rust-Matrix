use numru;
use crate::numru::matrix::Matrix;
use crate::numru::matrix::decomposition::LUDecomposition;


#[test]
fn matrixe_inverse_float() {
    let mat_a: Matrix<f64> = Matrix::<f64> {values: vec![1.0/3.0, 2.0/3.0, -2.0/3.0, -2.0/3.0, 2.0/3.0, 1.0/3.0, 2.0/3.0, 1.0/3.0, 2.0/3.0], shape:(3,3)};
    let id: Matrix<f64> = Matrix::<f64>::fill_diagonal(3,1.0);
    assert_eq!(mat_a.dot(&mat_a.invert()).round(), id, "Testing inverting");
}

#[test]
fn reglin_normal_float() {
    let ans = Matrix::<f64> {values: vec![2.0740558793576986,25.87379024570168], shape:(2,1)};
    assert_eq!(Matrix::<f64>::from_csv("./reg_data.csv").reglin_normal(), ans, "Testing the linear regression");
}

#[test]
fn determinant_float() {
    let matrix_a: Matrix<f64> = Matrix::<f64> {values: vec![1.0,0.0,6.0,3.0,4.0,15.0,5.0,6.0,21.0], shape:(3,3)};
    assert_eq!(matrix_a.det().round(), -18.00, "Testing det(A) = -18");
}

#[test]
fn lu_decomposition_float(){
    let mat_a: &mut Matrix<f64> = &mut Matrix::<f64> {values: vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 0.0], shape:(3,3)};
    let (p, l, u,_) = mat_a.lu_decomposition();
    assert_eq!(p.dot(&mat_a),l.dot(&u), "PA = LU");
}
