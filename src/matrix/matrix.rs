use num_traits::Zero;
use num_traits::NumOps;


use crate::matrix::utils::exception;
use crate::matrix::*;






impl<T> Matrix<T> {
    pub fn get(&self, i:usize, j:usize) -> &T {
        if i>= self.shape.0 || j>=self.shape.1 {
            exception::raise_exception(
                &"get",
                &mut String::from(format!("The indice is out of range : {} is not in [{}-{}] or {} is not in [{}-{}].", i, 0, self.shape.0-1, j, 0, self.shape.1-1)),
                String::from("Choose indices i and j contained in the correct domains."),
                100,
                30001);
                panic!();
        }
       else {
            return self.values.get(i*self.shape.1 + j).unwrap()
        }
    }

    pub fn get_mut(&mut self, i:usize, j:usize) ->  &mut T  {
        if i>= self.shape.0 || j>= self.shape.1 {
            exception::raise_exception(
                &"get_mut",
                &mut String::from(format!("The indice is out of range : {} is not in [{}-{}] or {} is not in [{}-{}].", i, 0, self.shape.0-1, j, 0, self.shape.1-1)),
                String::from("Choose indices i and j contained in the correct domains."),
                100,
                30002);
                panic!();
        
        }
       else {
            return self.values.get_mut(i*self.shape.1 + j).unwrap()
        }
    }
}

impl<T> Matrix<T> {
    pub fn length(&self) -> usize {
        self.shape.0*self.shape.1
    }
}


pub struct MatrixIter<'a, T> {
    data: &'a Matrix<T>,
    ind_axe : usize,
    index : usize,
    axe : bool
}

impl<'a, T> Iterator for MatrixIter<'a, T>{ 
    type Item = &'a T;
    fn next(&mut self) ->Option<Self::Item> {
     while self.index < {if self.axe {self.data.shape.1} else {self.data.shape.0}} {
        self.index += 1;
        return Some(&self.data.values[{if self.axe {self.ind_axe*self.data.shape.1 + self.index-1} else {(self.index-1)*self.data.shape.1 + self.ind_axe}}])
     }
     None
    }
}


impl<'a,T> Matrix<T> {
    pub fn col_iter(&'a self, i : usize) -> MatrixIter<'a, T> {
        if  i>= self.shape.1 {
            exception::raise_exception(
                &"col_iter",
                &mut String::from(format!("The col indice is out of range : {} is not in [{}-{}].", i, 0, self.shape.0-1)),
                String::from("Choose column indice in the correct domain."),
                100,
                30003);
                panic!();

        }
        else {
            MatrixIter::<'a, T> { data: &self, ind_axe : i, index : 0, axe : false}
        }
    }

    pub fn row_iter(&'a self, i : usize) -> MatrixIter<'a, T> {
        if i>= self.shape.0 {
            exception::raise_exception(
                &"row_iter",
                &mut String::from(format!("The row indice is out of range : {} is not in [{}-{}].", i, 0, self.shape.0-1)),
                String::from("Choose row indice in the correct domain."),
                100,
                30004);
                panic!();
        }
        else {
            MatrixIter::<'a, T> { data: &self, ind_axe : i, index : 0, axe : true}
        }
    }
}



impl<T> Matrix<T> {
    pub fn row_iter_mut(&mut self, i:usize) ->  &mut [T]  {
        if  i>= self.shape.0 {
            exception::raise_exception(
                &"row_iter_mut",
                &mut String::from(format!("The row indice is out of range : {} is not in [{}-{}].", i, 0, self.shape.0-1)),
                String::from("Choose row indice in the correct domain."),
                100,
                30005);
                panic!();
        }
       else {
            return self.values.chunks_mut(self.shape.0).nth(i).unwrap()
        }
    }

    pub fn col_iter_mut(&mut self, j:usize) ->  std::iter::StepBy<std::slice::IterMut<'_, T>>  {
        if j>= self.shape.1 {
            exception::raise_exception(
                &"col_iter_mut",
                &mut String::from(format!("The col indice is out of range : {} is not in [{}-{}].", j, 0, self.shape.1-1)),
                String::from("Choose column indice in the correct domain."),
                100,
                30006);
                panic!();
        }
       else {
            return self.values[j..].iter_mut().step_by(self.shape.0)
        }
    }
}


impl<T: std::clone::Clone + Copy> Matrix<T> {
    pub fn row(&self, i:usize) -> Matrix<T> {
        if  i>= self.shape.0 {
            exception::raise_exception(
                &"row",
                &mut String::from(format!("The row indice is out of range : {} is not in [{}-{}].", i, 0, self.shape.0-1)),
                String::from("Choose row indice in the correct domain."),
                100,
                30007);
                panic!();
        }
       else {
            Matrix::<T> {values:(self.values[(i*self.shape.1)..(i*self.shape.1 + self.shape.0)]).to_vec(), shape:(1,self.shape.1)}
        }
    }
    
