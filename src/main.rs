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

    let u =vec![1,2,3];
    let u2=vec![4,5,6];
    let u3=vec![7,8,9];



    let mut V : Vec<&Vec<i32>>= Vec::new();
    V.push(&u);
    V.push(&u2);
    V.push(&u3);
    V.push(&u3);




    let M = Matrix::from_vec_of_vec_int(V);
    println!("{}", M);




}
