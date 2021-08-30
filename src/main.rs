use std::convert::TryInto;
use std::fmt::Display;
use std::ops::Mul;







struct Matrix<T> {
    values: Vec<T>,
    shape : (i32,i32),

}

/* Constructors */
impl<T: Clone> Matrix<T> {
    fn zeros(shape1 : i32, shape2 : i32) -> Matrix<i32>{
        Matrix::<i32>{
            values: vec![0; (shape1*shape2).try_into().unwrap()],
            shape : (shape1,shape2),
            }  
        }
    fn ones(shape1 : i32, shape2 : i32) -> Matrix<i32>{
        Matrix::<i32>{
            values: vec![1; (shape1*shape2).try_into().unwrap()],
            shape : (shape1,shape2),
            }  
        }
        
    fn eye(shape : i32) -> Matrix<i32>{
        Matrix::<i32>{
            values:
            {let mut value : Vec<i32> = Vec::new();
                for i in 0..shape{
                    for j in 0..shape{
                        if i==j {
                            value.push(1);
                        }
                        else {
                            value.push(0);
                        }     
                    }
                }
            value},
            shape : (shape,shape),
            }  
        
        }
        
    
}

/* General methods for Matrix */
impl<T: Display> Matrix<T> {
    fn show(&self) {
        let mut i=0;
        println!("");
        print!("| ");
        for val in &self.values {      
            if i == self.shape.1 {
                println!("|");
                print!("| ");
                i=0
            }
            print!("{} ", val);
            i+=1;
        }
        println!("|");
        
    }
}

impl<T> Matrix<T> {
    fn loc(&self, i: i32, j : i32) -> Option<&T>{
        if i>(self.shape.0-1) || j> (self.shape.1-1 ){
            return None;
        }
        else {
            return self.values.get((i*self.shape.1 + j) as usize);
        }
    }
}





impl<T: Copy +  Clone + std::ops::Add<Output = T>  + std::ops::Mul<Output = T>> Matrix<T> {
    fn add(&self, Vec1 : Matrix<T>) -> Matrix<T> {
        Matrix::<T>{
        values:
            {
            let mut r : Vec<T> = Vec::new();
            for i in 0..self.values.len() {
                r.push(Vec1.values[i] + self.values[i]);
            }
            r},
        shape:self.shape,
        }
    }
    fn mul(&self, scal : T) -> Matrix<T> 
    where T: std::ops::Mul<Output=T>{
        Matrix::<T>{
        values:
            {
            let mut r : Vec<T> = Vec::new();
            for i in 0..self.values.len() {
                r.push(scal*self.values[i]);
            }
            r},
        shape:self.shape,
        }
    }

    fn minus(&self, Vec1 : Matrix<T>) -> Matrix<T> {
       self.add(Vec1.mul(T))
    }

}




fn main() {
    let mut v = Matrix::<f64>::eye(6);
    let mut u = Matrix::<i32>::ones(6,6);
    (v.add(u)).mul(2).show();




    
}
