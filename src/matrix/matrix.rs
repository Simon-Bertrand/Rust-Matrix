use crate::matrix::*;

use num_rational::Ratio;
use num_traits::Zero;
use num_traits::NumOps;


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





impl<T : Copy> Matrix<T> {
    pub fn transpose(&self) -> Matrix<T> {
        Matrix::<T> {
            values:self.values.clone(),
            shape: (self.shape.1, self.shape.0)
        }
    }

    pub fn flatten(&self) -> Matrix<T> {
        Matrix::<T> {
            values:(*self.values).to_vec(),
            shape: (1, self.shape.1*self.shape.0)
        }
    }

}


impl<T : std::cmp::PartialOrd<T> + Zero + Clone + Copy> Matrix<T> {
    pub fn max(&self, axis : bool) -> Matrix<T> {
        if self.is_col() || self.is_row() {
            let mut max : Matrix<T> = Matrix::fill(1,1, Zero::zero());
            let mut t_max : T = self.values[0];
            for el in self.values.iter()  {if t_max < *el  {  t_max = *el;}}
            max.values[0] = t_max;
            max
        }
        else {
            if axis {
                let mut max : Matrix<T> = Matrix::fill(self.shape.0,1, Zero::zero());
                for j in 0..self.shape.0 {
                    let mut t_max : T = self.row_iter(j)[0].clone();
                    for el in self.row_iter(j)  {if t_max < *el  {  t_max = el.clone();}}
                    *max.get_mut(j,0) = t_max;
                }
                return max
                
            }
            else {
                let mut max : Matrix<T> = Matrix::fill(1,self.shape.1, Zero::zero());
                for i in 0..self.shape.1 {
                    let mut t_max : T = *self.col_iter(i).nth(0).unwrap();
                    for el in self.col_iter(i)  {if t_max < *el  {  t_max = *el;}}
                    *max.get_mut(0,i) = t_max;
                }
                return max
            }
        
        }
    }

    pub fn max_all(&self) -> T {
            let mut t_max : T = self.values[0];
            for el in self.values.iter()  {if t_max < *el  {  t_max = *el;}}

            t_max
    }

    pub fn min(&self, axis : bool) -> Matrix<T> {
        if self.is_col() || self.is_row() {
            let mut min : Matrix<T> = Matrix::fill(1,1, Zero::zero());
            let mut t_min : T = self.values[0];
            for el in self.values.iter()  {if t_min > *el  {  t_min = *el;}}
            min.values[0] = t_min;
            min
        }
        else {
            if axis {
                let mut min : Matrix<T> = Matrix::fill(self.shape.0,1, Zero::zero());
                for j in 0..self.shape.0 {
                    let mut t_min : T = self.row_iter(j)[0].clone();
                    for el in self.row_iter(j)  {if t_min > *el  {  t_min = el.clone();}}
                    *min.get_mut(j,0) = t_min;
                }
                return min
                
            }
            else {
                let mut min : Matrix<T> = Matrix::fill(1,self.shape.1, Zero::zero());
                for i in 0..self.shape.1 {
                    let mut t_min : T = *self.col_iter(i).nth(0).unwrap();
                    for el in self.col_iter(i)  {if t_min > *el  {  t_min = *el;}}
                    *min.get_mut(0,i) = t_min;
                }
                return min
            }
        
        }
    }

    pub fn min_all(&self) -> T {
            let mut t_min : T = self.values[0];
            for el in self.values.iter()  {if t_min > *el  {  t_min = *el;}}

            t_min
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
