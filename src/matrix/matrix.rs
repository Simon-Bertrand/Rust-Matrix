use crate::matrix::*;
use num_rational::Ratio;
use num_traits::Zero;
use num_traits::NumOps;


use crate::matrix::constructors::Constructors;


impl<T> Matrix<T> {
    pub fn get(&self, i:usize, j:usize) -> &T {
        if i>= self.shape.0 {
            eprintln!("\nfn get_mut(&mut self, i:usize, j:usize) >>> The indice is out of range : {} is not in [{}-{}] or {} is not in [{}-{}] . \n", i, 0, self.shape.0-1, j, 0, self.shape.1-1);
            std::process::exit(-1);
        }
       else {
            return self.values.get(i*self.shape.1 + j).unwrap()
        }
    }

    pub fn get_mut(&mut self, i:usize, j:usize) ->  &mut T  {
        if i>= self.shape.0 || j>= self.shape.1 {
            eprintln!("\nfn get_mut(&mut self, i:usize, j:usize) >>> The indice is out of range : {} is not in [{}-{}] or {} is not in [{}-{}] . \n", i, 0, self.shape.0-1, j, 0, self.shape.1-1);
            std::process::exit(-1);
        }
       else {
            return self.values.get_mut(i*self.shape.1 + j).unwrap()
        }
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
            eprintln!("\nfn col_iter(&self, j:usize) >>> The col indice is out of range : {} is not in [{}-{}]. \n", i, 0, self.shape.1-1);
            std::process::exit(-1);
        }
        else {
            MatrixIter::<'a, T> { data: &self, ind_axe : i, index : 0, axe : false}
        }
    }

    pub fn row_iter(&'a self, i : usize) -> MatrixIter<'a, T> {
        if i>= self.shape.0 {
            eprintln!("\nfn row_iter(&self, i:usize) >>> The row indice is out of range : {} is not in [{}-{}]. \n", i, 0, self.shape.0-1);
            std::process::exit(-1);
        }
        else {
            MatrixIter::<'a, T> { data: &self, ind_axe : i, index : 0, axe : true}
        }
    }
}



impl<T> Matrix<T> {
    pub fn row_iter_mut(&mut self, i:usize) ->  &mut [T]  {
        if  i>= self.shape.0 {
            eprintln!("\nfn row_iter_mut(&mut self, i:usize) >>> The row indice is out of range : {} is not in [{}-{}]. \n", i, 0, self.shape.0-1);
            std::process::exit(-1);
        }
       else {
            return self.values.chunks_mut(self.shape.0).nth(i).unwrap()
        }
    }

    pub fn col_iter_mut(&mut self, j:usize) ->  std::iter::StepBy<std::slice::IterMut<'_, T>>  {
        if j>= self.shape.1 {
            eprintln!("\nfn col_iter(&self, j:usize) >>> The col indice is out of range : {} is not in [{}-{}]. \n", j, 0, self.shape.1-1);
            std::process::exit(-1);
        }
       else {
            return self.values[j..].iter_mut().step_by(self.shape.0)
        }
    }
}


impl<T: std::clone::Clone + Copy> Matrix<T> {

    pub fn row(&self, i:usize) -> Matrix<T> {
        if  i>= self.shape.0 {
            eprintln!("\nfn row(&self, i:usize) >>> The row indice is out of range : {} is not in [{}-{}]. \n", i, 0, self.shape.0-1);
            std::process::exit(-1);
        }
       else {
            Matrix::<T> {values:(self.values[(i*self.shape.1)..(i*self.shape.1 + self.shape.0)]).to_vec(), shape:(1,self.shape.1)}
        }
    }
    
