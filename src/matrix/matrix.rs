use crate::matrix::*;

impl Matrix {


    pub fn row(&self, i:i32) -> Matrix {
        if i<0 || i>= self.get_shape().0 {
            eprintln!("\nfn row(&self, i:i32) >>> The row indice is out of range : {} is not in [{}-{}]. \n", i, 0, self.get_shape().0-1);
            std::process::exit(-1);
        }
       else {
           match &self {
            Matrix::Int(a) =>Matrix::Int(MatrixStruct::<i32> {values:(a.values[((i*a.shape.1) as usize)..((i*a.shape.1 + a.shape.0) as usize)]).to_vec(), shape:(1,a.shape.1)}),
            Matrix::Float(a) =>Matrix::Float(MatrixStruct::<f64> {values:(a.values[((i*a.shape.1) as usize)..((i*a.shape.1 + a.shape.0) as usize)]).to_vec(), shape:(1,a.shape.1)}),
            Matrix::Bool(a) =>Matrix::Bool(MatrixStruct::<bool> {values:(a.values[((i*a.shape.1) as usize)..((i*a.shape.1 + a.shape.0) as usize)]).to_vec(), shape:(1,a.shape.1)}),
            Matrix::Null =>Matrix::Null,
            }
        }
    }
    

    pub fn col(&self, j:i32) -> Matrix {
        if j<0 || j>= self.get_shape().1 {
            eprintln!("\nfn col(&self, i:i32) >>> The col indice is out of range : {} is not in [{}-{}]. \n", j, 0, self.get_shape().0-1);
            std::process::exit(-1);
        }
        match &self {
            Matrix::Int(a) =>Matrix::Int(MatrixStruct::<i32> {values:{let mut r : Vec<i32> = Vec::with_capacity(self.get_shape().0 as usize); for chun in a.values.chunks(a.shape.0 as usize)  { r.push(chun[j as usize])} r}, shape:(a.shape.0,1)}),
            Matrix::Float(a) =>Matrix::Float(MatrixStruct::<f64> {values:{let mut r : Vec<f64> = Vec::with_capacity(self.get_shape().0 as usize); for chun in a.values.chunks(a.shape.0 as usize)  { r.push(chun[j as usize])} r}, shape:(a.shape.0,1)}),
            Matrix::Bool(a) =>Matrix::Bool(MatrixStruct::<bool> {values:{let mut r : Vec<bool> = Vec::with_capacity(self.get_shape().0 as usize); for chun in a.values.chunks(a.shape.0 as usize)  { r.push(chun[j as usize])} r}, shape:(a.shape.0,1)}),
            Matrix::Null =>Matrix::Null,
            }
    }
    

}


impl Matrix {
    pub fn convert_to(self, type_ : &str) -> Matrix {
        if type_ == "i32" {
            return match self {
                Matrix::Int(_a)=>Matrix::Bool(MatrixStruct::<bool>{ values: vec![false], shape:(1,1)}),
                Matrix::Float(a)=>Matrix::Int(MatrixStruct::<i32>{values : { let mut r:Vec<i32>= Vec::new(); for i in 0..a.values.len() {r.push(a.values[i] as i32)} r}, shape: a.shape}),
                Matrix::Bool(_a)=>Matrix::Bool(MatrixStruct::<bool>{ values: vec![false], shape:(1,1)}),
                Matrix::Null=>Matrix::Bool(MatrixStruct::<bool>{ values: vec![false], shape:(1,1)}),
            }
        }
        else if type_ == "f64" {
            return match self {
                Matrix::Int(a)=>Matrix::Float(MatrixStruct::<f64>{values : { let mut r:Vec<f64> = Vec::new(); for i in 0..a.values.len() {r.push(a.values[i] as f64)} r}, shape: a.shape}),
                Matrix::Float(_a)=>Matrix::Bool(MatrixStruct::<bool>{ values: vec![false], shape:(1,1)}),
                Matrix::Bool(_a)=>Matrix::Bool(MatrixStruct::<bool>{ values: vec![false], shape:(1,1)}),
                Matrix::Null=>Matrix::Bool(MatrixStruct::<bool>{ values: vec![false], shape:(1,1)}),
            }
        }
        else if type_ == "bool" {
            return match self {
                Matrix::Int(_a)=>Matrix::Bool(MatrixStruct::<bool>{ values: vec![false], shape:(1,1)}),
                Matrix::Float(_a)=>Matrix::Bool(MatrixStruct::<bool>{ values: vec![false], shape:(1,1)}),
                Matrix::Bool(_a)=>Matrix::Bool(MatrixStruct::<bool>{ values: vec![false], shape:(1,1)}),
                Matrix::Null=>Matrix::Bool(MatrixStruct::<bool>{ values: vec![false], shape:(1,1)}),
            }
        }
        else {
            return Matrix::Bool(MatrixStruct::<bool>{ values: vec![false], shape:(1,1)})
        }
    }

    pub fn get_shape(&self) -> (i32,i32) {
        match self {
            Matrix::Int(a)=>a.shape,
            Matrix::Float(a)=>a.shape,
            Matrix::Bool(a)=>a.shape,
            Matrix::Null=>(1,1)
        }
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

impl Matrix {
    pub fn max(&self) -> f64 {
        match &self {
            Matrix::Int(a)=>{let mut max = &a.values[0]; for el in a.values.iter()  {if max < el  { max = el;}} *max as f64},
            Matrix::Float(a)=>{let mut max = &a.values[0]; for el in a.values.iter()  {if max < el  { max = el;}} *max},
            Matrix::Bool(a)=>{let mut max = &0.0; for el in a.values.iter()  {if *el { max=&1.0 } else {max=&0.0}} *max},
            Matrix::Null=>0.0,
        }
    }
    
}




impl Matrix {
    fn is_row(&self) -> bool {
        if self.get_shape().0 == 1 {
            return true;
        }
        else {
            return false;
        }
    }
    fn is_col(&self) -> bool {
        if self.get_shape().1 == 1 {
            return true;
        }
        else {
            return false;
        }
    }
}




