use rust_matrix::matrix::*;




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

#[test]
fn transpose() {
    let mut mat_a: Matrix<i32> = Matrix::<i32> {values: vec![1, 2, 3, 4, 5, 6, 7, 8], shape:(2,4)};
    let mat_a_original_copy= Matrix::<i32> {values: vec![1, 2, 3, 4, 5, 6, 7, 8], shape:(2,4)};
    let ans: Matrix<i32> = Matrix::<i32> {values: vec![1, 5, 2, 6, 3, 7, 4, 8], shape:(4,2)};
    assert_eq!(mat_a.transpose(), &ans, "Transposing");
    assert_eq!(mat_a.transpose(), &mat_a_original_copy, "Double Transpose composition is equal to identity");
}

#[test]
fn min_max() {

    let mat_a: Matrix<i32> = Matrix::<i32> {values: vec![1, 2, 3, 4, 5, 6, 7, 8, 0], shape:(3,3)};
    let max_ans1: Matrix<i32> = Matrix::<i32> {values: vec![3, 6, 8], shape:(3,1)};
    let max_ans2: Matrix<i32> = Matrix::<i32> {values: vec![7, 8, 6], shape:(1,3)};

    let min_ans1: Matrix<i32> = Matrix::<i32> {values: vec![1, 4, 0], shape:(3,1)};
    let min_ans2: Matrix<i32> = Matrix::<i32> {values: vec![1, 2, 0], shape:(1,3)};

    assert_eq!(mat_a.max(true), max_ans1, "Matrix rows max");
    assert_eq!(mat_a.max(false), max_ans2, "Matrix columns max");
    assert_eq!(mat_a.min(true), min_ans1, "Matrix rows min");
    assert_eq!(mat_a.min(false), min_ans2, "Matrix columns min");
    assert_eq!(mat_a.max(true).max(true).values[0], *mat_a.max_all(), "Matrix all max");
    assert_eq!(mat_a.min(true).min(true).values[0], *mat_a.min_all(), "Matrix all min");
}

#[test]
fn sum_norm_mean() {
    let mat_a: Matrix<i32> = Matrix::<i32> {values: vec![1, 5, 3, 4, 5, 6, 7, 8, 0], shape:(3,3)};

    let ans1: Matrix<i32> = Matrix::<i32> {values: vec![9], shape:(1,1)};
    let ans2: Matrix<i32> = Matrix::<i32> {values: vec![35], shape:(1,1)};
    let ans3: Matrix<i32> = Matrix::<i32> {values: vec![149], shape:(1,1)};

    assert_eq!(mat_a.sum(true).min(true), ans1, "Matrix h-sum h-min");
    assert_eq!(mat_a.norm(true).min(true), ans2, "Matrix h-norm h-min");
    assert_eq!(mat_a.max(false).norm(true), ans3, "Matrix v-max h-norm");

}
