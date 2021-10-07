use crate::matrix::*;
use num_rational::Ratio;
use num_traits::Zero;
use num_traits::NumOps;
use std::cmp::Ord;
use crate::matrix::constructors::Constructors;


impl<T> Matrix<T> {
    pub fn get(&self, i:i32, j:i32) -> &T {
        if i<0 || i>= self.shape.0 {
            eprintln!("\nfn get_mut(&mut self, i:i32, j:i32) >>> The indice is out of range : {} is not in [{}-{}] or {} is not in [{}-{}] . \n", i, 0, self.shape.0-1, j, 0, self.shape.1-1);
            std::process::exit(-1);
        }
       else {
            return self.values.iter().nth((i*self.shape.1 + j) as usize).unwrap()
        }
    }

    pub fn get_mut(&mut self, i:i32, j:i32) ->  &mut T  {
        if (i<0 || i>= self.shape.0)||((j<0 || j>= self.shape.1)) {
            eprintln!("\nfn get_mut(&mut self, i:i32, j:i32) >>> The indice is out of range : {} is not in [{}-{}] or {} is not in [{}-{}] . \n", i, 0, self.shape.0-1, j, 0, self.shape.1-1);
            std::process::exit(-1);
        }
       else {
            return self.values.iter_mut().nth((i*self.shape.1 + j) as usize).unwrap()
        }
    }
}

impl<T> Matrix<T> {
    pub fn row_iter(&self, i:i32) -> &[T] {
        if i<0 || i>= self.shape.0 {
            eprintln!("\nfn row_iter(&self, i:i32) >>> The row indice is out of range : {} is not in [{}-{}]. \n", i, 0, self.shape.0-1);
            std::process::exit(-1);
        }
       else {
            return self.values.chunks(self.shape.0 as usize).nth(i as usize).unwrap()
        }
    }

    pub fn row_iter_mut(&mut self, i:i32) ->  &mut [T]  {
        if i<0 || i>= self.shape.0 {
            eprintln!("\nfn row_iter_mut(&mut self, i:i32) >>> The row indice is out of range : {} is not in [{}-{}]. \n", i, 0, self.shape.0-1);
            std::process::exit(-1);
        }
       else {
            return self.values.chunks_mut(self.shape.0 as usize).nth(i as usize).unwrap()
        }
    }

    pub fn col_iter(&self, j:i32) -> std::iter::StepBy<std::slice::Iter<'_, T>>{
        if j<0 || j>= self.shape.1 {
            eprintln!("\nfn col_iter(&self, j:i32) >>> The col indice is out of range : {} is not in [{}-{}]. \n", j, 0, self.shape.1-1);
            std::process::exit(-1);
        }
       else {
            return self.values[j as usize..].iter().step_by(self.shape.0 as usize)
        }
    }       

    pub fn col_iter_mut(&mut self, j:i32) ->  std::iter::StepBy<std::slice::IterMut<'_, T>>  {
        if j<0 || j>= self.shape.1 {
            eprintln!("\nfn col_iter(&self, j:i32) >>> The col indice is out of range : {} is not in [{}-{}]. \n", j, 0, self.shape.1-1);
            std::process::exit(-1);
        }
       else {
            return self.values[j as usize..].iter_mut().step_by(self.shape.0 as usize)
        }
    }
}


impl<T: std::clone::Clone + Copy> Matrix<T> {

    pub fn row(&self, i:i32) -> Matrix<T> {
        if i<0 || i>= self.shape.0 {
            eprintln!("\nfn row(&self, i:i32) >>> The row indice is out of range : {} is not in [{}-{}]. \n", i, 0, self.shape.0-1);
            std::process::exit(-1);
        }
       else {
            Matrix::<T> {values:(self.values[((i*self.shape.1) as usize)..((i*self.shape.1 + self.shape.0) as usize)]).to_vec(), shape:(1,self.shape.1)}
        }
    }
    
    pub fn col(&self, j:i32) -> Matrix<T> {
        if j<0 || j>= self.shape.1 {
            eprintln!("\nfn col(&self, i:i32) >>> The col indice is out of range : {} is not in [{}-{}]. \n", j, 0, self.shape.0-1);
            std::process::exit(-1);
        }
        else {
            Matrix::<T> {values:{let mut r : Vec<T> = Vec::with_capacity(self.shape.0 as usize); for col_el in self.col_iter(j) { r.push(*col_el)} r}, shape:(1,self.shape.1)}
        }
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
                let mut r:Vec<T>= Vec::with_capacity((self.shape.1*m.shape.0) as usize); 
                for i in 0..self.shape.0 {
                    for j in 0..m.shape.1 {
                        r.push({
                            let mut sum : T = Zero::zero();
                                for k in 0..(m.shape.0 as usize) {
                                    sum = sum + *self.get(i,k as i32)* *m.get(k as i32,j);
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





impl<T : Clone> Matrix<T> {
    pub fn flatten(&self) -> Matrix<T> {
        Matrix::<T> {
            values:self.values.clone(),
            shape: (1, self.shape.1*self.shape.0)
        }
    }

}





fn core_min_max<T : std::cmp::Ord + PartialOrd + Zero + Clone + Copy>(this : &Matrix<T>, axis : bool, is_max : bool) -> Matrix<T>{
    if this.is_col() || this.is_row() {
        Matrix::<T> {
            values: vec![{if is_max{*this.values.iter().max().unwrap()} else {*this.values.iter().min().unwrap()}}],
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

impl<T : Ord +  std::cmp::PartialOrd<T> + Zero + Clone + Copy> Matrix<T> {
    pub fn max(&self, axis : bool) -> Matrix<T> {
        core_min_max(self, axis, true)
    }
    pub fn min(&self, axis : bool) -> Matrix<T> {
        core_min_max(self, axis, false)
    }
    pub fn max_all(&self) -> &T {
        self.values.iter().max().unwrap()
    }
    pub fn min_all(&self) -> &T {
        self.values.iter().min().unwrap()
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
                let mut r : Vec<T> = Vec::with_capacity(this.shape.0 as usize);
                for i in 0..this.shape.0 {
                    let mut t_s : T  = Zero::zero(); 
                    for el in this.row_iter(i)  {t_s = t_s + {if squared {*el**el} else {*el}};}
                    r.push(t_s / *reduct )
                }   
                r}, shape:(this.shape.0,1)}      
        }
        else {
            return Matrix::<T> {values: {
                let mut r : Vec<T> = Vec::with_capacity(this.shape.0 as usize);
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
                core_sum_mean_norm(&self.clone_to_ratio(), axis, &{if axis {Ratio::new(self.shape.1.into(),1)} else {Ratio::new(self.shape.0.into(),1)}} , false)
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
                core_sum_mean_norm(&self, axis, &{if axis {Ratio::new(self.shape.1.into(),1)} else {Ratio::new(1,self.shape.0.into())}} , false)
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
