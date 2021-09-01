#![allow(dead_code)]
#![allow(unused_variables)]

use std::ops::Mul;
use std::ops::Add;
use std::ops::Sub;
use std::ops::Div;


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



impl std::fmt::Display for Matrix {
    fn fmt(&self, _f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self {
            Matrix::Int(a)=>Ok({show(&a.values,&a.shape); println!("-Int ({},{})-", &a.shape.0, &a.shape.1)}),
            Matrix::Float(a)=>Ok({show(&a.values,&a.shape);println!("-Float ({},{})-", &a.shape.0, &a.shape.1)}),
            Matrix::Bool(a)=>Ok({show(&a.values, &a.shape);println!("-Bool ({},{})-", &a.shape.0, &a.shape.1)}),
            Matrix::Null=>Err(std::fmt::Error),
        }
    }
}






fn show_vector<T: std::fmt::Display>(vect: &Vec<&T>) {
    print!("[ ");
    for v in &*vect {
        print!("{} ",v);
    }
    print!("]\n");
}

impl std::fmt::Display for MatVector<'_> {
    fn fmt(&self, _f: &mut std::fmt::Formatter) -> std::fmt::Result{
        match &self {
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
            Matrix::Int(a)=>MatVector::Int(a.values[(self.get_shape().1 as usize)*k..((self.get_shape().1 as usize) * (k+1))].iter().collect()),
            Matrix::Float(a)=>MatVector::Float(a.values[(self.get_shape().1 as usize)*k..((self.get_shape().1 as usize) * (k+1))].iter().collect()),
            Matrix::Bool(a)=>MatVector::Bool(a.values.iter().collect()),
            Matrix::Null=>MatVector::Null,}
    }

    fn col(&self, j:i32) -> MatVector {
        let k = j as usize;
        if j<0 || j>= self.get_shape().1 {
            eprintln!("\nfn col(&self, i:i32) >>> The col indice is out of range : {} is not in [{}-{}]. \n", k, 0, self.get_shape().0-1);
            std::process::exit(-1);
        }
        match &self {
            Matrix::Int(a)=>MatVector::Int({let mut r: Vec<&i32> = Vec::with_capacity(self.get_shape().0 as usize); for i in 0..self.get_shape().0 {r.push(&(a.values.get((i*self.get_shape().1 + j) as usize).expect("Indice out of range")))} r}),
            Matrix::Float(a)=>MatVector::Float({let mut r: Vec<&f64> = Vec::with_capacity(self.get_shape().0 as usize); for i in 0..self.get_shape().0 {r.push(&(a.values.get((i*self.get_shape().1 + j) as usize).expect("Indice out of range")))} r}),
            Matrix::Bool(a)=>MatVector::Bool({let mut r: Vec<&bool> = Vec::with_capacity(self.get_shape().0 as usize); for i in 0..self.get_shape().0 {r.push(&(a.values.get((i*self.get_shape().1 + j) as usize).expect("Indice out of range")))} r}),
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



    /* 
            println!("|");
            
            
            let mut i=0;
            println!("");
            print!("| ");
            for val in self.values.iter() {      
                if i == self.shape.1 {
                    println!("|");
                    print!("| ");
                    i=0
                }
                print!("{} ", val);
                i+=1;
            }
            println!("|");*/






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
    fn ones(shape1 : i32, shape2 : i32, dtype : &str) -> Matrix { 

        if dtype == "i32" {
            Matrix::Int( MatrixI{values : vec![1; (shape1*shape2) as usize ], shape:(shape1,shape2)})
        }
        else if dtype == "f64" {
            Matrix::Float( MatrixF{values : vec![1.0; (shape1*shape2) as usize ], shape:(shape1,shape2)})
        }
        else if dtype == "bool" {
            Matrix::Bool( MatrixB{values : vec![true; (shape1*shape2) as usize ], shape:(shape1,shape2)})
        }
        else {
            Matrix::Null
        }

        
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


  
/*
    fn ones(shape1 : i32, shape2 : i32) -> Matrix { Matrix {values : vec![Type::Int(1); ((shape1*shape2) as usize) ], shape:(shape1,shape2)}}
    fn eye(shape : i32) -> Matrix { Matrix {
    values:
    {let mut value : Vec<Type> = Vec::new();
        for i in 0..shape{
            for j in 0..shape{
                if i==j {
                    value.push(Type::Int(1));
                }
                else {
                    value.push(Type::Int(0));
                }     
            }
        }
    value},
    shape : (shape,shape),
    }}

    fn from_slice(slice: &[Type], shape1:i32, shape2:i32) -> Matrix { 
        
        if slice.len() != (shape1*shape2).try_into().unwrap() {
            println!("\nfn from_slice : The lenght of the slice is not equal to the product of both shapes.");
            return Matrix{values:vec![Type::Int(0)], shape:(1,1)}
        }
        else {
            Matrix {values : {
                let mut v : Vec<Type> = Vec::new();
                for el in slice{
                    v.push(*el);
                }
                v
            }, shape:(shape1,shape2)} }
        } */
    



/*

fn TypeConvert_i32(i : i32) {
    Type::Int(i);
    }
fn TypeConvert_f64(i: f64) {
    Type::Float(i);
}
fn TypeConvert_bool(i: bool) {
    Type::Bool(i);
    }

fn TypeConvert(slice: &[T])
}



impl Matrix{
    fn show(&self) {
        let mut i=0;
        println!("");
        print!("| ");
        for val in self.values.iter() {      
            if i == self.shape.1 {
                println!("|");
                print!("| ");
                i=0
            }
            print!("{} ", val);
            i+=1;
        }
        println!("|");
    }
}








trait Mat { fn show(&self); }
impl Mat for Matrix<i32> { fn show(&self) {println!("256")} }
impl Mat for Matrix<f64> { fn show(&self) {println!("384")} }
impl Mat for Matrix<bool> { fn show(&self) {println!("512")} }

impl Mat for MatDataType {
    fn show(&self) {
        use MatDataType::*;
        match *self {
            Int(ref M )   => M.show(),
            Float(ref M)   => M.show(),
            Bool(ref M) => M.show(),
        }
    }
}



impl Matrix<i32> {
    fn new(shape1:i32, shape2:i32) -> Matrix<i32> {
        Matrix::<i32> { values: vec![0;((shape1*shape2) as usize).try_into().unwrap()] , shape: (shape1,shape2)}
    }

}

*/




/*
impl Mat {
    fn new(self, str: str) ->  {}
}


/* Constructors */
impl<T: Clone> Matrix<T> {
    fn zeros(shape1 : i32, shape2 : i32) -> Matrix<i32>{
        Matrix::<i32>{
            values: vec![0; (shape1*shape2).try_into().unwrap()],
            shape : (shape1,shape2),
            }  
        }
    fn ones(shape1 : i32, shape2 : i32) -> Matrix<i32>{
        Matrix::<i32>{
            values: vec![1; (shape1*shape2).try_into().unwrap()],
            shape : (shape1,shape2),
            }  
        }
        
    fn eye(shape : i32) -> Matrix<i32>{
        Matrix::<i32>{
            values:
            {let mut value : Vec<i32> = Vec::new();
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
            value},
            shape : (shape,shape),
            }  
        
        }
        
    
}

/* General methods for Matrix */
impl<T: Display> Matrix<T> {
    fn show(&self) {
        let mut i=0;
        println!("");
        print!("| ");
        for val in self.values.iter() {      
            if i == self.shape.1 {
                println!("|");
                print!("| ");
                i=0
            }
            print!("{} ", val);
            i+=1;
        }
        println!("|");
        
    }
}

impl<T> Matrix<T> {
    fn loc(&self, i: i32, j : i32) -> Option<&T>{
        if i>(self.shape.0-1) || j> (self.shape.1-1 ){
            return None;
        }
        else {
            return self.values.get((i*self.shape.1 + j) as usize);
        }
    }
}





impl<T: Copy +  Clone + std::ops::Add<Output = T>  + std::ops::Mul<Output = T>> Matrix<T> {
    fn add(&self, Vec1 : Matrix<T>) -> Matrix<T> {
        Matrix::<T>{
        values:
            {
            let mut r : Vec<T> = Vec::new();
            for i in 0..self.values.len() {
                r.push(Vec1.values[i] + self.values[i]);
            }
            r},
        shape:self.shape,
        }
    }

}


impl<T : Copy +  std::ops::Mul<Output = T>> Mul<T> for Matrix<T> {
    type Output = Self;
    fn mul(self, rhs: T) -> Self::Output {
        Self { values: self.values.iter().map(|v| *v * rhs).collect(), shape:self.shape}
    }
}

*/

fn main() {
    let mat = &Matrix::ones(3,3,"i32");
    let mat2 = &(Matrix::eye(3,"i32"));

    println!("{}", (3)*(7*mat + (-2)*mat + 1*3*mat2));
    println!("{}", (8*mat));
    println!("{}", (3.15*(3*mat))/0.5);

    
}
