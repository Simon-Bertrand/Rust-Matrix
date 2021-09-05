mod display;
mod matvector;
mod operators;

pub enum MatVector<'a> {
    Int(Vec<&'a mut i32>),
    Float(Vec<&'a mut f64>),
    Bool(Vec<&'a mut bool>),
    Null
}