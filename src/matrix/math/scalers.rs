use crate::matrix::* ;
use num_traits::Zero;
use num_traits::NumOps;
use num_traits::FromPrimitive;

pub trait MatrixScalers {
    fn scale_standardscaler(&self, axe : bool) -> Self;
    fn scale_minmaxscaler(&self, axe : bool) -> Self;
    fn scale_robustscaler(&self, axe : bool) -> Self;
}




fn core_percentile<T>(sorted_values : &Vec<T>, percentile : usize, shape : usize) -> T {
    if shape%2 ==0 {(sorted_values[shape/2 -1] + sorted_values[shape/2])/FromPrimitive::from_usize(100).unwrap() * FromPrimitive::from_usize(percentile).unwrap()} else {sorted_values[(shape -1)/2]};
}
fn core_scalers<T :std::cmp::Ord + NumOps + Zero + FromPrimitive + Copy,U>(this : &Matrix<T>, axe : bool, scaler : (bool, bool)) -> Matrix<T> {
    Matrix::<T> {
        values:{
            let mut r : Vec<T> = Vec::with_capacity(this.length());
                match scaler {
                    (false,false) => {
                        for i in 0..this.shape.0 {
                            let mut sum : T = Zero::zero();
                            let mut sum_squared : T = Zero::zero();  
                            for el in this.row_iter(i){sum = sum + *el; sum_squared = sum_squared + *el**el;}
                            for el in this.row_iter(i){r.push((*el-sum/FromPrimitive::from_usize(this.shape.0).unwrap())/(sum_squared/FromPrimitive::from_usize(this.shape.0).unwrap()- sum*sum))}
                        }
                    },
                    (false,true) => {
                        for i in 0..this.shape.0 {
                            let mut min : T = *this.row_iter(i).nth(0).unwrap();
                            let mut max : T = min.clone();  
                            for el in this.row_iter(i){if max<*el {max = *el;} if min>*el {min = *el;}}
                            for el in this.row_iter(i){r.push((*el - min)/(max-min))}
                        }
                    },
                    (true,false) => {
                        for i in 0..this.shape.0 {
                            let mut r2 : Vec<T> = Vec::with_capacity(this.shape.0);
                            for el in this.row_iter(i){r2.push(*el)}
                            r2.sort();
                            let median = if this.shape.0%2 ==0 {(r2[this.shape.0/2 -1] + r2[this.shape.0/2])/FromPrimitive::from_usize(2).unwrap()} else {r2[(this.shape.0 -1)/2]};
                            let firstquantile = if this.shape.0%2 ==0 {(r2[this.shape.0/2 -1] + r2[this.shape.0/2])/FromPrimitive::from_usize(100).unwrap()} else {r2[(this.shape.0 -1)/2]};
                            let thirrdquantile = if this.shape.0%2 ==0 {(r2[this.shape.0/2 -1] + r2[this.shape.0/2])/FromPrimitive::from_usize(100).unwrap()} else {r2[(this.shape.0 -1)/2]};
                            for el in this.row_iter(i){r.push((*el - min)/(max-min))}
                        }
                    },
                }
        r},
        shape: this.shape
    }
}