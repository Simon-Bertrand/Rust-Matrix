#![allow(dead_code)]
#![allow(unused_variables)]




include!("mod.rs");


fn main() {
    let n=5;
    let m = 5;

    let binary = &Matrix::rand_binary(n,m);
    let id = &Matrix::eye(m,"i32");
    let tri = &Matrix::tri(m, 0);
    let linstep = &Matrix::linstep(m, n, 1);
    tri.show();

    let v =&vec![1,2,3];
    let vect = &MatVector::from_vec(v);
    println!("{}", vect);




}
