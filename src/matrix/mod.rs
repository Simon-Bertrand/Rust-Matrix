
pub mod constructors;
mod display;
pub mod matrix;
mod operators;

pub mod conversion;

pub mod decomposition;
mod transformations;
pub mod boolean;
mod exception;
pub mod math;

#[derive(Debug)]
pub struct Matrix<T> {
    pub values: Vec<T>,
    pub shape : (usize,usize),
}


