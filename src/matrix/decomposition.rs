use crate::matrix::*;
use crate::matrix::constructors::Constructors;
use num_traits::NumOps;
use num_traits::Zero;
use num_traits::One;

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
            pub fn is_invertible(&self)  -> bool {
                !core_lu_decomposition(self,-1.0).2.has_full_zeros_in_rows()
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
            pub fn is_invertible(&self)  -> bool {
                !core_lu_decomposition(&self.clone_to_ratio(),Ratio::new(-1,1)).2.has_full_zeros_in_rows()
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
            pub fn is_invertible(&self)  -> bool {
                !core_lu_decomposition(self, Ratio::new(-1,1)).2.has_full_zeros_in_rows()
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









fn core_lu_permut<T : NumOps + Copy + PartialOrd>(matrix_a : &mut Matrix<T>, col: usize, num_piv : &mut usize, negative_identity : T) -> usize{
    let mut ind_max=col;
    for i in (col+1)..matrix_a.shape.0{
        if *matrix_a.get(ind_max,col) < *matrix_a.get(i,col) && matrix_a.get(ind_max,col) > &(negative_identity * *matrix_a.get(i,col)) {ind_max = i;}
    }
    if ind_max!=col {*num_piv+=1}
    matrix_a.swap(col, ind_max, true);
    ind_max
}



fn core_lu_decomposition<T : NumOps + Clone + Zero + Copy + PartialOrd + One>(this : &Matrix<T>, negative_identity : T) -> (Matrix<T>,Matrix<T>,Matrix<T>,usize){
    if this.shape.0 != this.shape.1 {
        exception::raise_exception(
            &"LU_lower_generator",
            &mut String::from("The matrix is not square."),
            String::from("Choose matrix A such as A.shape.0==A.shape.1"),
            75,
            20001);
            panic!();
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
            let mut sum : T = *b.get(ind_row,0);

            for k in 0..=({if ind_row != 0{ind_row-1} else {0}}){
                
                sum = sum - *matrix_l.get(ind_row,k)* *(y.get(k,0));
            }sum}/ *matrix_l.get(ind_row,ind_row);
    }

    let mut x = Matrix::<T>::fill(this.shape.0, 1, Zero::zero());
    for ind_row in (0..x.shape.0).rev() {
    *x.get_mut(ind_row,0) = {
        let mut sum : T = *y.get(ind_row,0);
        for k in (ind_row +1)..x.shape.0  {
            sum = sum - *matrix_u.get(ind_row,k)* *(x.get(k,0));
        }sum}/ *matrix_u.get(ind_row, ind_row);
    }
    x
}

fn core_resolve_system<T : Copy + Zero + One + Clone + NumOps + PartialOrd>(this : &Matrix<T>, vect_b : &Matrix<T>, negative_id :T) -> Matrix<T> {
    if  !(vect_b.is_col()) || (this.shape.1 != vect_b.shape.0) {
        exception::raise_exception(
            &"core_resolve_system",
            &mut String::from("Shapes are incompatible to resolve a system."),
            String::from("Choose matrix A and a vector B such as A.shape == (n,n) and B.shape ==(n,1)"),
            75,
            20002);
            panic!();
    }
    core_resolve_tri_using_lu(this, vect_b, &core_lu_decomposition(this,negative_id))
}

fn core_invert<T : std::fmt::Display +  Zero + Copy + Zero + One + Clone + NumOps + PartialOrd>(this : &Matrix<T>, negative_id : T)  -> Matrix<T> {
    let lu_decomp = core_lu_decomposition(this,negative_id);
    if !lu_decomp.2.has_full_zeros_in_rows(){
        let mut matrix_inv = Matrix::<T>::fill(0,0, Zero::zero());
        for i in 0..this.shape.0 {
            let mut vect_ei = Matrix::<T>::fill(this.shape.0,1,Zero::zero());
            *vect_ei.get_mut(i,0)=One::one();

            matrix_inv.concat(&core_resolve_tri_using_lu(this,&vect_ei, &lu_decomp));
        }
        matrix_inv
    }
    else {
        exception::raise_exception(
            &"core_invert",
            &mut String::from("Matrix is a singular one. Inversion is not possible."),
            String::from("Choose a matrix A that is not singular."),
            75,
            20003);
            panic!();

    }
    
}

fn core_det<T : Zero + Copy + Zero + One + Clone + NumOps + PartialOrd>(this : &Matrix<T>, negative_id : T) -> T {
        if  this.shape.1 != this.shape.0 {
            exception::raise_exception(
                &"core_det",
                &mut String::from("Matrix is not square to compute the determinant."),
                String::from("Choose a matrix A such as A.shape.0==A.shape.1"),
                75,
                20004);
                panic!();
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


