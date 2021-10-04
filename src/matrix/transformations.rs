use crate::matrix::*;

use num_rational::Ratio;
impl<T : Copy> Matrix<T> {
    pub fn swap(&mut self, i:i32, j:i32, axe:bool) -> &mut Matrix<T> {
        if axe {
            if i>=self.shape.0 || j>=self.shape.0 {
                eprintln!("\nfn swap(&mut self, i:i32, j:i32, axe:bool)  >>> The indice i or j to swap rows is too high. \n");
                std::process::exit(-1);
            }
            else {
                for k in 0..self.shape.1 {
                    let temp_val : T = *self.get(i,k);
                    *self.get_mut(i,k) = *self.get(j,k);
                    *self.get_mut(j,k) = temp_val;
                }
            }
        }
        else {
            if i>=self.shape.1 || j>=self.shape.1 {
                eprintln!("\nfn swap(&mut self, i:i32, j:i32, axe:bool)  >>> The indice i or j to swap columns is too high. \n");
                std::process::exit(-1);
            }
            else {
                for k in 0..self.shape.0 {
                    let temp_val : T = *self.get(k,i);
                    *self.get_mut(k,i) = *self.get(k,j);
                    *self.get_mut(k,j) = temp_val;
                }
            }
        }
        self
    }
    pub fn concat(&mut self, vect_a : &Matrix<T>) -> &mut Self {
        if self.shape.0 == 0 && self.shape.1 == 0  {
            *self = vect_a.copy();
            self
        }
        else {
        if vect_a.is_col() {
            if vect_a.shape.0 == self.shape.0 {
                for i in 0..vect_a.shape.0 {
                    self.values.insert(((i+1)*(self.shape.1) + i) as usize,*vect_a.get(i,0));
                }
                self.shape = (self.shape.0,self.shape.1 + 1)
            }

            else {
                eprintln!("\nfn concat(&mut self, vect_a : &Matrix<T>)  >>> Shapes are not compatible with horizontal concatenate. \n");
                std::process::exit(-1);
            }
        }
        else if vect_a.is_row() {
            if vect_a.shape.1 == self.shape.1 {
                for j in 0..vect_a.shape.1 {
                    self.values.push(*vect_a.get(0,j));
                }
                self.shape = (self.shape.0+1,self.shape.1)
            }
            else {
                eprintln!("\nfn concat(&mut self, vect_a : &Matrix<T>)  >>> Shapes are not compatible with vertical concatenate. \n");
                std::process::exit(-1);
            }
        }
        else {
            eprintln!("\nfn concat(&mut self, vect_a : &Matrix<T>)  >>> Cannot concatenate a matrix that is not a column or a row. \n");
            std::process::exit(-1);
        }
        self
    }
    }
}



macro_rules! sub_impl {
    ($($t:ty)*) => ($(
        impl Matrix<$t>{
            pub fn clone_to_ratio(&self) -> Matrix<Ratio<$t>>{
                Matrix::<Ratio<$t>> {
                    values:self.values.iter().map(|v : &$t| Ratio::from_integer(*v)).collect(),
                    shape:self.shape,
                }
            }
        }
        
    )*)
}

sub_impl! {i32 i64}
