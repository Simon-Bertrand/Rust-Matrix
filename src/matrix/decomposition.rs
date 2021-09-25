use crate::matrix::*;
use crate::matrix::constructors::Constructors;

macro_rules! sub_impl {
    ($($t:ty)*) => ($(
        impl Matrix<$t> {
            pub fn lu_decomposition(&self) -> (Matrix<$t>,Matrix<$t>){
                if (self.shape.0 != self.shape.1) {
                    eprintln!("\nfn LU_lower_generator(permuted_matrix : &Matrix<$t>, col : i32) >>> The matrix is not square.");
                    std::process::exit(-1);
                }
                else {
                let mut L = Matrix::<$t>::fill(self.shape.0, self.shape.0, 0 as $t);
                let mut U = Matrix::<$t>::fill(self.shape.0, self.shape.0, 0 as $t);
                for i in 0..self.shape.0 {
                    for k in i..self.shape.0 {
                        let mut sum = 0.0;
                        for j in 0..i {
                            sum+= L.get(i,j) * U.get(j,k);
                        }
                        *U.get_mut(i,k) = self.get(i,k) - sum;
                    }
                    for k in i ..self.shape.0 {
                        if i==k {*L.get_mut(i,i) = 1.0;} else{
                            let mut sum = 0.0;
                            for j in 0..i {
                                sum+= L.get(k,j) * U.get(j,i);
                            }
                            *L.get_mut(k,i) = (self.get(k,i) - sum)/U.get(i,i);
                        }
                    }
                }
                (L,U)
                }
            }
            pub fn resolve_system(&self, vect_b : &Matrix<$t>) -> Matrix<$t> {
                if  !(vect_b.is_col()) || (self.shape.1 != vect_b.shape.0) {
                    eprintln!("\nfn resolve_system(matrix_a : Matrix<$t>, vect_b : Matrix<$t>) >>> Shapes are incompatible to resolve a system.");
                    std::process::exit(-1);
                }

                let (L,U) = self.lu_decomposition();

                let mut y = Matrix::<$t>::fill(self.shape.0, 1, 0 as $t);
                for ind_row in 0..vect_b.shape.0 {
                    
                    let mut sum : $t = *vect_b.get(ind_row as i32,0);
                    for k in 0..(ind_row-1) {
                        sum -= L.get(ind_row as i32,k as i32)*(y.get(k,0));
                    }
                    *y.get_mut(ind_row,0)=sum/L.get(ind_row as i32,ind_row as i32);
                }
                L.show();
                y.show();
                L.dot(&y).show();
                /*
                let mut x = Matrix::<$t>::fill(self.shape.0, 1, 0 as $t);
                for ind_row in 0..x.shape.0 {
                    
                    let mut sum : $t = *y.get(x.shape.0-1 - ind_row,0);
                    for k in (x.shape.0-1 - ind_row -1)..0 {
                        sum -= U.get(x.shape.0-1 - ind_row ,x.shape.0-1 - k)*(x.get(x.shape.0-1 - k,0));
                    }
                    println!("Matrix x {}", x);
                     *x.get_mut(x.shape.0 -1 - ind_row,0)=sum/U.get(x.shape.0-1 - ind_row ,x.shape.0-1 - ind_row); */
                
                

                y
            }
        }
)*)
}

sub_impl! {f32 f64 }






















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
                

                
                # Python3 Program to decompose
# a matrix into lower and
# upper triangular matrix
MAX = 100
 
 
def luDecomposition(mat, n):
 
    lower = [[0 for x in range(n)]
             for y in range(n)]
    upper = [[0 for x in range(n)]
             for y in range(n)]
 
    # Decomposing matrix into Upper
    # and Lower triangular matrix
    for i in range(n):
 
        # Upper Triangular
        for k in range(i, n):
 
            # Summation of L(i, j) * U(j, k)
            sum = 0
            for j in range(i):
                sum += (lower[i][j] * upper[j][k])
 
            # Evaluating U(i, k)
            upper[i][k] = mat[i][k] - sum
 
        # Lower Triangular
        for k in range(i, n):
            if (i == k):
                lower[i][i] = 1  # Diagonal as 1
            else:
 
                # Summation of L(k, j) * U(j, i)
                sum = 0
                for j in range(i):
                    sum += (lower[k][j] * upper[j][i])
 
                # Evaluating L(k, i)
                lower[k][i] = int((mat[k][i] - sum) /
                                  upper[i][i])
 
    # setw is for displaying nicely
    print("Lower Triangular\t\tUpper Triangular")
 
    # Displaying the result :
    for i in range(n):
 
        # Lower
        for j in range(n):
            print(lower[i][j], end="\t")
        print("", end="\t")
 
        # Upper
        for j in range(n):
            print(upper[i][j], end="\t")
        print("")
        */


       
            
 

