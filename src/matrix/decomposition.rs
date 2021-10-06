use crate::matrix::*;
use crate::matrix::constructors::Constructors;
use num_traits::NumOps;
use num::Zero;
use num::One;

use num_rational::Ratio;

   
macro_rules! sub_impl {
    ($($t:ty)*) => ($(
        impl Matrix<$t> {
            pub fn lu_decomposition(&self) -> (Matrix<$t>,Matrix<$t>,Matrix<$t>,usize){
                core_lu_decomposition(self, -1.0)
            }
            pub fn resolve_system(&self, vect_b : &Matrix<$t>) -> Matrix<$t> {
                core_resolve_system(self, vect_b, -1.0)
            }
            pub fn invert(&self)  -> Matrix<$t> {
                core_invert(self, -1.00)
            }
            pub fn det(&self) -> $t {
                core_det(self, -1.0)
            }
        }
)*)
}
sub_impl! { f32 f64 }


macro_rules! sub_impl {
    ($($t:ty)*) => ($(
        impl Matrix<$t> {
            pub fn lu_decomposition(&self) -> (Matrix<Ratio<$t>>,Matrix<Ratio<$t>>,Matrix<Ratio<$t>>,usize){
                core_lu_decomposition(&(self.clone_to_ratio()), Ratio::new(-1,1))
            }
            pub fn resolve_system(&self, vect_b : &Matrix<$t>) -> Matrix<Ratio<$t>> {
                core_resolve_system(&(self.clone_to_ratio()), &vect_b.clone_to_ratio(),Ratio::new(-1,1))
            }
            pub fn invert(&self)  -> Matrix<Ratio<$t>> {
                core_invert(&(self.clone_to_ratio()), Ratio::new(-1,1))
            }
            pub fn det(&self) -> Ratio<$t> {
                core_det(&(self.clone_to_ratio()), Ratio::new(-1,1))
            }
        }
)*)
}
sub_impl! { i32 i64 }

macro_rules! sub_impl {
    ($($t:ty)*) => ($(
        impl Matrix<$t> {
            pub fn lu_decomposition(&self) -> (Matrix<$t>,Matrix<$t>,Matrix<$t>,usize){
                core_lu_decomposition(self, Ratio::new(-1,1))
            }
            pub fn resolve_system(&self, vect_b : &Matrix<$t>) -> Matrix<$t> {
                core_resolve_system(self, vect_b, Ratio::new(-1,1))
            }
            pub fn invert(&self)  -> Matrix<$t> {
                core_invert(self, Ratio::new(-1,1))
            }
            pub fn det(&self) -> $t {
                core_det(self, Ratio::new(-1,1))
            }
            }
)*)
}
sub_impl! { Ratio<i32> Ratio<i64>}









fn core_lu_permut<T : NumOps + Copy + PartialOrd>(matrix_a : &mut Matrix<T>, col: i32, num_piv : &mut usize, negative_identity : T) -> i32{
    let mut ind_max=col;
    for i in (col+1)..matrix_a.shape.0{
        if *matrix_a.get(ind_max,col) < *matrix_a.get(i as i32,col) && matrix_a.get(ind_max,col) > &(negative_identity * *matrix_a.get(i as i32,col)) {ind_max = i as i32;}
    }
    if ind_max!=col {*num_piv+=1}
    matrix_a.swap(col, ind_max, true);
    ind_max
}



fn core_lu_decomposition<T : NumOps + Clone + Zero + Copy + PartialOrd + One>(this : &Matrix<T>, negative_identity : T) -> (Matrix<T>,Matrix<T>,Matrix<T>,usize){
    if this.shape.0 != this.shape.1 {
        eprintln!("\nfn LU_lower_generator(permuted_matrix : &Matrix<$t>, col : i32) >>> The matrix is not square.");
        std::process::exit(-1);
    }
    else {
        let mut num_piv : usize =  0;
        let mut matrix_l = Matrix::<T>::fill(this.shape.0, this.shape.0, Zero::zero());
        let mut matrix_u = Matrix::<T>::fill(this.shape.0, this.shape.0, Zero::zero());
        let mut matrix_p = Matrix::<T>::fill_diagonal(this.shape.0, One::one());
        let mut working_mat = this.copy();
        
        for i in 0..working_mat.shape.0 {
            matrix_p.swap(i,core_lu_permut(&mut working_mat, i, &mut num_piv, negative_identity), true);
            for k in i..working_mat.shape.0 {
                let mut sum = Zero::zero();
                for j in 0..i {
                    sum = sum + *matrix_l.get(i,j) * *matrix_u.get(j,k);
                }
                *matrix_u.get_mut(i,k) = *working_mat.get(i,k) - sum;
            }
            for k in i ..working_mat.shape.0 {
                if i==k {*matrix_l.get_mut(i,i) = One::one();} else{
                    let mut sum = Zero::zero();
                    for j in 0..i {
                        sum= sum + *matrix_l.get(k,j) * *matrix_u.get(j,i);
                    }
                    *matrix_l.get_mut(k,i) = (*working_mat.get(k,i) - sum)/ *matrix_u.get(i,i);
                }
            }
        }
    (matrix_p, matrix_l,matrix_u, num_piv)
    }
}


