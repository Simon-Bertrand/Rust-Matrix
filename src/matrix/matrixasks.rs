use num_traits::Zero;
use crate::matrix::*;


pub trait MatrixAsks {
    fn is_row(&self) -> bool;
    fn is_col(&self) -> bool;
    fn has_full_zeros_in_rows(&self) -> bool;
    fn has_full_zeros_in_cols(&self) -> bool;
}

impl<T : PartialEq + Zero> MatrixAsks for Matrix<T> {
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
    fn has_full_zeros_in_rows(&self) -> bool {
        core_has_full_zeros_in_row_or_col(self, true)
    }
     fn has_full_zeros_in_cols(&self) -> bool {
        core_has_full_zeros_in_row_or_col(self, false)
    }
}






fn core_has_full_zeros_in_row_or_col<T : PartialEq + Zero>(this :&Matrix<T>, axe : bool) -> bool {
    if axe {
        for i in 0..this.shape.0 {
            let mut compt : usize = 0;
            for col_el in this.row_iter(i) {
                if *col_el == Zero::zero() {
                    compt += 1;
                    if compt == this.shape.1 {
                        return true
                    }
                }

            }
        }
        return false
    } 
    else {
        for j in 0..this.shape.1 {
            let mut compt : usize = 0;
            for row_el in this.col_iter(j) {
                if *row_el == Zero::zero() {
                    compt += 1;
                    if compt == this.shape.0 {
                        return true
                    }
                }
            }       
        }
        return false
    }
}