use crate::matrix::*;


use rand::Rng;
use std::vec;

impl Matrix {
    pub fn new(shape1 : i32, shape2 : i32, dtype : &str) -> Matrix { 

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
    pub fn ones(shape1 : i32, shape2 : i32) -> Matrix { 
        Matrix::new(shape1, shape2, "i32") + 1
    }
    pub fn fill_int(fill_value:i32, shape1 : i32, shape2 : i32) -> Matrix { 
        Matrix::new(shape1, shape2, "i32") + fill_value
    }
    pub fn fill_float(fill_value:f64, shape1 : i32, shape2 : i32) -> Matrix { 
        Matrix::new(shape1, shape2, "f64") + fill_value
    }

    pub fn rand_binary(shape1 : i32, shape2 : i32) -> Matrix { 
        Matrix::Int(MatrixI{values : {
            let mut r : Vec<i32> = Vec::with_capacity((shape1*shape2) as usize);
            let mut rng = rand::thread_rng();
            for _ in 0..shape1*shape2 {r.push(rng.gen_range(0..=1))}
            r
        }, shape: (shape1,shape2)})
    }


    pub fn eye(shape : i32, dtype : &str) -> Matrix { 
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