fn core_resolve_tri_using_lu<T : NumOps + Copy + Zero + One>(this : &Matrix<T>, vect_b : &Matrix<T>, lu_result : &(Matrix<T>,Matrix<T>,Matrix<T>,usize)) -> Matrix<T> {
    let  (matrix_p,matrix_l,matrix_u,_) =lu_result;
    let mut y = Matrix::<T>::fill(this.shape.0, 1, Zero::zero());
    let b = matrix_p.dot(&vect_b);
    for ind_row in 0..y.shape.0 {
        *y.get_mut(ind_row,0)={                    
            let mut sum : T = *b.get(ind_row as i32,0);
            for k in 0..=(ind_row-1) {
                sum = sum - *matrix_l.get(ind_row as i32,k as i32)* *(y.get(k,0));
            }sum}/ *matrix_l.get(ind_row as i32,ind_row as i32);
    }

    let mut x = Matrix::<T>::fill(this.shape.0, 1, Zero::zero());
    for ind_row in (0..x.shape.0).rev() {
    *x.get_mut(ind_row,0) = {
        let mut sum : T = *y.get(ind_row as i32,0);
        for k in (ind_row +1)..x.shape.0  {
            sum = sum - *matrix_u.get(ind_row,k as i32)* *(x.get(k,0));
        }sum}/ *matrix_u.get(ind_row as i32, ind_row as i32);
    }
    x
}

fn core_resolve_system<T : Copy + Zero + One + Clone + NumOps + PartialOrd>(this : &Matrix<T>, vect_b : &Matrix<T>, negative_id :T) -> Matrix<T> {
    if  !(vect_b.is_col()) || (this.shape.1 != vect_b.shape.0) {
        eprintln!("\nfn resolve_system(matrix_a : Matrix<$t>, vect_b : Matrix<$t>) >>> Shapes are incompatible to resolve a system.");
        std::process::exit(-1);
    }
    core_resolve_tri_using_lu(this, vect_b, &core_lu_decomposition(this,negative_id))
}

fn core_invert<T : Zero + Copy + Zero + One + Clone + NumOps + PartialOrd>(this : &Matrix<T>, negative_id : T)  -> Matrix<T> {
    let lu_decomp = core_lu_decomposition(this,negative_id);
    let mut matrix_inv = Matrix::<T>::fill(0,0, Zero::zero());
    for i in 0..this.shape.0 {
        let mut vect_ei = Matrix::<T>::fill(this.shape.0,1,Zero::zero());
        *vect_ei.get_mut(i,0)=One::one();
        matrix_inv.concat(&core_resolve_tri_using_lu(this,&vect_ei, &lu_decomp));
    }
    
    matrix_inv
}

fn core_det<T : Zero + Copy + Zero + One + Clone + NumOps + PartialOrd>(this : &Matrix<T>, negative_id : T) -> T {
        if  this.shape.1 != this.shape.0 {
            eprintln!("\nfn det(&self) >>> Matrix is not square to compute the determinant.");
            std::process::exit(-1);
        }
        else {
            let (_matrix_p,matrix_l,matrix_u,num_piv) = core_lu_decomposition(this,negative_id);
            let mut val_det : T = {if (num_piv)%2 == 0 {One::one()} else {negative_id}};
            for k in 0..this.shape.0 {
                val_det = val_det * *matrix_l.get(k,k)* *matrix_u.get(k,k);
            }
            val_det
        }
}




















