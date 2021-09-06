use crate::matrix::*;

















impl<T> Matrix<T> {
    pub fn row_iter(&self, i:i32) ->  std::slice::Iter<'_,T>  {
        if i<0 || i>= self.shape.0 {
            eprintln!("\nfn row(&self, i:i32) >>> The row indice is out of range : {} is not in [{}-{}]. \n", i, 0, self.shape.0-1);
            std::process::exit(-1);
        }
       else {
            return self.values[((i*self.shape.1) as usize)..((i*self.shape.1 + self.shape.0) as usize)].iter()
        }
    }

    pub fn row_iter_mut(&mut self, i:i32) ->  std::slice::IterMut<'_,T>  {
        if i<0 || i>= self.shape.0 {
            eprintln!("\nfn row(&self, i:i32) >>> The row indice is out of range : {} is not in [{}-{}]. \n", i, 0, self.shape.0-1);
            std::process::exit(-1);
        }
       else {
            return (self.values[((i*self.shape.1) as usize)..((i*self.shape.1 + self.shape.0) as usize)]).iter_mut()
        }
    }
}


impl<T: std::clone::Clone> Matrix<T> {


    pub fn row(&self, i:i32) -> Matrix<T> {
        if i<0 || i>= self.shape.0 {
            eprintln!("\nfn row(&self, i:i32) >>> The row indice is out of range : {} is not in [{}-{}]. \n", i, 0, self.shape.0-1);
            std::process::exit(-1);
        }
       else {
            Matrix::<T> {values:(self.values[((i*self.shape.1) as usize)..((i*self.shape.1 + self.shape.0) as usize)]).to_vec(), shape:(1,self.shape.1)}
        }
    }
    
    /*
    pub fn col(&self, j:i32) -> Matrix<T> {
        if j<0 || j>= self.shape.1 {
            eprintln!("\nfn col(&self, i:i32) >>> The col indice is out of range : {} is not in [{}-{}]. \n", j, 0, self.shape.0-1);
            std::process::exit(-1);
        }
        else {
            Matrix::<T> {values:{let mut r : Vec<T> = Vec::with_capacity(self.shape.0 as usize); for chun in self.values.chunks(self.shape.0 as usize)  { r.push(chun[j as usize])} r}, shape:(1,self.shape.1)}
        }
    }
    */
}


impl Matrix<i32> {
    pub fn convert_to_float(self, type_ : &str) -> Matrix<f64> {
        Matrix::<f64>{values :{ let mut r:Vec<f64>= Vec::new(); for i in 0..self.values.len() {r.push(self.values[i] as f64)} r}, shape: self.shape}
    }
}


/*

impl Matrix {
    
    pub fn sprod(&self, m : &Matrix){
        if !(self.is_col() || self.is_row()) {

        }
        else if !(m.is_col() || m.is_row()){

        }
        else {

        }

    }
    
    pub fn dot(&self, m: &Matrix) -> Matrix {
        if self.get_shape().1 != m.get_shape().0 {
            eprintln!("\nfn dot(&self, M: &Matrix) >>> The shapes are not compatible for matrix product.\n");
            std::process::exit(-1);
        }

        let len = (self.get_shape().1* m.get_shape().0) as usize;
        match &self {
            Matrix::Int(b)=>{
                match m {
                Matrix::Int(a)=>{
                    Matrix::Int(MatrixStruct::<i32>{values:{let mut r : Vec<i32> = Vec::with_capacity(len);
                        for i in 0..self.get_shape().0 {
                            for j in 0..self.get_shape().1 {
                                r.push(self.row(i).sprod(&m.col(j)) as i32);
                            }
                        }
                    r},
                    shape:(self.get_shape().0, m.get_shape().1)})
                },
                Matrix::Float(a)=>{Matrix::Float(MatrixStruct::<f64>{values:{let mut r : Vec<f64> = Vec::with_capacity(len);
                    for i in 0..self.get_shape().0 {
                        for j in 0..self.get_shape().1 {
                            r.push(self.row(i).sprod(&m.col(j)));
                        }
                    }
                r},
                shape:(self.get_shape().0, m.get_shape().1)})
            },
                Matrix::Bool(a)=>Matrix::Null,
                Matrix::Null=>Matrix::Null,
            }},
            Matrix::Float(b)=>{
                match m {
                Matrix::Int(a)=> Matrix::Float(MatrixStruct::<f64>{values:{let mut r : Vec<f64> = Vec::with_capacity(len);
                    for i in 0..self.get_shape().0 {
                        for j in 0..self.get_shape().1 {
                            r.push(self.row(i).sprod(&m.col(j)));
                        }
                    }
                r},
                shape:(self.get_shape().0, m.get_shape().1)}),

                Matrix::Float(a)=> Matrix::Float(MatrixStruct::<f64>{values:{let mut r : Vec<f64> = Vec::with_capacity(len);
                    for i in 0..self.get_shape().0 {
                        for j in 0..self.get_shape().1 {
                            r.push(self.row(i).sprod(&m.col(j)));
                        }
                    }
                r},
                shape:(self.get_shape().0, m.get_shape().1)}),
            
                Matrix::Bool(a)=>Matrix::Null,
                Matrix::Null=>Matrix::Null,}
            },
            Matrix::Bool(b)=>Matrix::Null,
            Matrix::Null=>Matrix::Null,
        }
    }
}

*/

impl<T : std::cmp::PartialOrd<T>> Matrix<T> {
    pub fn max(&self) -> &T {
        let mut max = &self.values[0]; 
        for el in self.values.iter()  {if max < el  { max = el;}}
        max
    }
}





impl<T> Matrix<T> {
    fn is_row(&self) -> bool {
        if self.shape.0 == 1 {
            return true;
        }
        else {
            return false;
        }
    }
    fn is_col(&self) -> bool {
        if self.shape.1 == 1 {
            return true;
        }
        else {
            return false;
        }
    }
}




