use crate::matvector::*;

impl MatVector<'_> {
    pub fn len(&self)->usize{
        match self {
            MatVector::Int(a)=>a.len(),
            MatVector::Float(a)=>a.len(),
            MatVector::Bool(a)=>a.len(),
            MatVector::Null=>0,
        }
    }

}



impl MatVector<'_>{
    pub fn sprod(&self, vect: &'_ MatVector) -> f64{
        if (self.len()) != vect.len() {
            eprintln!("\nfn sprod(&self, vect: &MatVector) >>> The lengths are not the same : {} != {}. \n", self.len() , vect.len());
            std::process::exit(-1);
        }
        else {
            match &self {
                MatVector::Int(a)=>{
                    match vect {
                        MatVector::Int(b)=>{let mut s: i32 = 0; for i in 0..self.len() {s=s+ *a[i]**b[i]} s as f64},
                        MatVector::Float(b)=>{let mut s: f64 = 0.0; for i in 0..self.len() {s=s+(*a[i] as f64)**b[i]} s},
                        MatVector::Bool(b)=>{let mut s: f64 = 0.0; for i in 0..self.len() {s=s+0.0} s},
                        MatVector::Null=>0.0}}
                    MatVector::Float(a)=>{
                        match vect {
                            MatVector::Int(b)=>{let mut s: f64 = 0.0; for i in 0..self.len() {s=s+(*a[i] as f64)*(*b[i] as f64)} s},
                            MatVector::Float(b)=>{let mut s: f64 = 0.0; for i in 0..self.len() {s=s+(*a[i] as f64)**b[i]} s},
                            MatVector::Bool(b)=>{let mut s: f64 = 0.0; for i in 0..self.len() {s=s+0.0} s},
                            MatVector::Null=>0.0}},
                    MatVector::Bool(a)=>{
                        match vect {
                            MatVector::Int(b)=>0.0,
                            MatVector::Float(b)=>0.0,
                            MatVector::Bool(b)=>0.0,
                            MatVector::Null=>0.0}},
                    MatVector::Null=>0.0,
            }
        }
        
    }

    pub fn norm(&self) -> f64 {
        self.sprod(self).sqrt()
    }
}


