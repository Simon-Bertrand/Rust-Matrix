use std::ops::Mul;
use std::ops::Add;
use std::ops::Sub;
use std::ops::Div;
use std::cmp::PartialEq;

use crate::generic::*;
use num::Zero;

impl<T : NumFloat + NumInteger + NumBool + NumRatio + Zero, U : NumFloat + NumInteger + NumBool + NumRatio + Zero> Mul<&DataType<U>> for &DataType<T> 
{
    type Output = DataType<T>; 
    fn mul(self, rhs: &DataType<U>) -> DataType<T> {
        match (self,rhs) {
            (DataType::Int(a),DataType::Int(b))=> return DataType::Int(DataTypeInteger::<T>{v: Zero::zero()}),
            (DataType::Float(a),DataType::Float(b)) => return DataType::Float(DataTypeFloat::<T>{v: Zero::zero()}),
            (DataType::Ratio(a),DataType::Ratio(b)) => return DataType::Ratio(DataTypeRatio::<T>{v: Zero::zero()}),
            (DataType::Bool(a),DataType::Bool(b)) => return DataType::Bool(DataTypeBool::<T>{v: Zero::zero()}),

            (DataType::Int(a),DataType::Float(b)) => return DataType::Float(DataTypeFloat::<T>{v: Zero::zero()}),
            (DataType::Float(a),DataType::Int(b)) => return DataType::Float(DataTypeFloat::<T>{v: Zero::zero()}),

            (DataType::Int(a),DataType::Ratio(b)) => return DataType::Ratio(DataTypeRatio::<T>{v: Zero::zero()}),
            (DataType::Ratio(a),DataType::Int(b)) => return DataType::Ratio(DataTypeRatio::<T>{v: Zero::zero()}),

            (DataType::Int(a),DataType::Bool(b)) => return DataType::Int(DataTypeInteger::<T>{v: Zero::zero()}),
            (DataType::Bool(a),DataType::Int(b)) => return DataType::Int(DataTypeInteger::<T>{v: Zero::zero()}),

            (DataType::Float(a),DataType::Bool(b)) => return DataType::Float(DataTypeFloat::<T>{v: Zero::zero()}),
            (DataType::Bool(a),DataType::Float(b)) => return DataType::Float(DataTypeFloat::<T>{v: Zero::zero()}),

            (DataType::Float(a),DataType::Ratio(b)) => return DataType::Float(DataTypeFloat::<T>{v: Zero::zero()}),
            (DataType::Ratio(a),DataType::Float(b)) => return DataType::Float(DataTypeFloat::<T>{v: Zero::zero()}),

            (DataType::Bool(a),DataType::Ratio(b)) => return DataType::Ratio(DataTypeRatio::<T>{v: Zero::zero()}),
            (DataType::Ratio(a),DataType::Bool(b)) => return DataType::Ratio(DataTypeRatio::<T>{v: Zero::zero()}),

        }
}
}
/*
impl<T : NumFloat + NumInteger + NumBool + NumRatio + Zero, U : NumFloat + NumInteger + NumBool + NumRatio + Zero> Mul<&DataType<U>> for &DataType<T> 
{
    type Output = DataType<T>; 
    fn mul(self, rhs: &DataType<U>) -> DataType<T> {
        if let (DataType::Int(a),DataType::Int(b)) = (self,rhs)  {
            return DataType::Int(DataTypeInteger::<T>{v: Zero::zero()})
        }
        else if let (DataType::Float(a),DataType::Float(b)) = (self,rhs)  {
            return DataType::Float(DataTypeFloat::<T>{v: Zero::zero()})
        }
        else if let (DataType::Ratio(a),DataType::Ratio(b)) = (self,rhs) {
            return DataType::Ratio(DataTypeRatio::<T>{v: Zero::zero()})
        }
        else if let (DataType::Bool(a),DataType::Bool(b)) = (self,rhs)   {
            return DataType::Bool(DataTypeBool::<T>{v: Zero::zero()})
        }
        else if let (DataType::Int(a),DataType::Float(b)) = (self,rhs) | if let (DataType::Int(a),DataType::Float(b)) = (rhs,self)  {
            return DataType::Float(DataTypeFloat::<T>{v: Zero::zero()})
        }
        else if let (DataType::Int(a),DataType::Bool(b)) = (self,rhs) | if let (DataType::Int(a),DataType::Bool(b)) = (rhs,self) {
            return DataType::Int(DataTypeBool::<T>{v: Zero::zero()})
        }
        else if let (DataType::Float(a),DataType::Bool(b)) = (self,rhs) |  (DataType::Float(a),DataType::Bool(b)) = (rhs,self) {
            return DataType::Float(DataTypeFloat::<T>{v: Zero::zero()})      
        }
}
}
*/
/*
impl<T : NumFloat + NumInteger + NumBool + NumRatio + Zero, U : NumFloat + NumInteger + NumBool + NumRatio + Zero> Mul<&DataType<T>> for &DataType<U> 
{
    type Output = DataType<T>; 
    fn mul(self, rhs: &DataType<U>) -> DataType<T> {
        if let (DataType::Int(a),DataType::Int(b)) = (self,rhs)  {
            return DataType::Int(DataTypeInteger::<T>{v: Zero::zero()})
        }
        else if let (DataType::Float(a),DataType::Float(b)) = (self,rhs)  {
            return DataType::Float(DataTypeFloat::<T>{v: Zero::zero()})
        }
        else if let (DataType::Ratio(a),DataType::Ratio(b)) = (self,rhs)  {
            return DataType::Ratio(DataTypeRatio::<T>{v: Zero::zero()})
        }
        else if let (DataType::Bool(a),DataType::Bool(b)) = (self,rhs)  {
            return DataType::Ratio(DataTypeRatio::<T>{v: Zero::zero()})
        }
        else if let (DataType::Int(a),DataType::Float(b)) = (self,rhs)  {
            return DataType::Int(DataTypeInteger::<T>{v: Zero::zero()})
        }
        else if let (DataType::Int(a),DataType::Bool(b)) = (self,rhs) {
            return DataType::Int(DataTypeInteger::<T>{v: Zero::zero()})
        }
        else if let (DataType::Float(a),DataType::Bool(b)) = (self,rhs) {
            return DataType::Int(DataTypeInteger::<T>{v: Zero::zero()})
        }
        else {
            return
        }
    }
}





impl std::fmt::Display for DataType<T> {
    fn fmt(&self, _f: &mut std::fmt::Formatter) -> std::fmt::Result  {
 
            match self.v {
                DataType<T>::Int(a) => Ok(println!("{}", a)),
                DataType<T>::Float(a) =>Ok(println!("{}", a)),
                DataType<T>::Ratio(a) =>Ok(println!("{}", a)),
                DataType<T>::Bool(a) =>Ok(println!("{}", a)),
            }

        }
    
}

*/

 
/*
match gen1.v { 
        DataType<T>::Int(a) =>
            match rhs.v {
                DataType<T>::Int(b) =>  ,
                DataType<T>::Float(b) =>,
                DataType<T>::Ratio(b) =>,
                DataType<T>::Bool(b) =>,
            },
        DataType<T>::Float(a) =>
            match rhs.v {
                DataType<T>::Int(b) =>,
                DataType<T>::Float(b) =>,
                DataType<T>::Ratio(b) =>,
                DataType<T>::Bool(b) =>,
            },
        DataType<T>::Ratio(a) =>
            match rhs.v {
                DataType<T>::Int(b) =>,
                DataType<T>::Float(b) =>,
                DataType<T>::Ratio(b) =>,
                DataType<T>::Bool(b) =>,
            },
        DataType<T>::Bool(a) =>
            match rhs.v {
                DataType<T>::Int(b) =>,
                DataType<T>::Float(b) =>,
                DataType<T>::Ratio(b) =>,
                DataType<T>::Bool(b) =>,
            },
    }

    
       match self { 
        DataType::<T>::Int(a) =>
            match rhs {
                DataType::<T>::Int(b) =>  return DataType::Int(DataTypeInteger::<T>{v: Zero::zero()}),
                DataType::<T>::Float(b) =>return DataType::Int(DataTypeInteger::<T>{v: Zero::zero()}),
                DataType::<T>::Ratio(b) =>return DataType::Int(DataTypeInteger::<T>{v: Zero::zero()}),
                DataType::<T>::Bool(b) =>return DataType::Int(DataTypeInteger::<T>{v: Zero::zero()}),
            },
        DataType::<T>::Float(a) =>
            match rhs {
                DataType::<T>::Int(b) =>return DataType::Int(DataTypeInteger::<T>{v: Zero::zero()}),
                DataType::<T>::Float(b) =>return DataType::Int(DataTypeInteger::<T>{v: Zero::zero()}),
                DataType::<T>::Ratio(b) =>return DataType::Int(DataTypeInteger::<T>{v: Zero::zero()}),
                DataType::<T>::Bool(b) =>return DataType::Int(DataTypeInteger::<T>{v: Zero::zero()}),
            },
        DataType::<T>::Ratio(a) =>
            match rhs.v {
                DataType<T>::Int(b) =>return DataType::Int(DataTypeInteger::<T>{v: Zero::zero()}),
                DataType<T>::Float(b) =>return DataType::Int(DataTypeInteger::<T>{v: Zero::zero()}),
                DataType<T>::Ratio(b) =>return DataType::Int(DataTypeInteger::<T>{v: Zero::zero()}),
                DataType<T>::Bool(b) =>return DataType::Int(DataTypeInteger::<T>{v: Zero::zero()}),
            },
        DataType::<T>::Bool(a) =>
            match rhs.v {
                DataType<T>::Int(b) =>return DataType::Int(DataTypeInteger::<T>{v: Zero::zero()}),
                DataType<T>::Float(b) =>return DataType::Int(DataTypeInteger::<T>{v: Zero::zero()}),
                DataType<T>::Ratio(b) =>return DataType::Int(DataTypeInteger::<T>{v: Zero::zero()}),
                DataType<T>::Bool(b) =>return DataType<T> { v: DataType<T>::Int(&0)},
            },
        }
    }
    */