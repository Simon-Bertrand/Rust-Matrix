#![allow(dead_code)]
#![allow(unused_variables)]



include!("mod.rs");


fn main() {
    let mat = &Matrix::rand_binary(7,2);
    let mat2 = &Matrix::eye(2,"i32");
    mat.show();
    mat2.show();
    (mat.dot(mat2)).show();

}
