#![allow(dead_code)]
#![allow(unused_variables)]

use std::ops::Mul;
use std::ops::Add;
use std::ops::Sub;
use std::ops::Div;
use std::vec;


#[derive(Clone)]
enum Matrix {
    Int(MatrixI),
    Float(MatrixF),
    Bool(MatrixB),
    Null,
}


enum MatVector<'a> {
    Int(Vec<&'a i32>),
    Float(Vec<&'a f64>),
    Bool(Vec<&'a bool>),
    Null
}


#[derive(Clone)]
struct MatrixI {
    values: Vec<i32>,
    shape : (i32,i32),
}
#[derive(Clone)]
struct MatrixF {
    values: Vec<f64>,
    shape : (i32,i32),
}
#[derive(Clone)]
struct MatrixB {
    values: Vec<bool>,
    shape : (i32,i32),
}


impl MatVector<'_> {
    fn len(&self)->usize{
        match self {
            MatVector::Int(a)=>a.len(),
            MatVector::Float(a)=>a.len(),
            MatVector::Bool(a)=>a.len(),
            MatVector::Null=>0,
        }
    }

}

impl MatVector<'_>{
    fn sprod(&self, vect: &'_ MatVector) -> f64 {
        if (self.len()) != vect.len() {
            eprintln!("\nfn sprod(&self, vect: &MatVector) >>> The lengths are not the same : {} != {}. \n", self.len() , vect.len());
            std::process::exit(-1);
        }
        else {
            match &self {
                MatVector::Int(a)=>{
                    match vect {
                        MatVector::Int(b)=>{let mut s: f64 = 0.0; for i in 0..self.len() {s=s+ (*a[i] as f64)*(*b[i] as f64)} s},
                        MatVector::Float(b)=>{let mut s: f64 = 0.0; for i in 0..self.len() {s=s+(*a[i] as f64)*b[i]} s},
                        MatVector::Bool(b)=>{let mut s: f64 = 0.0; for i in 0..self.len() {s=s+0.0} s},
                        MatVector::Null=>0.0,}}
                    MatVector::Float(a)=>{
                        match vect {
                            MatVector::Int(b)=>{let mut s: f64 = 0.0; for i in 0..self.len() {s=s+(*a[i] as f64)*(*b[i] as f64)} s},
                            MatVector::Float(b)=>{let mut s: f64 = 0.0; for i in 0..self.len() {s=s+(*a[i] as f64)*b[i]} s},
                            MatVector::Bool(b)=>{let mut s: f64 = 0.0; for i in 0..self.len() {s=s+0.0} s},
                            MatVector::Null=>0.0,}},
                    MatVector::Bool(a)=>{
                        match vect {
                            MatVector::Int(b)=>{let mut s: f64 = 0.0; for i in 0..self.len() {s=s+0.0} s},
                            MatVector::Float(b)=>{let mut s: f64 = 0.0; for i in 0..self.len() {s=s+0.0} s},
                            MatVector::Bool(b)=>{let mut s: f64 = 0.0; for i in 0..self.len() {s=s+0.0} s},
                            MatVector::Null=>0.0,}},
                    MatVector::Null=>0.0,
            }
        }
        
    }
    
}

impl std::fmt::Display for Matrix {
    fn fmt(&self, _f: &mut std::fmt::Formatter) -> std::fmt::Result {
        fn show<T: std::fmt::Display>(vect: &Vec<T>, shape:&(i32,i32)) {
            let mut i=0;
            println!("");
            print!("| ");
            for val in vect.iter() {      
                if i == shape.1 {
                    println!("|");
                    print!("| ");
                    i=0
                }
                print!("{} ", val);
                i+=1;
            }
            println!("|");
        }
        match &self {
            Matrix::Int(a)=>Ok({show(&a.values,&a.shape); println!("-Int ({},{})-", &a.shape.0, &a.shape.1)}),
            Matrix::Float(a)=>Ok({show(&a.values,&a.shape);println!("-Float ({},{})-", &a.shape.0, &a.shape.1)}),
            Matrix::Bool(a)=>Ok({show(&a.values, &a.shape);println!("-Bool ({},{})-", &a.shape.0, &a.shape.1)}),
            Matrix::Null=>Err(std::fmt::Error),
        }
    }
}




