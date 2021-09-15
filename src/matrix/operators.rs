use std::ops::Mul;
use std::ops::Add;
use std::ops::Sub;
use std::ops::Div;



use crate::matrix::*;
use crate::matrix::constructors::Constructors;


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




/*

T::Output

impl<T : Mul> Mul<Matrix<T>> for T {
    type Output = Matrix<T>;
    fn mul(&self, rhs: Matrix<T>) -> Matrix<T>{
        &rhs*(*self)
    }
}

impl<T : Mul> Mul<T> for Matrix<T>{
    type Output = Matrix<T>;
    fn mul(&self,rhs: T) -> Matrix<T>{
        (*self)*rhs
    }
}

*/

/*

impl<T : Div> Div<&Matrix<T>> for &Matrix<T> {
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


impl<T> Div<T> for &Matrix<T> {
    type Output = Matrix<T>;
    fn div(self, rhs: T) {
        (<Matrix<T> as Constructors<T>>::new(n,m) + 1)/self
    }
}
impl<T : Div> Div<Matrix<T>> for &Matrix<T>
{
    type Output = Matrix<T>;
    fn div(self, rhs: Matrix<T>) -> Matrix<T>{
        self / &rhs
    }
}
impl<T : Div> Div<&Matrix<T>> for Matrix<T>
{
    type Output = Matrix<T>;
    fn div(self, rhs: &Matrix<T>) -> Matrix<T>{
        &self / rhs
    }
}


/*
impl<T : Div> Div<T> for &Matrix<T>
{
    type Output = Matrix<T>;
    fn div(self, rhs: &T) -> Matrix<T>{
        1/(*rhs))*self
    }
}
impl<T : Div> Div<T> for Matrix<T>
{
    type Output = Matrix<T>;
    fn div(self, rhs: T) -> Matrix<T>{
        &self/rhs
    }
}
*/


*/


//impl<T : Div> Div<Matrix<T>> for T { type Output = Matrix<T>; fn div(self, rhs: Matrix<T>) -> Matrix<T>{self / &rhs} }

