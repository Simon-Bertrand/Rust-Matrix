mod specific_impl;
mod utils;

pub mod math;
pub mod matrix;
pub mod constructors;
pub mod transformations;

#[derive(Debug)]
pub struct Matrix<T> {
    pub values: Vec<T>,
    pub shape : (usize,usize),
}