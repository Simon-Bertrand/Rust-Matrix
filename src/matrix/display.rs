use crate::matrix::*;
use crate::utils::functions::count_digits;


fn print_type_of<T>(_: &T) {
    print!("- {} ", std::any::type_name::<T>())
}

impl<T : std::fmt::Display + std::cmp::PartialOrd> std::fmt::Display for Matrix<T> {
    fn fmt(&self, _f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let spaces = count_digits(self.max());
            let mut i=0;
            println!("");
            print!("|");
            for val in self.values.iter() {      
                if i == self.shape.1 {
                    println!("{val:>width$}", val="|", width=4);
                    print!("|");
                    i=0
                }
                print!("{val:>width$}", val=val, width=spaces);
                i+=1;
            }
            println!("{val:>width$}", val="|", width=4);
            Ok({
                print_type_of(self.values.iter().nth(0).expect("Type not found"));
                println!("({},{}) -", self.shape.0, self.shape.1)}
            )
    }
}


impl<T : std::fmt::Display + std::cmp::PartialOrd> Matrix<T> {
    pub fn show(&self) {
        println!("{}", self)
    }
}



