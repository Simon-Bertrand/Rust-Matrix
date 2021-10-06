
pub mod constructors;
mod display;
pub mod matrix;
mod operators;
pub mod decomposition;
pub mod transformations;

mod math;

#[derive(Debug)]
pub struct Matrix<T> {
    pub values: Vec<T>,
    pub shape : (i32,i32),
}


