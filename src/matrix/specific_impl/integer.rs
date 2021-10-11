use crate::matrix::*;
use num_rational::Ratio;

macro_rules! sub_impl {
    ($($t:ty)*) => ($(
        impl Matrix<$t>{
            pub fn clone_to_ratio(&self) -> Matrix<Ratio<$t>>{
                Matrix::<Ratio<$t>> {
                    values:self.values.iter().map(|v : &$t| Ratio::from_integer(*v)).collect(),
                    shape:self.shape,
                }
            }
        }
        
    )*)
}

sub_impl! {i32 i64}

