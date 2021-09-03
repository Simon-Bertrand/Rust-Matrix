mod display;
mod matvector;
mod operators;

pub enum MatVector<'a> {
    Int(Vec<&'a i32>),
    Float(Vec<&'a f64>),
    Bool(Vec<&'a bool>),
    Null
}