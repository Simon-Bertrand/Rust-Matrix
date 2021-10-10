use std::ops::Mul;
use std::ops::Add;
use std::ops::Sub;
use std::ops::Div;
use std::cmp::PartialEq;
use std::cmp::PartialOrd;
use num_traits::Zero;
use num_rational::Ratio;
use num_traits::NumOps;
use crate::matrix::*;
use crate::matrix::constructors::Constructors;


impl<T : PartialEq> PartialEq<Matrix<T>> for Matrix<T> {
    fn eq(&self, other: &Self) -> bool {
        if other.shape == self.shape {
            for (i,el) in self.values.iter().enumerate() {
                if *el != other.values[i] {
                    return false;
                }
            }
            return true;
        }
        else {
            return false;
        }
    }
}

impl<T : num_integer::Integer + Clone> PartialEq<Matrix<T>> for Matrix<Ratio<T>> {
    fn eq(&self, other: &Matrix<T>) -> bool {
        if other.shape == self.shape {
            for (i,el) in other.values.iter().enumerate() {
                if !(self.values[i].is_integer()) {
                    return false;
                }
                else {
                    if *el != self.values[i].to_integer() {
                        return false;
                    }
                }
            }
            return true;
        }
        else {
            return false;
        }
    }
}

fn core_compare<T : Zero + PartialOrd>(this : &Matrix<T>, val : &T, comparison_type : (bool,bool,bool)) -> Matrix<bool> {
    Matrix::<bool> {
        values: {
            let mut r : Vec<bool> = Vec::with_capacity(this.length());
            match comparison_type {
                (false,false, false) =>for i in 0..this.length() {r.push(this.values[i]>*val)},
                (false,false, true) =>for i in 0..this.length() {r.push(this.values[i]>=*val)},
                (false, true, false) =>for i in 0..this.length() {r.push(this.values[i]<*val)},
                (false, true, true) =>for i in 0..this.length() {r.push(this.values[i]<=*val)},
                (true, false, false) =>for i in 0..this.length() {r.push(this.values[i]==*val)},
                _ => panic!("Unknow comparison type"),
            }

        r},
        shape: this.shape
    }
}

impl<T : Zero + PartialOrd> Matrix<T> {
    pub fn compare_greater(&self, val : &T, or_equal : bool) -> Matrix<bool> {
        core_compare(self, val, (false,false, or_equal))
    }
    pub fn compare_lower(&self, val : &T, or_equal : bool) -> Matrix<bool> {
        core_compare(self, val, (false,true, or_equal))
    }
    pub fn compare_equal(&self, val : &T) -> Matrix<bool> {
        core_compare(self, val, (false,true, false))
    }
}


macro_rules! sub_impl {
    ($($t:ty)*) => ($(
        impl Mul<&Matrix<$t>> for $t { type Output = Matrix<$t> ; fn mul(self, rhs: &Matrix<$t>) -> Matrix<$t> {core_add_mul_sub_div_with_reals(rhs, self, (false,true))}}
        impl Mul<Matrix<$t>> for $t { type Output = Matrix<$t> ; fn mul(self, rhs: Matrix<$t>) -> Matrix<$t> {core_add_mul_sub_div_with_reals(&rhs, self, (false,true))}}

        impl Add<&Matrix<$t>> for $t { type Output = Matrix<$t>; fn add(self, rhs: &Matrix<$t>) -> Matrix<$t> {core_add_mul_sub_div_with_reals(rhs, self, (false,false))}}
        impl Add<Matrix<$t>> for $t { type Output = Matrix<$t>; fn add(self, rhs: Matrix<$t>) -> Matrix<$t> {core_add_mul_sub_div_with_reals(&rhs, self, (false,false))}}

        impl Sub<&Matrix<$t>> for $t {type Output = Matrix<$t>;fn sub(self, rhs: &Matrix<$t>) -> Matrix<$t> {core_add_mul_sub_div_with_reals(rhs, self, (true,false))}}
        impl Sub<Matrix<$t>> for $t {type Output = Matrix<$t>;fn sub(self, rhs: Matrix<$t>) -> Matrix<$t> {core_add_mul_sub_div_with_reals(&rhs, self, (true,false))}}

        impl Div<&Matrix<$t>> for $t {type Output = Matrix<$t>;fn div(self, rhs: &Matrix<$t>) -> Matrix<$t>{core_add_mul_sub_div_with_reals(rhs, self, (true,true))}}
        impl Div<Matrix<$t>> for $t {type Output = Matrix<$t>;fn div(self, rhs: Matrix<$t>) -> Matrix<$t>{core_add_mul_sub_div_with_reals(&rhs, self, (true,true))}}
    )*)
}
sub_impl! { usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 f32 f64 Ratio<i32> Ratio<i64>}


