mod utils;
mod matrixtraits;

pub mod math;
pub mod matrix;
pub mod matrixasks;
pub mod constructors;
pub mod transformations;
pub mod specific_impl;

#[derive(Debug)]
pub struct Matrix<T> {
    pub values: Vec<T>,
    pub shape : (usize,usize),
}