impl std::fmt::Display for MatVector<'_> {
    fn fmt(&self, _f: &mut std::fmt::Formatter) -> std::fmt::Result{
        fn show_vector<T: std::fmt::Display>(vect: &Vec<&T>) {
            print!("[ ");
            for v in vect {
                print!("{} ",v);
            }
            print!("]\n");
        }
        match self {
            MatVector::Int(a)=>Ok(show_vector(a)),
            MatVector::Float(a)=>Ok(show_vector(a)),
            MatVector::Bool(a)=>Ok(show_vector(a)),
            MatVector::Null=>Err(std::fmt::Error),
        }
    }
}



impl Matrix {
    fn row(&self, i:i32) -> MatVector {
        let k = i as usize;
        if i<0 || i>= self.get_shape().0 {
            eprintln!("\nfn row(&self, i:i32) >>> The row indice is out of range : {} is not in [{}-{}]. \n", i, 0, self.get_shape().0-1);
            std::process::exit(-1);
        }
        match &self {
            Matrix::Int(a)=>MatVector::Int(vec![&1,&2,&0]),
            Matrix::Float(a)=>MatVector::Float(vec![&1.0,&2.0,&0.0]),
            Matrix::Bool(a)=> MatVector::Bool(vec![&true,&true,&true]),
            Matrix::Null=>MatVector::Null,
        }
    }

    fn col(&self, j:i32) -> MatVector {
        let k = j as usize;
        if j<0 || j>= self.get_shape().1 {
            eprintln!("\nfn col(&self, i:i32) >>> The col indice is out of range : {} is not in [{}-{}]. \n", k, 0, self.get_shape().0-1);
            std::process::exit(-1);
        }
        match &self {
            Matrix::Int(a)=>MatVector::Int(vec![&1,&2,&0]),
            Matrix::Float(a)=>MatVector::Float(vec![&1.0,&2.0,&0.0]),
            Matrix::Bool(a)=>MatVector::Bool(vec![&false,&true,&true]),
            Matrix::Null=>MatVector::Null,}
    }

}


    










impl Matrix {
    fn convert_to(self, type_ : &str) -> Matrix {
        if type_ == "i32" {
            return match self {
                Matrix::Int(_a)=>Matrix::Bool(MatrixB{ values: vec![false], shape:(1,1)}),
                Matrix::Float(a)=>Matrix::Int(MatrixI{values : { let mut r:Vec<i32>= Vec::new(); for i in 0..a.values.len() {r.push(a.values[i] as i32)} r}, shape: a.shape}),
                Matrix::Bool(_a)=>Matrix::Bool(MatrixB{ values: vec![false], shape:(1,1)}),
                Matrix::Null=>Matrix::Bool(MatrixB{ values: vec![false], shape:(1,1)}),
            }
        }
        else if type_ == "f64" {
            return match self {
                Matrix::Int(a)=>Matrix::Float(MatrixF{values : { let mut r:Vec<f64> = Vec::new(); for i in 0..a.values.len() {r.push(a.values[i] as f64)} r}, shape: a.shape}),
                Matrix::Float(_a)=>Matrix::Bool(MatrixB{ values: vec![false], shape:(1,1)}),
                Matrix::Bool(_a)=>Matrix::Bool(MatrixB{ values: vec![false], shape:(1,1)}),
                Matrix::Null=>Matrix::Bool(MatrixB{ values: vec![false], shape:(1,1)}),
            }
        }
        else if type_ == "bool" {
            return match self {
                Matrix::Int(_a)=>Matrix::Bool(MatrixB{ values: vec![false], shape:(1,1)}),
                Matrix::Float(_a)=>Matrix::Bool(MatrixB{ values: vec![false], shape:(1,1)}),
                Matrix::Bool(_a)=>Matrix::Bool(MatrixB{ values: vec![false], shape:(1,1)}),
                Matrix::Null=>Matrix::Bool(MatrixB{ values: vec![false], shape:(1,1)}),
            }
        }
        else {
            return Matrix::Bool(MatrixB{ values: vec![false], shape:(1,1)})
        }
    }