/*


impl Div<f64> for &Matrix
{
    type Output = Matrix;
    fn div(self, rhs: f64) -> Matrix<T>{
        (1.0/(rhs))*self
    }
}
impl Div<f64> for Matrix
{
    type Output = Matrix;
    fn div(self, rhs: f64) -> Matrix<T>{
        (1.0/(rhs))*&self
    }
}











impl Mul<f64> for &Matrix
{
    type Output = Matrix;
    fn mul(self, rhs: f64) -> Matrix<T>{
        match self {
            Matrix::Int(a)=> Matrix::Float(MatrixStruct::<f64>{ values: a.values.iter().map(|v| (*v as f64) * rhs).collect(), shape:a.shape}),
            Matrix::Float(a)=> Matrix::Float(MatrixStruct::<f64>{ values: a.values.iter().map(|v| *v * rhs).collect(), shape:a.shape}),
            Matrix::Bool(a)=>Matrix::Bool(MatrixStruct::<bool>{ values: vec![false], shape:a.shape}),
            Matrix::Null=>Matrix::Bool(MatrixStruct::<bool>{ values: vec![false], shape:(1,1)}),
        }
    }
}
impl Mul<&Matrix> for f64 {
    type Output = Matrix;
    fn mul(self, rhs: &Matrix) -> Matrix{
        rhs*(self)
    }
}

impl Mul<Matrix> for f64 {
    type Output = Matrix;
    fn mul(self, rhs: Matrix) -> Matrix{
        &rhs*(self)
    }
}
impl Mul<f64> for Matrix<T>{
    type Output = Matrix;
    fn mul(self, rhs: f64) -> Matrix{
        rhs*(&self)
    }
}







impl Add<i32> for &Matrix
{
    type Output = Matrix;
    fn add(self, rhs: i32) -> Matrix<T>{
        match self {
            Matrix::Int(a)=> Matrix::Int(MatrixStruct::<i32>{ values: a.values.iter().map(|v| *v + rhs).collect(), shape:a.shape}),
            Matrix::Float(a)=> Matrix::Float(MatrixStruct::<f64>{ values: a.values.iter().map(|v| *v + (rhs as f64)).collect(), shape:a.shape}),
            Matrix::Bool(a)=> Matrix::Bool(MatrixStruct::<bool>{ values: vec![false], shape:a.shape}),
            Matrix::Null=> Matrix::Bool(MatrixStruct::<bool>{ values: vec![false], shape:(1,1)}),
        }
    }
}
impl Add<&Matrix> for i32 {
    type Output = Matrix;
    fn add(self, rhs: &Matrix) -> Matrix<T>{
        rhs + (self)
    }
}
impl Add<Matrix> for i32 {
    type Output = Matrix;
    fn add(self, rhs: Matrix) -> Matrix<T>{
        &rhs + (self)
    }
}
impl Add<i32> for Matrix<T>{
    type Output = Matrix;
    fn add(self, rhs: i32) -> Matrix<T>{
        rhs + &self
    }
}

impl Sub<i32> for &Matrix
{
    type Output = Matrix;
    fn sub(self, rhs: i32) -> Matrix{
       self + (-1)*rhs
    }
}
impl Sub<&Matrix> for i32 {
    type Output = Matrix;
    fn sub(self, rhs: &Matrix) -> Matrix<T>{
        (-1)*&(rhs - self)
    }
}
impl Sub<i32> for Matrix
{
    type Output = Matrix;
    fn sub(self, rhs: i32) -> Matrix{
       &self + (-1)*rhs
    }
}
impl Sub<Matrix> for i32 {
    type Output = Matrix;
    fn sub(self, rhs: Matrix) -> Matrix<T>{
        -1*&rhs + self
    }
}




impl Add<f64> for &Matrix
{
    type Output = Matrix;
    fn add(self, rhs: f64) -> Matrix<T>{
        match self {
            Matrix::Int(a)=> Matrix::Float(MatrixStruct::<f64>{ values: a.values.iter().map(|v| (*v as f64) + rhs).collect(), shape:a.shape}),
            Matrix::Float(a)=> Matrix::Float(MatrixStruct::<f64>{ values: a.values.iter().map(|v| *v + rhs).collect(), shape:a.shape}),
            Matrix::Bool(a)=> Matrix::Bool(MatrixStruct::<bool>{ values: vec![false], shape:a.shape}),
            Matrix::Null=> Matrix::Bool(MatrixStruct::<bool>{ values: vec![false], shape:(1,1)}),
        }
    }
}
impl Add<&Matrix> for f64 {
    type Output = Matrix;
    fn add(self, rhs: &Matrix) -> Matrix<T>{
        rhs+(self)
    }
}

impl Add<Matrix> for f64 {
    type Output = Matrix;
    fn add(self, rhs: Matrix) -> Matrix<T>{
        &rhs+(self)
    }
}

impl Add<f64> for Matrix<T>{
    type Output = Matrix;
    fn add(self, rhs: f64) -> Matrix<T>{
        rhs+(&self)
    }
}



impl Sub<f64> for &Matrix
{
    type Output = Matrix;
    fn sub(self, rhs: f64) -> Matrix<T>{
       self + (-1.0)*rhs
    }
}
impl Sub<&Matrix> for f64 {
    type Output = Matrix;
    fn sub(self, rhs: &Matrix) -> Matrix{
        (-1.0)*&(rhs - self)
    }
}

impl Sub<f64> for Matrix
{
    type Output = Matrix;
    fn sub(self, rhs: f64) -> Matrix<T>{
       &self + (-1.0)*rhs
    }
}
impl Sub<Matrix> for f64 {
    type Output = Matrix;
    fn sub(self, rhs: Matrix) -> Matrix{
        (-1.0)*&(&rhs - self)
    }
}




impl Add<Matrix> for Matrix<T>{
    type Output = Matrix;
    fn add(self, rhs: Matrix) -> Matrix<T>{&self + &rhs}
}

impl Add<Matrix> for &Matrix<T>{
    type Output = Matrix;
    fn add(self, rhs: Matrix) -> Matrix<T>{self + &rhs}
}
impl Add<&Matrix> for Matrix<T>{
    type Output = Matrix;
    fn add(self, rhs: &Matrix) -> Matrix<T>{&self + rhs}
}

impl Add<&Matrix> for &Matrix
{
    type Output = Matrix;
    fn add(self, rhs: &Matrix) -> Matrix<T>{
        if self.get_shape() != rhs.get_shape() {
            println!("\nCannot add both Matrix<T>because shapes are different.\n");
            return Matrix::Null
        }

        match self {
            Matrix::Int(a)=>
            {
                match rhs {
                    Matrix::Int(b)=> Matrix::Int( 
                        MatrixStruct::<i32>{ values: { 
                        let mut r: Vec<i32> = Vec::new(); for i in 0..a.values.len() {r.push(a.values[i] + b.values[i])} r
                    }, shape:a.shape}),
                    Matrix::Float(b)=> Matrix::Float(
                        
                        MatrixStruct::<f64>{ values:{
                        let mut r: Vec<f64> = Vec::new(); for i in 0..a.values.len() {r.push((a.values[i] as f64) + b.values[i])} r
                    }, shape:a.shape}),
                    Matrix::Bool(_b)=> Matrix::Null,
                    Matrix::Null=> Matrix::Null,
                }
            },
            Matrix::Float(a)=>
            {
                match rhs {
                    Matrix::Int(b)=> Matrix::Float(MatrixStruct::<f64>{ values: {
                        let mut r: Vec<f64> = Vec::new(); for i in 0..a.values.len() {r.push(a.values[i] + (b.values[i] as f64))} r
                    }, shape:a.shape}),
                    Matrix::Float(b)=> Matrix::Float(MatrixStruct::<f64>{ values:{
                        let mut r: Vec<f64> = Vec::new(); for i in 0..a.values.len() {r.push(a.values[i] + b.values[i])} r
                    }, shape:a.shape}),
                    Matrix::Bool(_b)=>  Matrix::Null,
                    Matrix::Null=> Matrix::Null,
                }

            },
            Matrix::Bool(_a)=>
            {
                match rhs {
                    Matrix::Int(_b)=> Matrix::Null,
                    Matrix::Float(_b)=> Matrix::Null,
                    Matrix::Bool(_b)=> Matrix::Null,
                    Matrix::Null=> Matrix::Null,
                }

            },
            Matrix::Null=>
            {
                match rhs {
                    Matrix::Int(_b)=> Matrix::Null,
                    Matrix::Float(_b)=> Matrix::Null,
                    Matrix::Bool(_b)=> Matrix::Null,
                    Matrix::Null=> Matrix::Null,
                }

            },
        }
    }
}








impl Sub<Matrix> for Matrix
{
    type Output = Matrix;
    fn sub(self, rhs: Matrix) -> Matrix<T>{
        &self + (-1) * &rhs
    }
}

impl Sub<&Matrix> for &Matrix
{
    type Output = Matrix;
    fn sub(self, rhs: &Matrix) -> Matrix<T>{
        self + (-1) * rhs
    }
}

impl Sub<&Matrix> for Matrix
{
    type Output = Matrix;
    fn sub(self, rhs: &Matrix) -> Matrix<T>{
        &self - rhs
    }
}
impl Sub<Matrix> for &Matrix
{
    type Output = Matrix;
    fn sub(self, rhs: Matrix) -> Matrix<T>{
        self - &rhs
    }
}




impl Mul<&Matrix> for &Matrix
{
    type Output = Matrix;
    fn mul(self, rhs: &Matrix) -> Matrix<T>{
        if self.get_shape() != rhs.get_shape() {
            println!("\nCannot multiplicate both Matrix<T>because shapes are different.\n");
            return Matrix::Null
        }

        match self {
            Matrix::Int(a)=>
            {
                match rhs {
                    Matrix::Int(b)=> Matrix::Int( 
                        MatrixStruct::<i32>{ values: { 
                        let mut r: Vec<i32> = Vec::new(); for i in 0..a.values.len() {r.push(a.values[i] * b.values[i])} r
                    }, shape:a.shape}),
                    Matrix::Float(b)=> Matrix::Float(
                        
                        MatrixStruct::<f64>{ values:{
                        let mut r: Vec<f64> = Vec::new(); for i in 0..a.values.len() {r.push((a.values[i] as f64) * b.values[i])} r
                    }, shape:a.shape}),
                    Matrix::Bool(_b)=> Matrix::Null,
                    Matrix::Null=> Matrix::Null,
                }
            },
            Matrix::Float(a)=>
            {
                match rhs {
                    Matrix::Int(b)=> Matrix::Float(MatrixStruct::<f64>{ values: {
                        let mut r: Vec<f64> = Vec::new(); for i in 0..a.values.len() {r.push(a.values[i] * (b.values[i] as f64))} r
                    }, shape:a.shape}),
                    Matrix::Float(b)=> Matrix::Float(MatrixStruct::<f64>{ values:{
                        let mut r: Vec<f64> = Vec::new(); for i in 0..a.values.len() {r.push(a.values[i] * b.values[i])} r
                    }, shape:a.shape}),
                    Matrix::Bool(_b)=>  Matrix::Null,
                    Matrix::Null=> Matrix::Null,
                }

            },
            Matrix::Bool(_a)=>
            {
                match rhs {
                    Matrix::Int(_b)=> Matrix::Null,
                    Matrix::Float(_b)=> Matrix::Null,
                    Matrix::Bool(_b)=> Matrix::Null,
                    Matrix::Null=> Matrix::Null,
                }

            },
            Matrix::Null=>
            {
                match rhs {
                    Matrix::Int(_b)=> Matrix::Null,
                    Matrix::Float(_b)=> Matrix::Null,
                    Matrix::Bool(_b)=> Matrix::Null,
                    Matrix::Null=> Matrix::Null,
                }

            },
        }
    }
}
impl Mul<&Matrix> for Matrix<T>{ type Output = Matrix; fn mul(self, rhs: &Matrix) -> Matrix<T>{&self * rhs} }
impl Mul<Matrix> for Matrix<T>{ type Output = Matrix; fn mul(self, rhs: Matrix) -> Matrix<T>{&self * &rhs} }



*/


