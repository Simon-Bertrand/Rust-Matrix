use crate::matrix::*;



pub trait MatrixMutation {
    fn swap(&mut self, i:usize, j:usize, axe:bool) -> &mut Self;
    fn swap_el(&mut self, i : usize, j :usize, ip : usize, jp : usize);
    fn concat(&mut self, vect_a : &Self) -> &mut Self;
    fn transpose(&mut self) -> &mut Self;
    fn reshape(&mut self, i_new : usize, j_new : usize) -> &mut Self;
    fn flatten(&mut self) -> &mut Self;
}


impl<T : Copy> MatrixMutation for Matrix<T> {
    fn swap(&mut self, i:usize, j:usize, axe:bool) -> &mut Self {
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
    fn concat(&mut self, vect_a : &Matrix<T>) -> &mut Self {
        if self.shape.0 == 0 && self.shape.1 == 0  {
            *self = vect_a.copy();
            self
        }
        else {
            if vect_a.is_col() {
                if vect_a.shape.0 == self.shape.0 {
                    for i in 0..vect_a.shape.0 {
                        self.values.insert((i+1)*(self.shape.1) + i,*vect_a.get(i,0));
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

    fn swap_el(&mut self, i : usize, j :usize, ip : usize, jp : usize) {
        let temp_val : T = *self.get(ip,jp);
        *self.get_mut(ip,jp) = *self.get(i,j);
        *self.get_mut(i,j) = temp_val;
    }

     fn transpose(&mut self) -> &mut Self {
            self.values = {
                let mut temp_val = self.values.clone();
                for (klin,el) in temp_val.iter_mut().enumerate() {
                    *el = *self.get(klin%(self.shape.0), klin/(self.shape.0))
                }
                temp_val
            };
            self.shape = (self.shape.1,self.shape.0);
            self
    }

    fn reshape(&mut self, i_new : usize, j_new : usize) -> &mut Self {
        if i_new*j_new != self.length() {
            eprintln!("\nfn reshape(&mut self, i_new : usize, j_new : usize)  >>> The product of the new indices is not equal to the product of the current matrix shapes. \n");
            std::process::exit(-1);
        }
        else {
            self.shape = (i_new,j_new)
        }
    self
    }

    fn flatten(&mut self) -> &mut Self {
        self.shape= (1, self.shape.1*self.shape.0);
    self
    }


}





