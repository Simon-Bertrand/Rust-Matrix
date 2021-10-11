use crate::matrix::*;

pub fn count_digits<T : std::string::ToString>(matrix : &Matrix<T>) -> usize {
    let mut max_lenght: usize =0;
    for el in matrix.values.iter() {
        if el.to_string().len() > max_lenght {

            max_lenght = el.to_string().len();
        }
    }
    max_lenght + 4
}


fn print_type_of<T>(_: &T) {
    if std::any::type_name::<T>().contains("Ratio<i32>") {
        print!("- Ratio<i32> ")
    }
    else if std::any::type_name::<T>().contains("Ratio<i64>")  {
        print!("- Ratio<i64> ")
    }
    else {
        print!("- {} ", std::any::type_name::<T>())
    }
    
}





impl<T : std::fmt::Display + std::cmp::PartialOrd> std::fmt::Display for Matrix<T> {
    fn fmt(&self, _f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.shape.0 == 1 && self.shape.1 == 1 {
            Ok(print!("\n {} \n", self.values[0]))
        }
        else {            
            let spaces = count_digits(&self);
            println!("");
            for (i,val) in self.values.iter().enumerate() { 
                if i%self.shape.1 ==0 {
                    if self.shape.1==1 && i!=0 {
                        print!("   | \n");
                    } 
                    print!("|{val:>width$}", val=val, width=spaces);
                }
                else if i%self.shape.1 == self.shape.1-1 {  
                    print!("{val:>width$}    |\n", val=val, width=spaces);
                }
                else {
                    print!("{val:>width$}", val=val, width=spaces);
                }
              
                
            }
            if self.shape.1==1 {
                print!("   | \n");
            } 
            Ok({
                print_type_of(self.values.iter().nth(0).expect("Empty values."));
                println!("({},{}) -", self.shape.0, self.shape.1)}
            )}
    }
}


impl<T : std::fmt::Display + std::cmp::PartialOrd> Matrix<T> {
    pub fn show(&self) {
        println!("{}", self)
    }
}



/*
 
impl<T : std::fmt::Display + std::cmp::PartialOrd> std::fmt::Display for Matrix<T> {
    fn fmt(&self, _f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.shape.0 == 1 && self.shape.1 == 1 {
            Ok(print!("\n {} \n", self.values[0]))
        }
        else {            
            let spaces = count_digits(&self);
            let mut i=0; 
            println!("");
            for val in self.values.iter() {      
                if i==0 {
                    print!("{}", "|    ");
                    print!("{val:>width$}", val=format!("{:.1$}", val, spaces), width=2);
                }
                else if i == self.shape.1 {
                    println!("{val:>width$}", val="|", width=2);
                    i=0;
                    print!("{}", "|    ");
                    print!("{val:>width$}", val=format!("{:.1$}", val, spaces), width=2);

                }
                else if i >=1 {
                    print!("{val:>width$}", val=format!("{:.1$}", val, spaces), width=spaces);
                }
                
                i+=1;
            }
            println!("{val:>width$}", val="|", width=2);
            Ok({
                print_type_of(self.values.iter().nth(0).expect("Empty values."));
                println!("({},{}) -", self.shape.0, self.shape.1)}
            )}
    }
}

 */