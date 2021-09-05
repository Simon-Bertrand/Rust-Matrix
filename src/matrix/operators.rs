use std::ops::Mul;
use std::ops::Add;
use std::ops::Sub;
use std::ops::Div;

use crate::matrix::*;


impl Mul<i32> for &Matrix
{
    type Output = Matrix;
    fn mul(self, rhs: i32) -> Matrix {
        match self {
            Matrix::Int(a)=> Matrix::Int(MatrixStruct::<i32>{ values: a.values.iter().map(|v| *v * rhs).collect(), shape:a.shape}),
            Matrix::Float(a)=> Matrix::Float(MatrixStruct::<f64>{ values: a.values.iter().map(|v| *v * (rhs as f64)).collect(), shape:a.shape}),
            Matrix::Bool(a)=> Matrix::Bool(MatrixStruct::<bool>{ values: vec![false], shape:a.shape}),
            Matrix::Null=> Matrix::Bool(MatrixStruct::<bool>{ values: vec![false], shape:(1,1)}),
        }
    }
}



impl Mul<&Matrix> for i32 {
    type Output = Matrix;
    fn mul(self, rhs: &Matrix) -> Matrix{
        rhs*(self)
    }
}
impl Mul<Matrix> for i32 {
    type Output = Matrix;
    fn mul(self, rhs: Matrix) -> Matrix{
        &rhs*(self)
    }
}
impl Mul<i32> for Matrix{
    type Output = Matrix;
    fn mul(self, rhs: i32) -> Matrix{
        rhs*(&self)
    }
}









impl Div<i32> for &Matrix
{
    type Output = Matrix;
    fn div(self, rhs: i32) -> Matrix {
        (1.0/(rhs as f64))*self
    }
}
impl Div<i32> for Matrix
{
    type Output = Matrix;
    fn div(self, rhs: i32) -> Matrix {
        &self/rhs
    }
}



impl Div<&Matrix> for f64
{
    type Output = Matrix;
    fn div(self, rhs: &Matrix) -> Matrix {

        // Need to verify that there is no zero in the Matrix
        match rhs {
            Matrix::Int(a)=>Matrix::Float(MatrixStruct::<f64>{ values: a.values.iter().map(|v| self/(*v as f64)).collect(), shape:a.shape}),
            Matrix::Float(a)=>Matrix::Float(MatrixStruct::<f64>{ values: a.values.iter().map(|v| self / *v).collect(), shape:a.shape}),
            Matrix::Bool(a)=>Matrix::Bool(MatrixStruct::<bool>{ values: vec![false], shape:(1,1)}),
            Matrix::Null=>Matrix::Bool(MatrixStruct::<bool>{ values: vec![false], shape:(1,1)}),
        }
        
    }
}

impl Div<Matrix> for f64 { type Output = Matrix; fn div(self, rhs: Matrix) -> Matrix {self / &rhs} }
impl Div<&Matrix> for i32{ type Output = Matrix; fn div(self, rhs: &Matrix) -> Matrix {self as f64 / rhs} }
impl Div<Matrix> for i32{ type Output = Matrix; fn div(self, rhs: Matrix) -> Matrix {self as f64 / &rhs} }













