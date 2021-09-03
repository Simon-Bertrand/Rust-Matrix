use crate::matrix::Matrix;
use crate::utils::functions::count_digits;

impl std::fmt::Display for Matrix {
    fn fmt(&self, _f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let spaces = 1 + count_digits(self.max());
        fn show<T: std::fmt::Display>(vect: &Vec<T>, shape:&(i32,i32), space:i32) {
            let mut i=0;
            println!("");
            print!("|");
            for val in vect.iter() {      
                if i == shape.1 {
                    println!("{val:>width$}", val="|", width=(space as usize));
                    print!("|");
                    i=0
                }
                print!("{val:>width$}", val=val, width=(space as usize));
                i+=1;
            }
            println!("{val:>width$}", val="|", width=(space as usize));
        }
        match &self {
            Matrix::Int(a)=>Ok({show(&a.values,&a.shape, spaces); println!("-Int ({},{})-", &a.shape.0, &a.shape.1)}),
            Matrix::Float(a)=>Ok({show(&a.values,&a.shape,spaces);println!("-Float ({},{})-", &a.shape.0, &a.shape.1)}),
            Matrix::Bool(a)=>Ok({show(&a.values, &a.shape,spaces);println!("-Bool ({},{})-", &a.shape.0, &a.shape.1)}),
            Matrix::Null=>Err(std::fmt::Error),
        }
    }
}


impl Matrix {
    pub fn show(&self) {
        println!("{}", self)
    }
}