// LU Decomposition is only available for floats and signed integers. The integer part returns a Ratio Matrix to avoid the division of integers

/* FLOAT */


/*
macro_rules! sub_impl {
    ($($t:ty)*) => ($(
        impl Matrix<$t> {
            fn lu_permut(matrix_a : &mut Matrix<$t>, col: i32, num_piv : &mut usize) -> i32{
                let mut ind_max=col;
                for i in (col+1)..matrix_a.shape.0{
                    if *matrix_a.get(ind_max,col) < *matrix_a.get(i as i32,col) && matrix_a.get(ind_max,col) > &(-1.0 * *matrix_a.get(i as i32,col)) {ind_max = i as i32;}
                }
                if ind_max!=col {*num_piv+=1}
                matrix_a.swap(col, ind_max, true);
                ind_max
            }

            pub fn lu_decomposition(&self) -> (Matrix<$t>,Matrix<$t>,Matrix<$t>,usize){
                
                if self.shape.0 != self.shape.1 {
                    eprintln!("\nfn LU_lower_generator(permuted_matrix : &Matrix<$t>, col : i32) >>> The matrix is not square.");
                    std::process::exit(-1);
                }
                else {
                    let mut num_piv : usize =  0;
                    let mut matrix_l = Matrix::fill(self.shape.0, self.shape.0, 0.0);
                    let mut matrix_u = Matrix::fill(self.shape.0, self.shape.0, 0.0);
                    let mut matrix_p = Matrix::fill_diagonal(self.shape.0, 1.0);

                    let mut working_mat = self.copy();
                    for i in 0..working_mat.shape.0 {
                        matrix_p.swap(i,Matrix::<$t>::lu_permut(&mut working_mat, i, &mut num_piv), true);

                        
                        for k in i..working_mat.shape.0 {
                            let mut sum = 0.0;
                            for j in 0..i {
                                sum+= matrix_l.get(i,j) * matrix_u.get(j,k);
                            }
                            *matrix_u.get_mut(i,k) = working_mat.get(i,k) - sum;
                        }
                        for k in i ..working_mat.shape.0 {
                            if i==k {*matrix_l.get_mut(i,i) = 1.0;} else{
                                let mut sum = 0.0;
                                for j in 0..i {
                                    sum+= matrix_l.get(k,j) * matrix_u.get(j,i);
                                }
                                *matrix_l.get_mut(k,i) = (working_mat.get(k,i) - sum)/matrix_u.get(i,i);
                            }
                        }
                        
                    }
                (matrix_p, matrix_l,matrix_u, num_piv)
                }
            }
            fn resolve_tri_using_lu(&self, vect_b : &Matrix<$t>, lu_result : &(Matrix<$t>,Matrix<$t>,Matrix<$t>,usize)) -> Matrix<$t> {
                let  (matrix_p,matrix_l,matrix_u,_) =lu_result;

                let mut y = Matrix::<$t>::fill(self.shape.0, 1, 0 as $t);
                let b = matrix_p.dot(&vect_b);
                for ind_row in 0..y.shape.0 {
                    *y.get_mut(ind_row,0)={                    
                        let mut sum : $t = *b.get(ind_row as i32,0);
                        for k in 0..=(ind_row-1) {
                            sum -= matrix_l.get(ind_row as i32,k as i32)*(y.get(k,0));
                        }sum}/matrix_l.get(ind_row as i32,ind_row as i32);
                }

                let mut x = Matrix::<$t>::fill(self.shape.0, 1, 0 as $t);
                for ind_row in (0..x.shape.0).rev() {
                *x.get_mut(ind_row,0) = {
                    let mut sum : $t = *y.get(ind_row as i32,0);
                    for k in (ind_row +1)..x.shape.0  {
                        sum -= (matrix_u.get(ind_row,k as i32)*(x.get(k,0))) as $t;
                    }sum}/matrix_u.get(ind_row as i32, ind_row as i32);
                }
                x
            }

            pub fn resolve_system(&self, vect_b : &Matrix<$t>) -> Matrix<$t> {
                if  !(vect_b.is_col()) || (self.shape.1 != vect_b.shape.0) {
                    eprintln!("\nfn resolve_system(matrix_a : Matrix<$t>, vect_b : Matrix<$t>) >>> Shapes are incompatible to resolve a system.");
                    std::process::exit(-1);
                }
                return self.resolve_tri_using_lu(vect_b, &self.lu_decomposition())
            }

            pub fn invert(&self)  -> Matrix<$t> {
                let lu_decomp = &self.lu_decomposition();

                let mut matrix_inv = Matrix::<$t>::fill(0,0, 0.0);
                
                for i in 0..self.shape.0 {
                    let mut vect_ei = Matrix::<$t>::fill(self.shape.0,1,0.0);
                    *vect_ei.get_mut(i,0)=1.0;
                    matrix_inv.concat(&self.resolve_tri_using_lu(&vect_ei, lu_decomp));
                }
                
                matrix_inv
            }

            pub fn det(&self) -> $t {
                if  (self.shape.1 != self.shape.0) {
                    eprintln!("\nfn det(&self) >>> Matrix is not square to compute the determinant.");
                    std::process::exit(-1);
                }
                else {
                    let (_matrix_p,matrix_l,matrix_u,num_piv) = self.lu_decomposition();
                    let mut val_det : $t = {if (num_piv as $t)%2.0 == 0.0 {1.0} else {-1.0}};
                    for k in 0..self.shape.0 {
                        val_det *= matrix_l.get(k,k)*matrix_u.get(k,k);
                    }
                    (val_det*1e10).round()*1e-10
                    
                }
            }
        }
)*)
}
sub_impl! { f32 f64 }

*/



