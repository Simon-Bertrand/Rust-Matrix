mod constructors;
mod display;
mod matrix;
mod operators;


#[derive(Clone)]
pub enum Matrix {
    Int(MatrixStruct<i32>),
    Float(MatrixStruct<f64>),
    Bool(MatrixStruct<bool>),
    Null,
}


#[derive(Clone)]
pub struct MatrixStruct<T> {
    values: Vec<T>,
    shape : (i32,i32),
}