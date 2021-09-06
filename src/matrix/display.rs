use crate::matrix::*;
use crate::utils::functions::count_digits;



impl<T : std::fmt::Display> std::fmt::Display for Matrix<T> {
    fn fmt(&self, _f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let spaces = 1 + count_digits(3.0);
            let mut i=0;
            println!("");
            print!("|");
            for val in self.values.iter() {      
                if i == self.shape.1 {
                    println!("{val:>width$}", val="|", width=(spaces as usize));
                    print!("|");
                    i=0
                }
                print!("{val:>width$}", val=val, width=(spaces as usize));
                i+=1;
            }
            println!("{val:>width$}", val="|", width=(spaces as usize));
            Ok(println!("- Type ({},{})", self.shape.0, self.shape.1))
    }
}


impl<T : std::fmt::Display> Matrix<T> {
    pub fn show(&self) {
        println!("{}", self)
    }
}



