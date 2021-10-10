
pub mod constructors;
mod display;
pub mod matrix;
mod operators;
mod decomposition;
mod transformations;
pub mod boolean;

mod math;

#[derive(Debug)]
pub struct Matrix<T> {
    pub values: Vec<T>,
    pub shape : (usize,usize),
}


