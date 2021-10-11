use num_rational::Ratio;
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











fn core_has_full_zeros_in_row_or_col<T : std::fmt::Display + PartialEq + Zero>(this :&Matrix<T>, axe : bool) -> bool {
    if axe {
        for i in 0..this.shape.0 {
            let mut compt : usize = 0;
            for col_el in this.row_iter(i) {
                if *col_el == Zero::zero() {
                    compt += 1;
                    if compt == this.shape.1 {
                        return true
                    }
                }

            }
        }
        return false
    } 
    else {
        for j in 0..this.shape.1 {
            let mut compt : usize = 0;
            for row_el in this.col_iter(j) {
                if *row_el == Zero::zero() {
                    compt += 1;
                    if compt == this.shape.0 {
                        return true
                    }
                }
            }       
        }
        return false
    }
}
impl<T : Zero + PartialEq + std::fmt::Display> Matrix<T> {
    pub fn has_full_zeros_in_rows(&self) -> bool {
        core_has_full_zeros_in_row_or_col(self, true)
    }
    pub fn has_full_zeros_in_cols(&self) -> bool {
        core_has_full_zeros_in_row_or_col(self, false)
    }
}
























macro_rules! sub_impl {
    ($($t:ty)*) => ($(
        impl Matrix<$t> {
            pub fn round(mut self) -> Self{
                for el in self.values.iter_mut() {
                    if el.abs() < 1e-10 {
                        *el = 0.0;
                    }
                    else {
                        *el = (*el as $t * 1e10).round()*1e-10;
                    }   
                }
                self
            }
        }
)*)
}
sub_impl! { f32 f64 }

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

