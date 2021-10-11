use rand::Rng;
use std::vec;
use num_traits::Zero;
use num_traits::FromPrimitive;

use crate::matrix::* ;


trait Display {
    fn show(&self);
}

pub trait Constructors<T> {
    fn new(shape1 : usize, shape2 : usize) -> Self;
    fn fill(shape1 : usize, shape2 : usize, fill_value :T) -> Self;
}


impl<T : Clone + Zero> Constructors<T> for Matrix<T> {
    fn new(shape1 : usize, shape2 : usize) -> Matrix<T> {
        let zero : T = Zero::zero();
        return Matrix::fill(shape1,shape2, zero);
    }
    fn fill(shape1 : usize, shape2 : usize, fill_value : T) -> Matrix<T> {
        Matrix::<T>{values : vec![fill_value; shape1*shape2], shape:(shape1,shape2)}
    }

}

impl<T : Clone + Copy + Zero> Matrix<T> {
    pub fn fill_diagonal(shape : usize, value : T) -> Matrix<T>{
        let mut result = Matrix::new(shape,shape);
        for k in 0..shape {*result.get_mut(k,k) =value;}
        result
    }
    pub fn fill_tri(shape : usize, value : T, offset:i32) -> Matrix<T>{
        let mut result = Matrix::new(shape,shape);
        for i in 0..shape {
            for j in 0..shape {
                if i as i32 - (offset+1) < j as i32 {
                    *result.get_mut(i,j) = value;
                }
            }
        }
        result
    }
}

impl<T : FromPrimitive + Zero + Clone + Copy> Matrix<T> {
    pub fn rand_binary(shape1 : usize, shape2: usize) -> Matrix<T> {
        let mut result  = Matrix::<T>::fill(shape1,shape2, Zero::zero());
        let mut rng = rand::thread_rng();
        for mut_el in result.values.iter_mut() {
            *mut_el = FromPrimitive::from_i32(rng.gen_range(0..=1)).unwrap();
        } result
    }
    
}

impl<T : std::str::FromStr > Matrix<T> {
    pub fn from_csv(path : &str) -> Matrix<T> {
        let mut rdr = csv::Reader::from_path(path).unwrap();
        let mut data = Vec::<T>::new();
        let mut n_cols : usize = 0;
        let mut n_rows : usize = 0;
        for rows in rdr.records() {
            n_rows+=1;
          match rows {
            Ok(result) => {
              n_cols = result.len();
              for el in result.iter() {
                
                data.push(el.parse::<T>().ok().unwrap());
              }
          },
            Err(_result) => {}
          }
            
        }
        Matrix::<T> {values : data, shape : (n_rows, n_cols)}
    }
}