    pub fn col(&self, j:usize) -> Matrix<T> {
        if j>= self.shape.1 {
            exception::raise_exception(
                &"col",
                &mut String::from(format!("The column indice is out of range : {} is not in [{}-{}].", j, 0, self.shape.1-1)),
                String::from("Choose col indice in the correct domain."),
                100,
                30008);
                panic!();

        }
        else {
            Matrix::<T> {values:{let mut r : Vec<T> = Vec::with_capacity(self.shape.0); for col_el in self.col_iter(j) { r.push(*col_el)} r}, shape:(self.shape.0,1)}
        }
    }
    
}


impl<T : std::convert::AsMut<Matrix<T>>> AsMut<Matrix<T>> for Matrix<T> {
    fn as_mut(&mut self) -> &mut Matrix<T> {
        self
    }
}

impl<T : Copy + Zero + NumOps> Matrix<T>{
    pub fn dot(&self, m: &Matrix<T>) -> Matrix<T> {
        if self.shape.1 != m.shape.0 {
            exception::raise_exception(
                &"dot",
                &mut String::from(format!("The shapes are not compatible for matrix product : {}!={}", self.shape.1, m.shape.0)),
                String::from("For matrix A and B, the dot product A.B is computable if A.shape.1==B.shape.0."),
                100,
                30008);
                panic!();
            
        }

        else {
            Matrix::<T> {
                values:{
                    let mut r:Vec<T>= Vec::with_capacity(self.shape.1*m.shape.0); 
                    for i in 0..self.shape.0 {
                        for j in 0..m.shape.1 {
                            r.push({
                                let mut sum : T = Zero::zero();
                                    for k in 0..m.shape.0 {
                                        sum = sum + *self.get(i,k)* *m.get(k,j);
                                    } 
                                    sum
                            })
                        }
                    }
                    r
                },
                shape: (self.shape.0,m.shape.1)
            }
        }
    }
}




impl<T : Copy> Matrix<T> {
    pub fn copy(&self) -> Matrix<T> {
        Matrix::<T> {values:self.values.clone(),shape:self.shape.clone()}
    }
}


impl<T> Matrix<T> {
    pub fn is_row(&self) -> bool {
        if self.shape.0 == 1 {
            return true;
        }
        else {
            return false;
        }
    }
    pub fn is_col(&self) -> bool {
        if self.shape.1 == 1 {
            return true;
        }
        else {
            return false;
        }
    }
}














impl<T : std::fmt::Display + Copy> Matrix<T> {
   

    pub fn copy_transpose(&self) -> Matrix<T> {
        return Matrix::<T> {values: {
            let mut r : Vec<T> = Vec::with_capacity(self.shape.0*self.shape.1);
            for i in 0..self.shape.1 {
                for j in 0..self.shape.0 {
                    r.push(*self.get(j,i));
                } 
            }
            r}, shape:(self.shape.1,self.shape.0)}
    }

   
}










impl<T : Copy + Zero + PartialEq + std::fmt::Display> Matrix<T> {
    pub fn select(&self, mask : &Matrix<bool>)  -> Matrix<T> {
        if self.shape == mask.shape {
            let mut nb_el : usize = 0;
            Matrix::<T> {
                values : {
                    let mut r : Vec<T> = Vec::with_capacity(self.length());
                    for (i,el) in self.values.iter().enumerate() {
                        if *mask.values.get(i).unwrap() {
                            r.push(*el);
                            nb_el += 1;
                        }
                    }
                r},
                shape: (1,nb_el)
            }
        }
        else {
            exception::raise_exception(
                &"select",
                &mut String::from("The mask shape needs to be the same as self matrix."),
                String::from("Choose matrix A and B such as A.shape==B.shape"),
                100,
                10002);
                panic!();
        }
        
    }
}




pub struct MatrixMaskIter<'a, T> {
    data: &'a Matrix<T>,
    mask : &'a Matrix<bool>,
    index : usize,
}

impl<'a, T> Iterator for MatrixMaskIter<'a, T>{ 
    type Item = &'a T;
    fn next(&mut self) ->Option<Self::Item> {
     while self.index < self.data.length() {
         if *self.mask.values.get(self.index).unwrap() {
            self.index += 1;
            return Some(&self.data.values[self.index-1])
         }
         else {
            self.index += 1;
         }
     }
     None
    }
}


impl<'a,T> Matrix<T> {
    pub fn select_iter(&'a self, mask : &'a Matrix<bool>) -> MatrixMaskIter<'a, T> {
        if  mask.shape != self.shape {
            exception::raise_exception(
                &"select_iter",
                &mut String::from("The mask shape needs to be the same as self matrix."),
                String::from("Choose matrix A and B such as A.shape==B.shape"),
                100,
                10003);
                panic!();
        }
        else {
            MatrixMaskIter::<'a, T> { data: &self, mask: &mask, index : 0}
        }
    }

}







