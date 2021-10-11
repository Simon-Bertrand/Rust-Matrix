use num_traits::Zero;
use num_traits::Float;
use num_rational::Ratio;
use num_traits::NumOps;
use num_traits::FromPrimitive;
    

use crate::matrix::* ;
use crate::matrix::constructors::Constructors;
use crate::matrix::math::decomposition::LUDecomposition;


pub struct Functions<T> { v : T}


fn core_apply<T : Copy,U>(this : &Matrix<T>, f : impl Fn(T) -> U) -> Matrix<U> {
    Matrix::<U> {
        values : {
            let mut r : Vec<U> = Vec::with_capacity(this.length());
            for i in 0..this.length(){
                r.push(f(this.values[i]));
            }
        r},
        shape:this.shape,
    }
}

impl<T : Copy> Matrix<T> {
    fn apply(&self, f: impl Fn(T)->T) -> Matrix<T> {
        core_apply(&self, f)
    }
}

impl<T : Copy + Float> Functions<T>{
    pub fn exp(this : &Matrix<T>) -> Matrix<T> {
        core_apply(this, | v | v.exp())
    }
    pub fn sin(this : &Matrix<T>) -> Matrix<T> {
        core_apply(this, | v | v.sin())
    }
    pub fn cos(this : &Matrix<T>) -> Matrix<T> {
        core_apply(this, | v | v.cos())
    }
    pub fn log(this : &Matrix<T>, base : T) -> Matrix<T> {
        core_apply(this, | v | v.log(base))
    }
    pub fn powi_ew(this : &Matrix<T>, powi : i32) -> Matrix<T> {
        core_apply(this, | v | v.powi(powi))
    }
    pub fn powf_ew(this : &Matrix<T>, powf : T) -> Matrix<T> {
        core_apply(this, | v | v.powf(powf))
    }

}








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
            *b.get_mut(0,0) = *b.get_mut(0,0) +  *this.get(i,0) * *this.get(i,1);
            *b.get_mut(1,0) = *b.get_mut(1,0)  + *this.get(i,1);

        }
        *result.get_mut(1,1) = FromPrimitive::from_usize(this.shape.0).unwrap();

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





fn core_min_max_all<T : PartialOrd>(this : &Matrix<T>, is_max : bool) -> &T {
    let mut lhs : &T = &this.values[0];
    for rhs in this.values.iter() {
        if is_max {if lhs < rhs  { lhs = rhs;}} else {if lhs > rhs  { lhs = rhs;}}
    }
    lhs
}

fn core_min_max<T : PartialOrd + Zero + Clone + Copy>(this : &Matrix<T>, axis : bool, is_max : bool) -> Matrix<T>{
    if this.is_col() || this.is_row() {
        return Matrix::<T> {
            values: vec![{
                let mut lhs : T = this.values[0];
                for rhs in this.values.iter() {if is_max {if lhs < *rhs  { lhs = *rhs;}} else {if lhs > *rhs  { lhs = *rhs;}}}
                lhs
            }],
            shape : (1,1)
        }
    }
    else {
        if axis { 
            let mut result : Matrix<T> = Matrix::fill(this.shape.0,1, Zero::zero());
            for j in 0..this.shape.0 {
                let mut lhs : T = *this.get(j,0);
                for rhs in this.row_iter(j) {if is_max {if lhs < *rhs  { lhs = *rhs;}} else {if lhs > *rhs  { lhs = *rhs;}}}
                *result.get_mut(j,0) = lhs;
            }
            result
            
        }
        else {
            let mut result : Matrix<T> = Matrix::fill(1,this.shape.1, Zero::zero());
            for i in 0..this.shape.1 {
                let mut lhs : T = *this.get(0,i);
                for rhs in this.col_iter(i)  {if is_max {if lhs < *rhs  { lhs = *rhs;}} else {if lhs > *rhs  { lhs = *rhs;}}}
                *result.get_mut(0,i) = lhs;
            }
            result
        }}
}

impl<T : PartialOrd + Zero + Clone + Copy> Matrix<T> {
    pub fn max(&self, axis : bool) -> Matrix<T> {
        core_min_max(self, axis, true)
    }
    pub fn min(&self, axis : bool) -> Matrix<T> {
        core_min_max(self, axis, false)
    }

}

