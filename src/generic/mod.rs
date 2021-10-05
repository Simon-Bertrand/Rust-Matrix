mod operators;
mod generic;
use num_rational::Ratio;


pub trait AllowedTypes {}
impl AllowedTypes for i32 {}
impl AllowedTypes for i64 {}
impl AllowedTypes for f32 {}
impl AllowedTypes for f64 {}
impl AllowedTypes for Ratio<i32> {}
impl AllowedTypes for Ratio<i64> {}
impl AllowedTypes for bool {}


pub struct GenericNum<T : AllowedTypes> {
    pub v : T,
}