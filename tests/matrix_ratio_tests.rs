use numru;
use crate::numru::matrix::Matrix;
use crate::numru::matrix::decomposition::LUDecomposition;

use num_rational::Rational32;

#[test]
fn matrixe_inverse_ratio() {
    let mat_a: Matrix<Rational32> = Matrix::<Rational32> {values: vec![Rational32::new(1,3), Rational32::new(2,3), Rational32::new(-2,3), Rational32::new(-2,3), Rational32::new(2,3), Rational32::new(1,3), Rational32::new(2,3), Rational32::new(1,3), Rational32::new(2,3)], shape:(3,3)};
    let id: Matrix<Rational32> = Matrix::<Rational32>::fill_diagonal(3,Rational32::from(1));
    assert_eq!(mat_a.dot(&mat_a.invert()), id, "Testing inverting");
}
