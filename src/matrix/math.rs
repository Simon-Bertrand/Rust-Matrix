use num_traits::Zero;
use num_rational::Ratio;
use crate::matrix::Matrix;
use crate::matrix::constructors::Constructors;

use num_traits::NumOps;
use num_traits::FromPrimitive;
    


fn core_reglin_normal<T : FromPrimitive + NumOps + Into<T> + Clone + Copy + Zero>(this : &Matrix<T>) -> (Matrix<T>,Matrix<T>) {
    if this.shape.1 != 2 {
        eprintln!("\nfn reglin_normal(&self) >>> The column shape needs to be equal to 2.\n");
        std::process::exit(-1);
    }
    else {
        let mut result = Matrix::<T>::fill(2,2,Zero::zero());
        let mut b = Matrix::<T>::fill(2,1,Zero::zero());

        for i in 0..this.shape.0{
            *result.get_mut(0,0) = *result.get_mut(0,0) +  *this.get(i,0) * *this.get(i,0);
            *result.get_mut(0,1) = *result.get_mut(0,1) +  *this.get(i,0);
            *result.get_mut(1,0) = *result.get(0,1);
            *b.get_mut(0,0) = *b.get_mut(0,0) +  *this.get(i,0) * *this.get(i as i32,1);
            *b.get_mut(1,0) = *b.get_mut(1,0)  + *this.get(i as i32,1);

        }
        *result.get_mut(1,1) = FromPrimitive::from_i32(this.shape.0).unwrap();

        (result,b)
        
    }
}

macro_rules! sub_impl {
    ($($t:ty)*) => ($(
        impl Matrix<$t> {
            pub fn reglin_normal(&self) -> Matrix<$t> {
                let (result,b) = core_reglin_normal(self);
                result.resolve_system(&b) 
            }
        }
    )*)
}
sub_impl! { f32 f64 }

macro_rules! sub_impl {
    ($($t:ty)*) => ($(
        impl Matrix<$t> {
            pub fn reglin_normal(&self) -> Matrix<Ratio<$t>> {
                let (result,b) = core_reglin_normal(self);
                result.resolve_system(&b) 
            }
        }
    )*)
}
sub_impl! { i32 i64}


macro_rules! sub_impl {
    ($($t:ty)*) => ($(
        impl Matrix<$t> {
            pub fn reglin_normal(&self) -> Matrix<$t> {
                let (result,b) = core_reglin_normal(self);
                result.resolve_system(&b) 
            }
        }
    )*)
}
sub_impl! { Ratio<i32> Ratio<i64>}



// 

/*
macro_rules! sub_impl {
    ($($t:ty)*) => ($(
        impl Matrix<$t> {
            pub fn reglin_normal(&self) -> Matrix<$t> {
                if self.shape.1 != 2 {
                    eprintln!("\nfn reglin_normal(&self) >>> The column shape needs to be equal to 2.\n");
                    std::process::exit(-1);
                }
                else {
                    let mut result = Matrix::<$t>::fill(2,2,0.0);
                    let mut b = Matrix::<$t>::fill(2,1,0.0);

                    for i in 0..self.shape.0{
                        *result.get_mut(0,0) +=  self.get(i,0)*self.get(i,0);
                        *result.get_mut(0,1) += self.get(i,0);
                        *result.get_mut(1,0) = *result.get(0,1);
                        *b.get_mut(0,0) +=  self.get(i,0) * *self.get(i as i32,1);
                        *b.get_mut(1,0) +=  *self.get(i as i32,1);
     
                    }
                    *result.get_mut(1,1) = self.shape.0 as $t;
        
                    result.resolve_system(&b) 
                }
            }
            
        }
    )*)
}

sub_impl! { f32 f64 }



macro_rules! sub_impl {
    ($($t:ty)*) => ($(
        impl Matrix<$t> {
            pub fn reglin_normal(&self) -> Matrix<Ratio<$t>> {
                if self.shape.1 != 2 {
                    eprintln!("\nfn reglin_normal(&self) >>> The column shape needs to be equal to 2.\n");
                    std::process::exit(-1);
                }
                else {
                    let mut result = Matrix::<$t>::fill(2,2,Zero::zero());
                    let mut b = Matrix::<$t>::fill(2,1,Zero::zero());
        
                    for i in 0..self.shape.0{
                        *result.get_mut(0,0) +=  self.get(i,0)*self.get(i,0);
                        *result.get_mut(0,1) += self.get(i,0);
                        *result.get_mut(1,0) = *result.get(0,1);
                        *b.get_mut(0,0) +=  self.get(i,0) * *self.get(i as i32,1);
                        *b.get_mut(1,0) +=  *self.get(i as i32,1);
     
                    }
                    *result.get_mut(1,1) = self.shape.0 as $t;
        
                    result.resolve_system(&b) 
                }
            }
            
        }
    )*)
}

sub_impl! { i32 i64 }

*/