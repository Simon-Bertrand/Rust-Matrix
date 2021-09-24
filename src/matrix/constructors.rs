use crate::matrix::*;


use rand::Rng;
use std::vec;

trait Display {
    fn show(&self);
}

pub trait Constructors<T> {
    fn fill(shape1 : i32, shape2 : i32, fill_value :T) -> Self;

}


impl<T : Clone> Constructors<T> for Matrix<T> {
    fn fill(shape1 : i32, shape2 : i32, fill_value : T) -> Matrix<T> {
        Matrix::<T>{values : vec![fill_value; (shape1*shape2) as usize], shape:(shape1,shape2)}
    }

}



macro_rules! sub_impl {
    ($($t:ty)*) => ($(
        impl Matrix<$t> {
            pub fn fill_diagonal(shape : i32, value : $t) -> Matrix<$t>{
                let mut result = Matrix::fill(shape,shape,0 as $t);
                for k in 0..shape {*result.get_mut(k,k) =value;}
                result
            }
        }
    )*)
}

sub_impl! { usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 f32 f64 }



/*






impl<T> Matrix for i32 {
    pub fn new(shape1 : i32, shape2 : i32, dtype : &str) -> Matrix { 

        if dtype == "i32" {
           
        }
        else if dtype == "f64" {
            Matrix( Matrix::<f64>{values : vec![0.0; (shape1*shape2) as usize ], shape:(shape1,shape2)})

        }
        else if dtype == "bool" {
            Matrix( Matrix::<bool>{values : vec![false; (shape1*shape2) as usize ], shape:(shape1,shape2)})
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

    pub fn tri(shape : i32, offset: i32) -> Matrix { 
        Matrix::Int(MatrixStruct::<i32>{values : {let mut r : Vec<i32> = Vec::with_capacity((shape*shape) as usize); 
        for i in 0..shape {
            for j in 0..shape {
                if i<=j-offset {
                    r.push(1)
                }
                else {
                    r.push(0)
                }
                
            }
        }
        r}, shape:(shape, shape)})
    }

    pub fn linstep(shape1 : i32,shape2 : i32,step: i32) -> Matrix { 
        Matrix::Int(MatrixStruct::<i32>{values : {let mut r : Vec<i32> = Vec::with_capacity((shape1*shape2) as usize); 
        for i in 0..shape1*shape2 {
                r.push(step*i)
            }
        
        r}, shape:(shape1, shape2)})
    }

    pub fn rand_binary(shape1 : i32, shape2 : i32) -> Matrix { 
        Matrix::Int(MatrixStruct::<i32>{values : {
            let mut r : Vec<i32> = Vec::with_capacity((shape1*shape2) as usize);
            let mut rng = rand::thread_rng();
            for _ in 0..shape1*shape2 {r.push(rng.gen_range(0..=1))}
            r
        }, shape: (shape1,shape2)})
    }


    pub fn eye(shape : i32, dtype : &str) -> Matrix { 
        if dtype == "i32" {
            Matrix::Int( MatrixStruct::<i32>{values : {let mut value : Vec<i32> = Vec::new();
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
            Matrix::Float( MatrixStruct::<f64>{values : {let mut value : Vec<f64> = Vec::new();
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
            Matrix::Bool( MatrixStruct::<bool>{values : {let mut value : Vec<bool> = Vec::new();
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
    /*
    pub fn from_vec_of_vec_int(M:Vec<Vec<i32>>) -> Matrix {
        let mut result_vect : Vec<MatVector>= Vec::new();
        for vect in M.iter() {
            result_vect.push(MatVector::from_int_vec(*vect));
        }
        Matrix::from_matvector(&result_vect)
    }
    pub fn from_vec_of_vec_float(M:Vec<Vec<f64>>) -> Matrix {
        let mut result_vect : Vec<MatVector>= Vec::new();
        for vect in M.iter() {
            result_vect.push(MatVector::from_float_vec(*vect));
        }
        Matrix::from_matvector(&result_vect)
    }
    pub fn from_vec_of_vec_bool(M:Vec<Vec<bool>>) -> Matrix {
        let mut result_vect : Vec<MatVector>= Vec::new();
        for vect in M.iter() {
            result_vect.push(MatVector::from_bool_vec(*vect));
        }
        Matrix::from_matvector(&result_vect)
    }

    fn from_matvector(l_mv : &Vec<MatVector>) -> Matrix { 


        let mut mv_type_old= match &l_mv[0] {
            MatVector::Int(a)=>String::from("i32"),
            MatVector::Float(a)=>String::from("f64"),
            MatVector::Bool(a)=>String::from("bool"),
            MatVector::Null=>String::from("Null"),
        };

        for el in l_mv.iter() {
            let mv_type= match el {
                MatVector::Int(a)=>String::from("i32"),
                MatVector::Float(a)=>String::from("f64"),
                MatVector::Bool(a)=>String::from("bool"),
                MatVector::Null=>String::from("Null"),
            };
            if mv_type != mv_type_old {
                eprintln!("\nfn from_matvector(l_mv : Vec<MatVector>) >>> The MatVector variants are not the same inside the Vec. \n");
                std::process::exit(-1);
            }
            else {
                mv_type_old=mv_type
            }
        }


        let mut result_i : Vec<i32> = Vec::new();
        let mut result_f : Vec<f64> = Vec::new();
        let mut result_b : Vec<bool> = Vec::new();
        
        for el in l_mv.iter() {
            match el {
                MatVector::Int(a)=> for element in a {result_i.push(**element)},
                MatVector::Float(a)=>for element in a {result_f.push(**element)},
                MatVector::Bool(a)=>for element in a {result_b.push(**element)},
                MatVector::Null=>{},
            }
            
        }



        if mv_type_old == "i32" {
            Matrix::Int(MatrixStruct::<i32>{values:result_i, shape:(l_mv.len() as i32,l_mv[0].len() as i32)})
        }
        else if mv_type_old == "f64" {
            Matrix::Float(MatrixStruct::<f64>{values:result_f, shape:(l_mv.len() as i32,l_mv[0].len() as i32)})
        }
        else if mv_type_old == "bool" {
            Matrix::Bool(MatrixStruct::<bool>{values:result_b, shape:(l_mv.len() as i32,l_mv[0].len() as i32)})
        }
        else {
            Matrix::Null
        }

    }

*/
}

*/