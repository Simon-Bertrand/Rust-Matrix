mod constructors;
mod display;
mod matrix;
mod operators;


#[derive(Clone)]
pub enum Matrix {
    Int(MatrixI),
    Float(MatrixF),
    Bool(MatrixB),
    Null,
}


#[derive(Clone)]
pub struct MatrixI {
    values: Vec<i32>,
    shape : (i32,i32),
}
#[derive(Clone)]
pub struct MatrixF {
    values: Vec<f64>,
    shape : (i32,i32),
}
#[derive(Clone)]
pub struct MatrixB {
    values: Vec<bool>,
    shape : (i32,i32),
}