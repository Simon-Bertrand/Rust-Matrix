use crate::matrix::*;
use crate::matrix::constructors::Constructors;


macro_rules! sub_impl {
    ($($t:ty)*) => ($(
        impl Matrix<$t> {
            pub fn round(mut self) -> Self{
                for el in self.values.iter_mut() {
                    println!("{}", el.abs());
                    if el.abs() < 1e-10 {
                        *el = 0.0;
                    }
                    else {
                        *el = (*el as $t * 1e10).round()*1e-10;
                    }   
                }
                self
            }
            fn lu_permut(matrix_a : &mut Matrix<$t>, col: i32, num_piv : &mut usize) -> i32{
                let mut ind_max=col;
                for i in (col+1)..matrix_a.shape.0{
                    if matrix_a.get(ind_max,col).abs()< matrix_a.get(i as i32,col).abs() {ind_max = i as i32;}
                }
                if (ind_max!=col) {*num_piv+=1}
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
                let mut matrix_l = Matrix::<$t>::fill(self.shape.0, self.shape.0, 0 as $t);
                let mut matrix_u = Matrix::<$t>::fill(self.shape.0, self.shape.0, 0 as $t);
                let mut matrix_p = Matrix::<$t>::fill_diagonal(self.shape.0, 1 as $t);
                let mut working_mat = self.copy();
                for i in 0..working_mat.shape.0 {
                    matrix_p.swap(i,Matrix::<$t>::lu_permut(&mut working_mat, i, &mut num_piv), true);
                    for k in i..working_mat.shape.0 {
                        let mut sum = 0 as $t;
                        for j in 0..i {
                            sum+= matrix_l.get(i,j) * matrix_u.get(j,k);
                        }
                        *matrix_u.get_mut(i,k) = working_mat.get(i,k) - sum;
                    }
                    for k in i ..working_mat.shape.0 {
                        if i==k {*matrix_l.get_mut(i,i) = 1 as $t;} else{
                            let mut sum = 0 as $t;
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
            fn resolve_system_using_lu(&self, vect_b : &Matrix<$t>, lu_result : &(Matrix<$t>,Matrix<$t>,Matrix<$t>,usize)) -> Matrix<$t> {
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
                return self.resolve_system_using_lu(vect_b, &self.lu_decomposition())
            }

            pub fn invert(&self)  -> Matrix<$t> {
                let lu_decomp = &self.lu_decomposition();

                let mut matrix_inv = Matrix::<$t>::fill(0,0, 0.0);
                
                for i in 0..self.shape.0 {
                    let mut vect_ei = Matrix::<$t>::fill(self.shape.0,1,0.0);
                    *vect_ei.get_mut(i,0)=1.0;
                    matrix_inv.concat(&self.resolve_system_using_lu(&vect_ei, lu_decomp));
                }
                
                matrix_inv
            }

            pub fn det(&self) -> $t {
                if  (self.shape.1 != self.shape.0) {
                    eprintln!("\nfn det(&self) >>> Matrix is not square to compute the determinant.");
                    std::process::exit(-1);
                }
                else {
                    let (matrix_p,matrix_l,matrix_u,num_piv) = self.lu_decomposition();
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






















            /*
            pub fn lu_permutation_generator(A : &Matrix<$t>, col : i32) -> (Matrix<$t>,&$t) {
                if (A.shape.0 != A.shape.1) {
                    eprintln!("\nfn LU_permutation_generator(A : Matrix<T>, col : i32) >>> The matrix is not square.");
                    std::process::exit(-1);
                }

                let mut abs_max : (usize, &$t) = (0,A.get(col,col));
                let mut permut = Matrix::fill(A.shape.0, A.shape.0, 0 as $t);
        
                for (i,el) in A.col_iter(col).enumerate() {
                    if abs_max.1.abs() < *el && i as i32 >= col {
                        abs_max=(i,el);
                    }
                }

                *permut.get_mut(col, abs_max.0 as i32)=1 as $t;
                *permut.get_mut(abs_max.0 as i32, col)=1 as $t;
                for k in 0..A.shape.0 {
                    if k != col && k!= abs_max.0 as i32{
                        *permut.get_mut(k,k) = 1 as $t
                    }
                    
                }
                (permut, abs_max.1)
            }
            pub fn lu_lower_generator((permuted_matrix,piv) : (&Matrix<$t>, &$t), col : i32) -> Matrix<$t>{
                if (permuted_matrix.shape.0 != permuted_matrix.shape.1) {
                    eprintln!("\nfn LU_lower_generator(permuted_matrix : &Matrix<$t>, col : i32) >>> The matrix is not square.");
                    std::process::exit(-1);
                }

                let mut permut = Matrix::fill_diagonal(permuted_matrix.shape.0, 1 as $t);
                for (i,el) in permut.col_iter_mut(col).enumerate() {
                    if i as i32 > col {
                        *el = -permuted_matrix.get(i as i32,col)/piv
                    }
                }
                permut
            }
            */

            

        



                /*

                let start_matrix = &self;
                let (mut matrix_pi,mut piv) = Matrix::<$t>::lu_permutation_generator(start_matrix, 0);
                let mut matrix_p = matrix_pi;

                let mut matrix_li =Matrix::<$t>::lu_lower_generator((&matrix_p.dot(start_matrix),piv), 0);

                let mut matrix_u = matrix_li.dot(&matrix_p.dot(start_matrix));

                for col in 1..(self.shape.1-1) {
                    let tuples = Matrix::<$t>::lu_permutation_generator(&matrix_u, col);
                    matrix_pi = tuples.0;
                    piv = tuples.1;

                    matrix_p = matrix_pi.dot(&matrix_p);

                    matrix_li =Matrix::<$t>::lu_lower_generator((&matrix_pi.dot(&matrix_u),piv), col);
    

                    matrix_u = matrix_li.dot(&matrix_pi.dot(&matrix_u))
                }
               
                 The li matrix returned is the last computed. This has to be expanded when the matrix inverse function will be implemented. 
                (matrix_p, matrix_li, matrix_u)
                

           */

       
            
 