    fn get_shape(&self) -> (i32,i32) {
        match self {
            Matrix::Int(a)=>a.shape,
            Matrix::Float(a)=>a.shape,
            Matrix::Bool(a)=>a.shape,
            Matrix::Null=>(1,1)
        }
    }
}









impl Mul<i32> for &Matrix
{
    type Output = Matrix;
    fn mul(self, rhs: i32) -> Matrix {
        match self {
            Matrix::Int(a)=> Matrix::Int(MatrixI{ values: a.values.iter().map(|v| *v * rhs).collect(), shape:a.shape}),
            Matrix::Float(a)=> Matrix::Float(MatrixF{ values: a.values.iter().map(|v| *v * (rhs as f64)).collect(), shape:a.shape}),
            Matrix::Bool(a)=> Matrix::Bool(MatrixB{ values: vec![false], shape:a.shape}),
            Matrix::Null=> Matrix::Bool(MatrixB{ values: vec![false], shape:(1,1)}),
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
            Matrix::Int(a)=>Matrix::Float(MatrixF{ values: a.values.iter().map(|v| self/(*v as f64)).collect(), shape:a.shape}),
            Matrix::Float(a)=>Matrix::Float(MatrixF{ values: a.values.iter().map(|v| self / *v).collect(), shape:a.shape}),
            Matrix::Bool(a)=>Matrix::Bool(MatrixB{ values: vec![false], shape:(1,1)}),
            Matrix::Null=>Matrix::Bool(MatrixB{ values: vec![false], shape:(1,1)}),
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
            Matrix::Int(a)=> Matrix::Float(MatrixF{ values: a.values.iter().map(|v| (*v as f64) * rhs).collect(), shape:a.shape}),
            Matrix::Float(a)=> Matrix::Float(MatrixF{ values: a.values.iter().map(|v| *v * rhs).collect(), shape:a.shape}),
            Matrix::Bool(a)=>Matrix::Bool(MatrixB{ values: vec![false], shape:a.shape}),
            Matrix::Null=>Matrix::Bool(MatrixB{ values: vec![false], shape:(1,1)}),
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
            Matrix::Int(a)=> Matrix::Int(MatrixI{ values: a.values.iter().map(|v| *v + rhs).collect(), shape:a.shape}),
            Matrix::Float(a)=> Matrix::Float(MatrixF{ values: a.values.iter().map(|v| *v + (rhs as f64)).collect(), shape:a.shape}),
            Matrix::Bool(a)=> Matrix::Bool(MatrixB{ values: vec![false], shape:a.shape}),
            Matrix::Null=> Matrix::Bool(MatrixB{ values: vec![false], shape:(1,1)}),
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
            Matrix::Int(a)=> Matrix::Float(MatrixF{ values: a.values.iter().map(|v| (*v as f64) + rhs).collect(), shape:a.shape}),
            Matrix::Float(a)=> Matrix::Float(MatrixF{ values: a.values.iter().map(|v| *v + rhs).collect(), shape:a.shape}),
            Matrix::Bool(a)=> Matrix::Bool(MatrixB{ values: vec![false], shape:a.shape}),
            Matrix::Null=> Matrix::Bool(MatrixB{ values: vec![false], shape:(1,1)}),
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
                        MatrixI{ values: { 
                        let mut r: Vec<i32> = Vec::new(); for i in 0..a.values.len() {r.push(a.values[i] + b.values[i])} r
                    }, shape:a.shape}),
                    Matrix::Float(b)=> Matrix::Float(
                        
                        MatrixF{ values:{
                        let mut r: Vec<f64> = Vec::new(); for i in 0..a.values.len() {r.push((a.values[i] as f64) + b.values[i])} r
                    }, shape:a.shape}),
                    Matrix::Bool(_b)=> Matrix::Null,
                    Matrix::Null=> Matrix::Null,
                }
            },
            Matrix::Float(a)=>
            {
                match rhs {
                    Matrix::Int(b)=> Matrix::Float(MatrixF{ values: {
                        let mut r: Vec<f64> = Vec::new(); for i in 0..a.values.len() {r.push(a.values[i] + (b.values[i] as f64))} r
                    }, shape:a.shape}),
                    Matrix::Float(b)=> Matrix::Float(MatrixF{ values:{
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
                        MatrixI{ values: { 
                        let mut r: Vec<i32> = Vec::new(); for i in 0..a.values.len() {r.push(a.values[i] * b.values[i])} r
                    }, shape:a.shape}),
                    Matrix::Float(b)=> Matrix::Float(
                        
                        MatrixF{ values:{
                        let mut r: Vec<f64> = Vec::new(); for i in 0..a.values.len() {r.push((a.values[i] as f64) * b.values[i])} r
                    }, shape:a.shape}),
                    Matrix::Bool(_b)=> Matrix::Null,
                    Matrix::Null=> Matrix::Null,
                }
            },
            Matrix::Float(a)=>
            {
                match rhs {
                    Matrix::Int(b)=> Matrix::Float(MatrixF{ values: {
                        let mut r: Vec<f64> = Vec::new(); for i in 0..a.values.len() {r.push(a.values[i] * (b.values[i] as f64))} r
                    }, shape:a.shape}),
                    Matrix::Float(b)=> Matrix::Float(MatrixF{ values:{
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








// CONSTRUCTORS



impl Matrix {
    fn new(shape1 : i32, shape2 : i32, dtype : &str) -> Matrix { 

        if dtype == "i32" {
            Matrix::Int( MatrixI{values : vec![0; (shape1*shape2) as usize ], shape:(shape1,shape2)})
        }
        else if dtype == "f64" {
            Matrix::Float( MatrixF{values : vec![0.0; (shape1*shape2) as usize ], shape:(shape1,shape2)})

        }
        else if dtype == "bool" {
            Matrix::Bool( MatrixB{values : vec![false; (shape1*shape2) as usize ], shape:(shape1,shape2)})
        }
        else {
            Matrix::Null
        }
        
    }
    fn ones(shape1 : i32, shape2 : i32) -> Matrix { 
        Matrix::new(shape1, shape2, "i32") + 1
    }
    fn fill_int(fill_value:i32, shape1 : i32, shape2 : i32) -> Matrix { 
        Matrix::new(shape1, shape2, "i32") + fill_value
    }
    fn fill_float(fill_value:f64, shape1 : i32, shape2 : i32) -> Matrix { 
        Matrix::new(shape1, shape2, "f64") + fill_value
    }


    fn eye(shape : i32, dtype : &str) -> Matrix { 
        if dtype == "i32" {
            Matrix::Int( MatrixI{values : {let mut value : Vec<i32> = Vec::new();
                for i in 0..shape{
                    for j in 0..shape{
                        if i==j {
                            value.push(1);
                        }
                        else {
                            value.push(0);
                        }     
                    }
                }
            value}, shape:(shape,shape)})
        }
        else if dtype == "f64" {
            Matrix::Float( MatrixF{values : {let mut value : Vec<f64> = Vec::new();
                for i in 0..shape{
                    for j in 0..shape{
                        if i==j {
                            value.push(1.0);
                        }
                        else {
                            value.push(0.0);
                        }     
                    }
                }
            value}, shape:(shape,shape)})
        }
        else if dtype == "bool" {
            Matrix::Bool( MatrixB{values : {let mut value : Vec<bool> = Vec::new();
                for i in 0..shape{
                    for j in 0..shape{
                        if i==j {
                            value.push(true);
                        }
                        else {
                            value.push(false);
                        }     
                    }
                }
            value}, shape:(shape,shape)})
        }
        else {
            Matrix::Null
        }
        
        
    }
}



fn main() {
    let mat = &Matrix::ones(3,3);
    let mat2 = &Matrix::eye(3,"i32");

    println!("{}", mat.col(1).sprod(&mat2.col(1)));
    println!("{}{}", mat.col(1),mat2.col(1))
}
