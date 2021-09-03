use crate::matrix::*;


impl std::fmt::Display for Matrix {
    fn fmt(&self, _f: &mut std::fmt::Formatter) -> std::fmt::Result {
        fn show<T: std::fmt::Display>(vect: &Vec<T>, shape:&(i32,i32)) {
            let mut i=0;
            println!("");
            print!("| ");
            for val in vect.iter() {      
                if i == shape.1 {
                    println!("|");
                    print!("| ");
                    i=0
                }
                print!("{} ", val);
                i+=1;
            }
            println!("|");
        }
        match &self {
            Matrix::Int(a)=>Ok({show(&a.values,&a.shape); println!("-Int ({},{})-", &a.shape.0, &a.shape.1)}),
            Matrix::Float(a)=>Ok({show(&a.values,&a.shape);println!("-Float ({},{})-", &a.shape.0, &a.shape.1)}),
            Matrix::Bool(a)=>Ok({show(&a.values, &a.shape);println!("-Bool ({},{})-", &a.shape.0, &a.shape.1)}),
            Matrix::Null=>Err(std::fmt::Error),
        }
    }
}


impl Matrix {
    pub fn show(&self) {
        println!("{}", self)
    }
}



