use std::ops::Mul;
use std::ops::Add;
use std::ops::Sub;
use std::ops::Div;
use std::cmp::PartialEq;
use num_rational::Ratio;
use num::Num;
use crate::matrix::*;
use crate::matrix::constructors::Constructors;



impl<T : Num> PartialEq for Matrix<T> {
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
impl<T : num_integer::Integer + Clone> PartialEq<Matrix<Ratio<T>>> for Matrix<T> {fn eq(&self, other: &Matrix<Ratio<T>>) -> bool {self == other}}


impl<T : Mul + Copy> Mul<T> for &Matrix<T>
{
    type Output = Matrix<T::Output>;
    fn mul(self, rhs: T) -> Matrix<T::Output> {
        Matrix::<T::Output>{ 
            values: {
                let mut r: Vec<T::Output> = Vec::with_capacity(((&self).shape.0*(&self).shape.1) as usize); 
                for el in (&self).values.iter() {
                    r.push(*el * rhs)
                }
                r},
            shape:self.shape}    
        }
}
impl<T : Mul + Copy> Mul<T> for Matrix<T>
{
    type Output = Matrix<T::Output>;
    fn mul(self, rhs: T) -> Matrix<T::Output> {&self * rhs}
}



macro_rules! sub_impl {
    ($($t:ty)*) => ($(
        impl Mul<&Matrix<$t>> for $t {
            type Output = Matrix<$t>;
            fn mul(self, rhs: &Matrix<$t>) -> Matrix<$t>{
                rhs*(self)
            }
        }
        impl Mul<Matrix<$t>> for $t {
            type Output = Matrix<$t>;
            fn mul(self, rhs: Matrix<$t>) -> Matrix<$t>{
                &rhs*(self)
            }
        }
    )*)
}

sub_impl! { usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 f32 f64 }




impl<T : Add + Copy> Add<&Matrix<T>> for &Matrix<T>
{
    type Output = Matrix<T::Output>;
    fn add(self, rhs: &Matrix<T>) -> Matrix<T::Output> {
        Matrix::<T::Output>{ 
            values: {
                let mut r: Vec<T::Output> = Vec::with_capacity(((&self).shape.0*(&self).shape.1) as usize); 
                for (i,el) in (&self).values.iter().enumerate() {
                    r.push(*el + *rhs.values.get(i).unwrap());
                }
                r},
            shape:self.shape}    
        }
}

impl<T : Add + Copy> Add<Matrix<T>> for &Matrix<T> { type Output = Matrix<T::Output>; fn add(self, rhs: Matrix<T>) -> Matrix<T::Output>{ self + &rhs }}
impl<T : Add + Copy> Add<&Matrix<T>> for Matrix<T> { type Output = Matrix<T::Output>; fn add(self, rhs: &Matrix<T>) -> Matrix<T::Output>{ &self + rhs }}
impl<T : Add + Copy> Add<Matrix<T>> for Matrix<T> { type Output = Matrix<T::Output>; fn add(self, rhs: Matrix<T>) -> Matrix<T::Output>{ &self + &rhs }}


impl<T : Add + Copy> Add<T> for &Matrix<T>
{
    type Output = Matrix<T::Output>;
    fn add(self, rhs: T) -> Matrix<T::Output> {
        Matrix::<T::Output>{ 
            values: {
                let mut r: Vec<T::Output> = Vec::with_capacity(((&self).shape.0*(&self).shape.1) as usize); 
                for el in (&self).values.iter() {
                    r.push(*el + rhs)
                }
                r},
            shape:self.shape}    
        }
}
impl<T : Add + Copy> Add<T> for Matrix<T>
{
    type Output = Matrix<T::Output>;
    fn add(self, rhs: T) -> Matrix<T::Output> {&self + rhs}
}



macro_rules! sub_impl {
    ($($t:ty)*) => ($(
        impl Add<&Matrix<$t>> for $t {
            type Output = Matrix<$t>;
            fn add(self, rhs: &Matrix<$t>) -> Matrix<$t>{
                rhs + (self)
            }
        }
        impl Add<Matrix<$t>> for $t {
            type Output = Matrix<$t>;
            fn add(self, rhs: Matrix<$t>) -> Matrix<$t>{
                &rhs + (self)
            }
        }
    )*)
}

sub_impl! { usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 f32 f64 }





impl<T : Sub + Copy> Sub<&Matrix<T>> for &Matrix<T>
{
    type Output = Matrix<T::Output>;
    fn sub(self, rhs: &Matrix<T>) -> Matrix<T::Output> {
        Matrix::<T::Output>{ 
            values: {
                let mut r: Vec<T::Output> = Vec::with_capacity(((&self).shape.0*(&self).shape.1) as usize); 
                for (i,el) in (&self).values.iter().enumerate() {
                    r.push(*el - *rhs.values.get(i).unwrap());
                }
                r},
            shape:self.shape}    
        }
}

impl<T : Sub + Copy> Sub<Matrix<T>> for &Matrix<T> { type Output = Matrix<T::Output>; fn sub(self, rhs: Matrix<T>) -> Matrix<T::Output>{ self - &rhs }}
impl<T : Sub + Copy> Sub<&Matrix<T>> for Matrix<T> { type Output = Matrix<T::Output>; fn sub(self, rhs: &Matrix<T>) -> Matrix<T::Output>{ &self - rhs }}
impl<T : Sub + Copy> Sub<Matrix<T>> for Matrix<T> { type Output = Matrix<T::Output>; fn sub(self, rhs: Matrix<T>) -> Matrix<T::Output>{ &self - &rhs }}

impl<T : Sub + Copy> Sub<T> for &Matrix<T>
{
    type Output = Matrix<T::Output>;
    fn sub(self, rhs: T) -> Matrix<T::Output> {
        Matrix::<T::Output>{ 
            values: {
                let mut r: Vec<T::Output> = Vec::with_capacity(((&self).shape.0*(&self).shape.1) as usize); 
                for el in (&self).values.iter() {
                    r.push(*el - rhs)
                }
                r},
            shape:self.shape}    
        }
}
impl<T : Sub + Copy> Sub<T> for Matrix<T>
{
    type Output = Matrix<T::Output>;
    fn sub(self, rhs: T) -> Matrix<T::Output> {&self - rhs}
}

macro_rules! sub_impl {
    ($($t:ty)*) => ($(
        impl Sub<&Matrix<$t>> for $t {
            type Output = Matrix<$t>;
            fn sub(self, rhs: &Matrix<$t>) -> Matrix<$t>{
                rhs - (self)
            }
        }
        impl Sub<Matrix<$t>> for $t {
            type Output = Matrix<$t>;
            fn sub(self, rhs: Matrix<$t>) -> Matrix<$t>{
                &rhs - (self)
            }
        }
    )*)
}

sub_impl! { usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 f32 f64 }










impl<T : Div + Copy> Div<&Matrix<T>> for &Matrix<T> {
    type Output = Matrix<T::Output>;
    fn div(self, rhs: &Matrix<T>) -> Matrix<T::Output>{
            // Need zero division check       
            Matrix::<T::Output>{ 
                values: {
                    let mut r: Vec<T::Output> = Vec::with_capacity(((&rhs).shape.0*(*rhs).shape.1) as usize); 
                    for (i,el) in (&rhs).values.iter().enumerate() {
                        r.push(*(&self).values.get(i).expect("Indice not found.") / *el)
                    }
                    r},
                shape:(&rhs).shape
            }    
    }
}


impl<T : Div + Copy> Div<Matrix<T>> for &Matrix<T> { type Output = Matrix<T::Output>; fn div(self, rhs: Matrix<T>) -> Matrix<T::Output>{ self / &rhs }}
impl<T : Div + Copy> Div<&Matrix<T>> for Matrix<T> { type Output = Matrix<T::Output>; fn div(self, rhs: &Matrix<T>) -> Matrix<T::Output>{ &self / rhs }}
impl<T : Div + Copy> Div<Matrix<T>> for Matrix<T> { type Output = Matrix<T::Output>; fn div(self, rhs: Matrix<T>) -> Matrix<T::Output>{ &self / &rhs }}


impl<T : Div + Copy> Div<T> for &Matrix<T>
{
    type Output = Matrix<T::Output>;
    fn div(self, rhs: T) -> Matrix<T::Output> {
        Matrix::<T::Output>{ 
            values: {
                let mut r: Vec<T::Output> = Vec::with_capacity(((&self).shape.0*(&self).shape.1) as usize); 
                for el in (&self).values.iter() {
                    r.push(*el / rhs)
                }
                r},
            shape:self.shape}    
        }
}


impl<T : Div + Copy> Div<T> for Matrix<T>
{
    type Output = Matrix<T::Output>;
    fn div(self, rhs: T) -> Matrix<T::Output> {&self / rhs}
}

macro_rules! sub_impl {
    ($($t:ty)*) => ($(
        impl Div<&Matrix<$t>> for $t {
            type Output = Matrix<$t>;
            fn div(self, rhs: &Matrix<$t>) -> Matrix<$t>{
                &Matrix::fill(5,5,self) / rhs
            }
        }
        impl Div<Matrix<$t>> for $t {
            type Output = Matrix<$t>;
            fn div(self, rhs: Matrix<$t>) -> Matrix<$t>{
                &Matrix::fill(5,5,self) / &rhs
            }
        }
    )*)
}

sub_impl! { usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 f32 f64 }


