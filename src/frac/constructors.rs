use crate::frac::*;


impl<T: PrimInt> Fraction<T> {
    pub fn new(numerator : T, denominator : T) -> Fraction<T> {
        Fraction::<T> {num : numerator, denom : denominator}
    }
}