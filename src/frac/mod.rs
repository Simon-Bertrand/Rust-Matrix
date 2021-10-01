use num_traits::PrimInt;



pub mod constructors;
mod display;
pub mod frac;
mod operators;



#[derive(Debug)]
pub struct Fraction<T : PrimInt> {
    pub num: T,
    pub denom : T,
}
