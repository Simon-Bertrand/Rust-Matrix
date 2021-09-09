


pub mod constructors;
mod display;
mod matrix;
mod operators;


pub struct Matrix<T> {
    values: Vec<T>,
    shape : (i32,i32),
}