fn core_add_mul_sub_div_with_reals<T : Copy + NumOps>(this : &Matrix<T>, val : T, operator : (bool,bool)) -> Matrix<T>{
    Matrix::<T>{ 
        values: {
            let mut r: Vec<T> = Vec::with_capacity(&this.shape.0*(&this).shape.1); 
            match operator {
                (false,false)=>for el in (&this).values.iter() {r.push(*el + val)},
                (false,true)=>for el in (&this).values.iter() {r.push(*el * val)},
                (true,false)=>for el in (&this).values.iter() {r.push(*el - val)},
                (true,true)=>for el in (&this).values.iter() {r.push(*el / val)},
            }
            r},
        shape:this.shape
    }    
}

impl<T : Copy + NumOps> Mul<T> for &Matrix<T>
{ type Output = Matrix<T>;
    fn mul(self, rhs: T) -> Matrix<T> {
       core_add_mul_sub_div_with_reals(&self, rhs, (false,true))
    }
}
impl<T : Copy + NumOps> Mul<T> for Matrix<T>
{ type Output = Matrix<T>;
    fn mul(self, rhs: T) -> Matrix<T> {&self * rhs}}


impl<T : Copy + NumOps> Add<T> for &Matrix<T>
{ type Output = Matrix<T>;
    fn add(self, rhs: T) -> Matrix<T> {
       core_add_mul_sub_div_with_reals(&self, rhs, (false,false))
    }
}
impl<T : Copy + NumOps> Add<T> for Matrix<T>
{ type Output = Matrix<T>;
    fn add(self, rhs: T) -> Matrix<T> {&self + rhs}}


impl<T : Copy + NumOps> Sub<T> for &Matrix<T>
{ type Output = Matrix<T>;
    fn sub(self, rhs: T) -> Matrix<T> {
       core_add_mul_sub_div_with_reals(&self, rhs, (true,false))
    }
}
impl<T : Copy + NumOps> Sub<T> for Matrix<T>
{ type Output = Matrix<T>;
    fn sub(self, rhs: T) -> Matrix<T> {&self - rhs}}


impl<T : Copy + NumOps> Div<T> for &Matrix<T>
{ type Output = Matrix<T>;
    fn div(self, rhs: T) -> Matrix<T> {
       core_add_mul_sub_div_with_reals(&self, rhs, (true,true))
    }
}
impl<T : Copy + NumOps> Div<T> for Matrix<T>
{ type Output = Matrix<T>;
    fn div(self, rhs: T) -> Matrix<T> {&self / rhs}}





fn core_add_mul_sub_div_with_matrix<T : Copy + NumOps>(this : &Matrix<T>, rhs : &Matrix<T>, operator : (bool,bool)) -> Matrix<T> {
    Matrix::<T>{ 
        values: {
            let mut r: Vec<T> = Vec::with_capacity((&this).shape.0*(&this).shape.1);
            match operator {
                (false,false)=>for (i,el) in (&this).values.iter().enumerate() {r.push(*el + *rhs.values.get(i).unwrap());},
                (false,true)=>for (i,el) in (&this).values.iter().enumerate() {r.push(*el * *rhs.values.get(i).unwrap())},
                (true,false)=>for (i,el) in (&this).values.iter().enumerate() { r.push(*el - *rhs.values.get(i).unwrap());},
                (true,true)=>for (i,el) in (&rhs).values.iter().enumerate() { r.push(*(&this).values.get(i).expect("Indice not found.") / *el)},
            } 
            r},
        shape:this.shape    
    }
}
impl<T : NumOps + Copy> Add<&Matrix<T>> for &Matrix<T>
{ type Output = Matrix<T>;
    fn add(self, rhs: &Matrix<T>) -> Matrix<T> {
        core_add_mul_sub_div_with_matrix(&self, rhs, (false,false))} 
}
impl<T : NumOps + Copy> Sub<&Matrix<T>> for &Matrix<T>
{ type Output = Matrix<T>;
    fn sub(self, rhs: &Matrix<T>) -> Matrix<T> {
        core_add_mul_sub_div_with_matrix(&self, rhs, (true,false))} 
}
impl<T : NumOps +  Copy> Div<&Matrix<T>> for &Matrix<T> {
    type Output = Matrix<T>;
    fn div(self, rhs: &Matrix<T>) -> Matrix<T>{
            // Need zero division check       
            core_add_mul_sub_div_with_matrix(&self, rhs, (false,false))}  
}