    pub fn col(&self, j:usize) -> Matrix<T> {
        if j>= self.shape.1 {
            eprintln!("\nfn col(&self, i:usize) >>> The col indice is out of range : {} is not in [{}-{}]. \n", j, 0, self.shape.0-1);
            std::process::exit(-1);
        }
        else {
            Matrix::<T> {values:{let mut r : Vec<T> = Vec::with_capacity(self.shape.0); for col_el in self.col_iter(j) { r.push(*col_el)} r}, shape:(1,self.shape.1)}
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
            eprintln!("\nfn dot(&self, M: &Matrix) >>> The shapes are not compatible for matrix product.\n");
            std::process::exit(-1);
        }
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




fn core_min_max_all<T : PartialOrd>(this : &Matrix<T>, is_max : bool) -> &T {
    let mut lhs : &T = &this.values[0];
    for rhs in this.values.iter() {
        if is_max {if lhs < rhs  { lhs = rhs;}} else {if lhs > rhs  { lhs = rhs;}}
    }
    lhs
}

fn core_min_max<T : PartialOrd + Zero + Clone + Copy>(this : &Matrix<T>, axis : bool, is_max : bool) -> Matrix<T>{
    if this.is_col() || this.is_row() {
        return Matrix::<T> {
            values: vec![{
                let mut lhs : T = this.values[0];
                for rhs in this.values.iter() {if is_max {if lhs < *rhs  { lhs = *rhs;}} else {if lhs > *rhs  { lhs = *rhs;}}}
                lhs
            }],
            shape : (1,1)
        }
    }
    else {
        if axis { 
            let mut result : Matrix<T> = Matrix::fill(this.shape.0,1, Zero::zero());
            for j in 0..this.shape.0 {
                let mut lhs : T = *this.get(j,0);
                for rhs in this.row_iter(j) {if is_max {if lhs < *rhs  { lhs = *rhs;}} else {if lhs > *rhs  { lhs = *rhs;}}}
                *result.get_mut(j,0) = lhs;
            }
            result
            
        }
        else {
            let mut result : Matrix<T> = Matrix::fill(1,this.shape.1, Zero::zero());
            for i in 0..this.shape.1 {
                let mut lhs : T = *this.get(0,i);
                for rhs in this.col_iter(i)  {if is_max {if lhs < *rhs  { lhs = *rhs;}} else {if lhs > *rhs  { lhs = *rhs;}}}
                *result.get_mut(0,i) = lhs;
            }
            result
        }}
}

impl<T : PartialOrd + Zero + Clone + Copy> Matrix<T> {
    pub fn max(&self, axis : bool) -> Matrix<T> {
        core_min_max(self, axis, true)
    }
    pub fn min(&self, axis : bool) -> Matrix<T> {
        core_min_max(self, axis, false)
    }

}

impl<T : Ord + Zero> Matrix<T> {
    pub fn max_all(&self) -> &T {
        core_min_max_all(self, true)
    }
    pub fn min_all(&self) -> &T {
        core_min_max_all(self, false)
    }
}












impl<T : std::cmp::PartialOrd<T> + Zero + Clone + Copy> Matrix<T> {
    pub fn sum_all(&self) -> T {
        let mut t_s : T =Zero::zero();
        for el in self.values.iter()  { t_s = t_s + *el;}
        t_s
    }
}

fn core_sum_mean_norm<T : std::iter::Sum + NumOps + Zero + Copy>(this : &Matrix<T>, axis:bool, reduct : &T , squared : bool) -> Matrix<T> {
    if this.is_col() || this.is_row() {
        Matrix::<T> {
            values: {
                let mut r : Vec<T> = Vec::with_capacity(1);
                let mut t_s : T  = Zero::zero(); 
                for el in this.values.iter().map(|v| if squared {*v**v} else {*v}) {t_s = t_s + el}
                r.push(t_s/ *reduct);
                r
            },
            shape : (1,1)
        }
    } 
    else {
        if axis {
            return Matrix::<T> {values: {
                let mut r : Vec<T> = Vec::with_capacity(this.shape.0);
                for i in 0..this.shape.0 {
                    let mut t_s : T  = Zero::zero(); 
                    for el in this.row_iter(i)  {t_s = t_s + {if squared {*el**el} else {*el}};}
                    r.push(t_s / *reduct )
                }   
                r}, shape:(this.shape.0,1)}      
        }
        else {
            return Matrix::<T> {values: {
                let mut r : Vec<T> = Vec::with_capacity(this.shape.0);
                for j in 0..this.shape.1 {
                    let mut t_s :T  = Zero::zero();
                    for el in this.col_iter(j)  {t_s = t_s + {if squared {*el**el} else {*el}};}
                    r.push(t_s / *reduct)
                } 
                r}, shape:(1,this.shape.1)}
        }
    }
}

macro_rules! sub_impl {
    ($($t:ty)*) => ($(
        impl Matrix<$t> {
            pub fn mean(&self, axis : bool) -> Matrix<$t> {
                core_sum_mean_norm(self, axis, &{if axis {self.shape.1 as $t} else {self.shape.0 as $t}}, false)
            }
            pub fn norm(&self, axis : bool) -> Matrix<$t> {
                core_sum_mean_norm(self, axis, &1.0, true)
            }
            pub fn sum(&self, axis : bool) -> Matrix<$t> {
                core_sum_mean_norm(self, axis, &1.0, false)
            }
        }
    )*)
}
sub_impl! { f32 f64 }

macro_rules! sub_impl {
    ($($t:ty)*) => ($(
        impl Matrix<$t> {
            pub fn mean(&self, axis : bool) -> Matrix<Ratio<$t>> {
                core_sum_mean_norm(&self.clone_to_ratio(), axis, &{if axis {Ratio::new((self.shape.1 as i32).into(),1)} else {Ratio::new((self.shape.0 as i32).into(),1)}} , false)
            }
            pub fn norm(&self, axis : bool) -> Matrix<$t> {
                core_sum_mean_norm(self, axis, &1 , true)
            }
            pub fn sum(&self, axis : bool) -> Matrix<$t> {
                core_sum_mean_norm(self, axis, &1 , false)
            }
        }
    )*)
}
sub_impl! { i32 i64 }

macro_rules! sub_impl {
    ($($t:ty)*) => ($(
        impl Matrix<$t> {
            pub fn mean(&self, axis : bool) -> Matrix<$t> {
                core_sum_mean_norm(&self, axis, &{if axis {Ratio::new((self.shape.1 as i32).into(),1)} else {Ratio::new(1,(self.shape.0 as i32).into())}} , false)
            }
            pub fn norm(&self, axis : bool) -> Matrix<$t> {
                core_sum_mean_norm(self, axis, &Ratio::new(1,1) , true)
            }
            pub fn sum(&self, axis : bool) -> Matrix<$t> {
                core_sum_mean_norm(self, axis, &Ratio::new(1,1) , false)
            }
        }
    )*)
}
sub_impl! { Ratio<i32> Ratio<i64> }


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

macro_rules! sub_impl {
    ($($t:ty)*) => ($(
        impl Matrix<$t> {
            pub fn round(mut self) -> Self{
                for el in self.values.iter_mut() {
                    println!("{}", el.abs());
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