impl<T : Ord + Zero> Matrix<T> {
    pub fn max_all(&self) -> &T {
        core_min_max_all(self, true)
    }
    pub fn min_all(&self) -> &T {
        core_min_max_all(self, false)
    }
}




impl<T : std::cmp::PartialOrd<T> + Zero + Clone + Copy> Matrix<T> {
    pub fn sum_all(&self) -> T {
        let mut t_s : T =Zero::zero();
        for el in self.values.iter()  { t_s = t_s + *el;}
        t_s
    }
}

fn core_sum_mean_norm<T : std::iter::Sum + NumOps + Zero + Copy>(this : &Matrix<T>, axis:bool, reduct : &T , squared : bool) -> Matrix<T> {
    if this.is_col() || this.is_row() {
        Matrix::<T> {
            values: {
                let mut r : Vec<T> = Vec::with_capacity(1);
                let mut t_s : T  = Zero::zero(); 
                for el in this.values.iter().map(|v| if squared {*v**v} else {*v}) {t_s = t_s + el}
                r.push(t_s/ *reduct);
                r
            },
            shape : (1,1)
        }
    } 
    else {
        if axis {
            return Matrix::<T> {values: {
                let mut r : Vec<T> = Vec::with_capacity(this.shape.0);
                for i in 0..this.shape.0 {
                    let mut t_s : T  = Zero::zero(); 
                    for el in this.row_iter(i)  {t_s = t_s + {if squared {*el**el} else {*el}};}
                    r.push(t_s / *reduct )
                }   
                r}, shape:(this.shape.0,1)}      
        }
        else {
            return Matrix::<T> {values: {
                let mut r : Vec<T> = Vec::with_capacity(this.shape.0);
                for j in 0..this.shape.1 {
                    let mut t_s :T  = Zero::zero();
                    for el in this.col_iter(j)  {t_s = t_s + {if squared {*el**el} else {*el}};}
                    r.push(t_s / *reduct)
                } 
                r}, shape:(1,this.shape.1)}
        }
    }
}

macro_rules! sub_impl {
    ($($t:ty)*) => ($(
        impl Matrix<$t> {
            pub fn mean(&self, axis : bool) -> Matrix<$t> {
                core_sum_mean_norm(self, axis, &{if axis {self.shape.1 as $t} else {self.shape.0 as $t}}, false)
            }
            pub fn norm(&self, axis : bool) -> Matrix<$t> {
                core_sum_mean_norm(self, axis, &1.0, true)
            }
            pub fn sum(&self, axis : bool) -> Matrix<$t> {
                core_sum_mean_norm(self, axis, &1.0, false)
            }
        }
    )*)
}
sub_impl! { f32 f64 }

macro_rules! sub_impl {
    ($($t:ty)*) => ($(
        impl Matrix<$t> {
            pub fn mean(&self, axis : bool) -> Matrix<Ratio<$t>> {
                core_sum_mean_norm(&self.clone_to_ratio(), axis, &{if axis {Ratio::new((self.shape.1 as i32).into(),1)} else {Ratio::new((self.shape.0 as i32).into(),1)}} , false)
            }
            pub fn norm(&self, axis : bool) -> Matrix<$t> {
                core_sum_mean_norm(self, axis, &1 , true)
            }
            pub fn sum(&self, axis : bool) -> Matrix<$t> {
                core_sum_mean_norm(self, axis, &1 , false)
            }
        }
    )*)
}
sub_impl! { i32 i64 }

macro_rules! sub_impl {
    ($($t:ty)*) => ($(
        impl Matrix<$t> {
            pub fn mean(&self, axis : bool) -> Matrix<$t> {
                core_sum_mean_norm(&self, axis, &{if axis {Ratio::new((self.shape.1 as i32).into(),1)} else {Ratio::new(1,(self.shape.0 as i32).into())}} , false)
            }
            pub fn norm(&self, axis : bool) -> Matrix<$t> {
                core_sum_mean_norm(self, axis, &Ratio::new(1,1) , true)
            }
            pub fn sum(&self, axis : bool) -> Matrix<$t> {
                core_sum_mean_norm(self, axis, &Ratio::new(1,1) , false)
            }
        }
    )*)
}
sub_impl! { Ratio<i32> Ratio<i64> }