/*
use num_rational::Ratio;


/* SIGNED INTEGERS */
macro_rules! sub_impl {
    ($($t:ty)*) => ($(
        impl Matrix<$t> {
            fn lu_permut(matrix_a : &mut Matrix<Ratio<$t>>, col: i32, num_piv : &mut usize) -> i32{
                let ind_max= core_ind_max::<Ratio<$t>>(matrix_a, col, num_piv, Ratio::new(-1,1));
                matrix_a.swap(col, ind_max, true);
                ind_max
            }

            pub fn lu_decomposition(&self) -> (Matrix<Ratio<$t>>,Matrix<Ratio<$t>>,Matrix<Ratio<$t>>,usize){
                if (self.shape.0 != self.shape.1) {
                    eprintln!("\nfn LU_lower_generator(permuted_matrix : &Matrix<$t>, col : i32) >>> The matrix is not square.");
                    std::process::exit(-1);
                }
                else {
                let mut num_piv : usize =  0;
                let mut matrix_l = Matrix::<Ratio<$t>>::fill(self.shape.0, self.shape.0, Ratio::new(0, 1));
                let mut matrix_u = Matrix::<Ratio<$t>>::fill(self.shape.0, self.shape.0, Ratio::new(0, 1));
                let mut matrix_p = Matrix::<Ratio<$t>>::fill_diagonal(self.shape.0, Ratio::new(1, 1));
                let mut working_mat = self.clone_to_ratio();
                for i in 0..working_mat.shape.0 {
                    matrix_p.swap(i,Matrix::<$t>::lu_permut(&mut working_mat, i, &mut num_piv), true);
                    for k in i..working_mat.shape.0 {
                        let mut sum = Ratio::new(0, 1);
                        for j in 0..i {
                            sum+= matrix_l.get(i,j) * matrix_u.get(j,k);
                        }
                        *matrix_u.get_mut(i,k) = working_mat.get(i,k) - sum;
                    }
                    for k in i ..working_mat.shape.0 {
                        if i==k {*matrix_l.get_mut(i,i) = Ratio::new(1, 1);} else{
                            let mut sum = Ratio::new(0, 1);
                            for j in 0..i {
                                sum+= matrix_l.get(k,j) * matrix_u.get(j,i);
                            }
                            *matrix_l.get_mut(k,i) = (working_mat.get(k,i) - sum)/matrix_u.get(i,i);
                        }
                    }
                }
                (matrix_p, matrix_l,matrix_u, num_piv)
                }
            }
            fn resolve_tri_using_lu(&self, vect_b : &Matrix<Ratio<$t>>, lu_result : &(Matrix<Ratio<$t>>,Matrix<Ratio<$t>>,Matrix<Ratio<$t>>,usize)) -> Matrix<Ratio<$t>> {
                let  (matrix_p,matrix_l,matrix_u,_) =lu_result;
                
                let mut y = Matrix::<Ratio<$t>>::fill(self.shape.0, 1, Ratio::new(0, 1));
                
                let b = matrix_p.dot(vect_b);
                for ind_row in 0..y.shape.0 {
                    *y.get_mut(ind_row,0)={                    
                        let mut sum : Ratio<$t> = *b.get(ind_row as i32,0);
                        for k in 0..=(ind_row-1) {
                            sum -= matrix_l.get(ind_row as i32,k as i32)*(y.get(k,0));
                        }sum}/matrix_l.get(ind_row as i32,ind_row as i32);
                }
                
                let mut x = Matrix::<Ratio<$t>>::fill(self.shape.0, 1, Ratio::from_integer(0));
                for ind_row in (0..x.shape.0).rev() {
                *x.get_mut(ind_row,0) = {
                    let mut sum : Ratio<$t> = *y.get(ind_row as i32,0);
                    for k in (ind_row +1)..x.shape.0  {
                        sum -= (matrix_u.get(ind_row,k as i32)*(x.get(k,0)));
                    }sum}/matrix_u.get(ind_row as i32, ind_row as i32);
                }
                x
            }

            pub fn resolve_system(&self, vect_b : &Matrix<$t>) -> Matrix<Ratio<$t>> {
                if  !(vect_b.is_col()) || (self.shape.1 != vect_b.shape.0) {
                    eprintln!("\nfn resolve_system(matrix_a : Matrix<$t>, vect_b : Matrix<$t>) >>> Shapes are incompatible to resolve a system.");
                    std::process::exit(-1);
                }
                return self.resolve_tri_using_lu(&vect_b.clone_to_ratio(), &self.lu_decomposition())
            }

            pub fn invert(&self)  -> Matrix<Ratio<$t>> {
                let lu_decomp = &self.lu_decomposition();

                let mut matrix_inv = Matrix::<Ratio<$t>>::fill(0,0, Ratio::new(0, 1));
                
                for i in 0..self.shape.0 {
                    let mut vect_ei = Matrix::<Ratio<$t>>::fill(self.shape.0,1,Ratio::new(0, 1));
                    *vect_ei.get_mut(i,0)=Ratio::new(1, 1);
                    
                    matrix_inv.concat(&self.resolve_tri_using_lu(&vect_ei, lu_decomp));
                    
                }
                
                matrix_inv
            }

            pub fn det(&self) -> Ratio<$t> {
                if  (self.shape.1 != self.shape.0) {
                    eprintln!("\nfn det(&self) >>> Matrix is not square to compute the determinant.");
                    std::process::exit(-1);
                }
                else {
                    let (_matrix_p,matrix_l,matrix_u,num_piv) = self.lu_decomposition();
                    let mut val_det : Ratio<$t> = {if (num_piv as $t)%2 == 0 {Ratio::new(1, 1)} else {Ratio::new(-1, 1)}};
                    for k in 0..self.shape.0 {
                        val_det *= matrix_l.get(k,k)*matrix_u.get(k,k);
                    }
                    val_det
                }
            }
        }
)*)
}