impl Mul<f64> for &Matrix
{
    type Output = Matrix;
    fn mul(self, rhs: f64) -> Matrix {
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
impl Mul<f64> for Matrix {
    type Output = Matrix;
    fn mul(self, rhs: f64) -> Matrix{
        rhs*(&self)
    }
}






impl Div<f64> for &Matrix
{
    type Output = Matrix;
    fn div(self, rhs: f64) -> Matrix {
        (1.0/(rhs))*self
    }
}
impl Div<f64> for Matrix
{
    type Output = Matrix;
    fn div(self, rhs: f64) -> Matrix {
        (1.0/(rhs))*&self
    }
}


impl Add<i32> for &Matrix
{
    type Output = Matrix;
    fn add(self, rhs: i32) -> Matrix {
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
    fn add(self, rhs: &Matrix) -> Matrix {
        rhs + (self)
    }
}
impl Add<Matrix> for i32 {
    type Output = Matrix;
    fn add(self, rhs: Matrix) -> Matrix {
        &rhs + (self)
    }
}
impl Add<i32> for Matrix {
    type Output = Matrix;
    fn add(self, rhs: i32) -> Matrix {
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
    fn sub(self, rhs: &Matrix) -> Matrix {
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
    fn sub(self, rhs: Matrix) -> Matrix {
        -1*&rhs + self
    }
}




impl Add<f64> for &Matrix
{
    type Output = Matrix;
    fn add(self, rhs: f64) -> Matrix {
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
    fn add(self, rhs: &Matrix) -> Matrix {
        rhs+(self)
    }
}

impl Add<Matrix> for f64 {
    type Output = Matrix;
    fn add(self, rhs: Matrix) -> Matrix {
        &rhs+(self)
    }
}

impl Add<f64> for Matrix {
    type Output = Matrix;
    fn add(self, rhs: f64) -> Matrix {
        rhs+(&self)
    }
}



impl Sub<f64> for &Matrix
{
    type Output = Matrix;
    fn sub(self, rhs: f64) -> Matrix {
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
    fn sub(self, rhs: f64) -> Matrix {
       &self + (-1.0)*rhs
    }
}
impl Sub<Matrix> for f64 {
    type Output = Matrix;
    fn sub(self, rhs: Matrix) -> Matrix{
        (-1.0)*&(&rhs - self)
    }
}




impl Add<Matrix> for Matrix {
    type Output = Matrix;
    fn add(self, rhs: Matrix) -> Matrix {&self + &rhs}
}

impl Add<Matrix> for &Matrix {
    type Output = Matrix;
    fn add(self, rhs: Matrix) -> Matrix {self + &rhs}
}
impl Add<&Matrix> for Matrix {
    type Output = Matrix;
    fn add(self, rhs: &Matrix) -> Matrix {&self + rhs}
}

impl Add<&Matrix> for &Matrix
{
    type Output = Matrix;
    fn add(self, rhs: &Matrix) -> Matrix {
        if self.get_shape() != rhs.get_shape() {
            println!("\nCannot add both Matrix because shapes are different.\n");
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






impl Div<&Matrix> for &Matrix
{
    type Output = Matrix;
    fn div(self, rhs: &Matrix) -> Matrix {
        if self.get_shape() != rhs.get_shape() {
            println!("\nCannot divise element by element on both Matrix because shapes are different.\n");
            return Matrix::Null
        }
        (1.0/rhs) * self
    }
}


impl Div<Matrix> for &Matrix
{
    type Output = Matrix;
    fn div(self, rhs: Matrix) -> Matrix {
        self / &rhs
    }
}
impl Div<&Matrix> for Matrix
{
    type Output = Matrix;
    fn div(self, rhs: &Matrix) -> Matrix {
        &self / rhs
    }
}






impl Sub<Matrix> for Matrix
{
    type Output = Matrix;
    fn sub(self, rhs: Matrix) -> Matrix {
        &self + (-1) * &rhs
    }
}

impl Sub<&Matrix> for &Matrix
{
    type Output = Matrix;
    fn sub(self, rhs: &Matrix) -> Matrix {
        self + (-1) * rhs
    }
}

impl Sub<&Matrix> for Matrix
{
    type Output = Matrix;
    fn sub(self, rhs: &Matrix) -> Matrix {
        &self - rhs
    }
}
impl Sub<Matrix> for &Matrix
{
    type Output = Matrix;
    fn sub(self, rhs: Matrix) -> Matrix {
        self - &rhs
    }
}




impl Mul<&Matrix> for &Matrix
{
    type Output = Matrix;
    fn mul(self, rhs: &Matrix) -> Matrix {
        if self.get_shape() != rhs.get_shape() {
            println!("\nCannot multiplicate both Matrix because shapes are different.\n");
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
impl Mul<&Matrix> for Matrix { type Output = Matrix; fn mul(self, rhs: &Matrix) -> Matrix {&self * rhs} }
impl Mul<Matrix> for Matrix { type Output = Matrix; fn mul(self, rhs: Matrix) -> Matrix {&self * &rhs} }






