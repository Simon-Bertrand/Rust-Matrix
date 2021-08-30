
use std::convert::TryInto;
use std::fmt::Display;
use core::ops::Mul;

#[derive(Clone, Copy)]
enum Type {
    Int(i32),
    Float(f64),
    Bool(bool),
}

impl std::fmt::Display for &Type {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Type::Int(a) => Ok(print!("{}",a)),
            Type::Float(a) => Ok(print!("{}",a)),
            Type::Bool(a)  => Ok(print!("{}",a)),
        }
        
    }
}




struct Matrix {
    values: Vec<Type>,
    shape : (i32,i32),

}

impl Matrix {
    fn new(shape1 : i32, shape2 : i32) -> Matrix { Matrix {values : vec![Type::Int(0); ((shape1*shape2) as usize) ], shape:(shape1,shape2)}}
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
        }
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






/*

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

    let Mat = Matrix::from_slice(&[Type::Int(1),Type::Int(10),Type::Float(1.2),Type::Bool(true)],2,2);
    Mat.show();
    /*
    let mut v = Matrix::<f64>::eye(6);
    let mut u = Matrix::<i32>::ones(6,6);
    */


    //((v.add(u)).mul(-1)).show();




    
}