sub_impl! { i32 i64 }



/* RATIO */
macro_rules! sub_impl {
    ($($t:ty)*) => ($(
        impl Matrix<$t> {
            fn lu_permut(matrix_a : &mut Matrix<$t>, col: i32, num_piv : &mut usize) -> i32{
                let ind_max= core_ind_max::<$t>(matrix_a, col, num_piv, Ratio::new(-1,1));
                matrix_a.swap(col, ind_max, true);
                ind_max
            }

            pub fn lu_decomposition(&self) -> (Matrix<$t>,Matrix<$t>,Matrix<$t>,usize){
                if (self.shape.0 != self.shape.1) {
                    eprintln!("\nfn LU_lower_generator(permuted_matrix : &Matrix<$t>, col : i32) >>> The matrix is not square.");
                    std::process::exit(-1);
                }
                else {
                let mut num_piv : usize =  0;
                let mut matrix_l = Matrix::<$t>::fill(self.shape.0, self.shape.0, Ratio::new(0, 1));
                let mut matrix_u = Matrix::<$t>::fill(self.shape.0, self.shape.0, Ratio::new(0, 1));
                let mut matrix_p = Matrix::<$t>::fill_diagonal(self.shape.0, Ratio::new(1, 1));
                let mut working_mat = self.copy();
                for i in 0..working_mat.shape.0 {
                    matrix_p.swap(i,Matrix::<$t>::lu_permut(&mut working_mat, i, &mut num_piv), true);
                    for k in i..working_mat.shape.0 {
                        let mut sum = Ratio::new(0, 1);
                        for j in 0..i {
                            sum+= matrix_l.get(i,j) * matrix_u.get(j,k);
                        }
                        *matrix_u.get_mut(i,k) = working_mat.get(i,k) - sum;
                    }
                    for k in i ..working_mat.shape.0 {
                        if i==k {*matrix_l.get_mut(i,i) = Ratio::new(1, 1);} else{
                            let mut sum = Ratio::new(0, 1);
                            for j in 0..i {
                                sum+= matrix_l.get(k,j) * matrix_u.get(j,i);
                            }
                            *matrix_l.get_mut(k,i) = (working_mat.get(k,i) - sum)/matrix_u.get(i,i);
                        }
                    }
                }
                (matrix_p, matrix_l,matrix_u, num_piv)
                }
            }
            fn resolve_tri_using_lu(&self, vect_b : &Matrix<$t>, lu_result : &(Matrix<$t>,Matrix<$t>,Matrix<$t>,usize)) -> Matrix<$t> {
                let  (matrix_p,matrix_l,matrix_u,_) =lu_result;
                
                let mut y = Matrix::<$t>::fill(self.shape.0, 1, Ratio::new(0, 1));
                
                let b = matrix_p.dot(vect_b);
                for ind_row in 0..y.shape.0 {
                    *y.get_mut(ind_row,0)={                    
                        let mut sum : $t = *b.get(ind_row as i32,0);
                        for k in 0..=(ind_row-1) {
                            sum -= matrix_l.get(ind_row as i32,k as i32)*(y.get(k,0));
                        }sum}/matrix_l.get(ind_row as i32,ind_row as i32);
                }
                
                let mut x = Matrix::<$t>::fill(self.shape.0, 1, Ratio::from_integer(0));
                for ind_row in (0..x.shape.0).rev() {
                *x.get_mut(ind_row,0) = {
                    let mut sum : $t = *y.get(ind_row as i32,0);
                    for k in (ind_row +1)..x.shape.0  {
                        sum -= (matrix_u.get(ind_row,k as i32)*(x.get(k,0)));
                    }sum}/matrix_u.get(ind_row as i32, ind_row as i32);
                }
                x
            }

            pub fn resolve_system(&self, vect_b : &Matrix<$t>) -> Matrix<$t> {
                if  !(vect_b.is_col()) || (self.shape.1 != vect_b.shape.0) {
                    eprintln!("\nfn resolve_system(matrix_a : Matrix<$t>, vect_b : Matrix<$t>) >>> Shapes are incompatible to resolve a system.");
                    std::process::exit(-1);
                }
                return self.resolve_tri_using_lu(&vect_b, &self.lu_decomposition())
            }

            pub fn invert(&self)  -> Matrix<$t> {
                let lu_decomp = &self.lu_decomposition();

                let mut matrix_inv = Matrix::<$t>::fill(0,0, Ratio::new(0, 1));
                
                for i in 0..self.shape.0 {
                    let mut vect_ei = Matrix::<$t>::fill(self.shape.0,1,Ratio::new(0, 1));
                    *vect_ei.get_mut(i,0)=Ratio::new(1, 1);
                    
                    matrix_inv.concat(&self.resolve_tri_using_lu(&vect_ei, lu_decomp));
                    
                }
                
                matrix_inv
            }

            pub fn det(&self) -> $t {
                if  (self.shape.1 != self.shape.0) {
                    eprintln!("\nfn det(&self) >>> Matrix is not square to compute the determinant.");
                    std::process::exit(-1);
                }
                else {
                    let (_matrix_p,matrix_l,matrix_u,num_piv) = self.lu_decomposition();
                    let mut val_det : $t = {if (num_piv)%2 == 0 {Ratio::new(1, 1)} else {Ratio::new(-1, 1)}};
                    for k in 0..self.shape.0 {
                        val_det *= matrix_l.get(k,k)*matrix_u.get(k,k);
                    }
                    val_det
                }
            }
        }
)*)
}

sub_impl! { Ratio<i32> Ratio<i64> }



*/







