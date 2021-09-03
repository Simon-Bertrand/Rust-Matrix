use crate::matvector::MatVector;

impl std::fmt::Display for MatVector<'_> {
    fn fmt(&self, _f: &mut std::fmt::Formatter) -> std::fmt::Result{
        fn show_vector<T: std::fmt::Display>(vect: &Vec<&T>) {
            print!("[ ");
            for v in vect {
                print!("{} ",v);
            }
            print!("]\n");
        }
        match self {
            MatVector::Int(a)=>Ok(show_vector(a)),
            MatVector::Float(a)=>Ok(show_vector(a)),
            MatVector::Bool(a)=>Ok(show_vector(a)),
            MatVector::Null=>Err(std::fmt::Error),
        }
    }
}
