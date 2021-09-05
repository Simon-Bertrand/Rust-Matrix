

use crate::matvector::*;


impl MatVector<'_> {
}
    /*
    pub fn from_int_vec<'a>(mut vect : Vec<i32>) -> MatVector<'a> { MatVector::Int((vect).iter_mut().collect())}
    pub fn from_float_vec<'a>(mut vect : Vec<f64>) -> MatVector<'a> { MatVector::Float((vect).iter_mut().collect())}
    pub fn from_bool_vec<'a>(mut vect : Vec<bool>) -> MatVector<'a> { MatVector::Bool((vect).iter_mut().collect())}
    */




/* 
impl Add<MatVector<'_>> for MatVector<'_> {
    type Output<'_> = MatVector<'_>;
    fn add(&self, rhs : MatVector) -> MatVector {
        match &self {
            MatVector::Int(a)=>{
                match rhs {
                    MatVector::Int(b)=>MatVector::Int({let mut r : Vec<&i32> = Vec::with_capacity(a.len()); for i in 0..a.len() {r.push(a[i]+b[i])} r}),
                    MatVector::Float(b)=>MatVector::Float({let mut r : Vec<&f64> = Vec::with_capacity(a.len()); for i in 0..a.len() {r.push((a[i] as f64)+b[i])} r}),
                    MatVector::Bool(b)=>MatVector::Null,
                    MatVector::Null=>MatVector::Null,}
            },
            MatVector::Float(a)=>{
                match rhs {
                    MatVector::Int(b)=>MatVector::Float({let mut r : Vec<&f64> = Vec::with_capacity(a.len()); for i in 0..a.len() {r.push((b[i] as f64)+a[i])} r}),
                    MatVector::Float(b)=>MatVector::Float({let mut r : Vec<&f64> = Vec::with_capacity(a.len()); for i in 0..a.len() {r.push(a[i]+b[i])} r}),
                    MatVector::Bool(b)=>MatVector::Null,
                    MatVector::Null=>MatVector::Null,}

            },
            MatVector::Bool(a)=>{
                match rhs {
                    MatVector::Int(b)=>MatVector::Null,
                    MatVector::Float(b)=>MatVector::Null,
                    MatVector::Bool(b)=>MatVector::Null,
                    MatVector::Null=>MatVector::Null,}

            },
            MatVector::Null=>MatVector::Null,
        }
    }
}


*/