mod display;
mod matvector;

pub enum MatVector<'a> {
    Int(Vec<&'a i32>),
    Float(Vec<&'a f64>),
    Bool(Vec<&'a bool>),
    Null
}