use crate::frac::*;

impl<T : std::fmt::Display + std::cmp::PartialOrd + PrimInt> std::fmt::Display for Fraction<T> {
    fn fmt(&self, _f: &mut std::fmt::Formatter) -> std::fmt::Result {
        Ok(println!("{}/{}", self.num, self.denom))
    }
}

