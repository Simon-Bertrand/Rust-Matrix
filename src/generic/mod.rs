mod operators;
mod generic;
use num_rational::Ratio;
use num::Zero;



pub trait AllowedTypes   {}

impl AllowedTypes for i32 {}
impl AllowedTypes for i64 {}
impl AllowedTypes for f32 {}
impl AllowedTypes for f64 {}
impl AllowedTypes for Ratio<i32> {}
impl AllowedTypes for Ratio<i64> {}
impl AllowedTypes for bool {}


pub trait NumInteger   {}

impl NumInteger for i32 {}
impl NumInteger for i64 {}

pub trait NumFloat  {}
impl NumFloat for f32 {}
impl NumFloat for f64 {}

pub trait NumRatio  {}
impl NumRatio for Ratio<i32> {}
impl NumRatio for Ratio<i64> {}

pub trait NumBool  {}
impl NumBool for bool {}


pub enum DataTypeVariants<I : NumInteger, F : NumFloat, R : NumRatio, B : NumBool> {
    Int(DataType<I>),
    Float(DataType<F>),
    Ratio(DataType<R>),
    Bool(DataType<B>),
}


impl<I : NumInteger, F : NumFloat, R : NumRatio, B : NumBool> DataTypeVariants<I , F , R , B >{
    pub fn value(&self) -> &T {
        match &self {
            DataTypeVariants::Int(a)=> return &a.v,
            DataTypeVariants::Float(a) => return &a.v,
            DataTypeVariants::Ratio(a) => return &a.v,
            DataTypeVariants::Bool(a) => return &a.v,
        }
    }
}

/*
macro_rules! sub_impl {
    ($($t:ty)*) => ($(
        impl DataType<$t> {
            fn new() -> DataType<$t> {
        
            }
        }
    )*)
}

sub_impl! { i32 i64 f32 f64 }
*/



pub struct DataType<T : AllowedTypes> {
    pub v : T,
}



/*    pub fn new(v_type : &str) -> DataType<T> {
        if v_type == "int" {DataType::Int(DataTypeInteger{ v : Zero::zero()})}
        else if v_type == "float" {DataType::Float(DataTypeFloat{ v : Zero::zero()})}
        else if v_type == "ratio" {DataType::Ratio(DataTypeRatio{ v : Zero::zero()})}
        else if v_type == "bool" {DataType::Bool(DataTypeBool{ v : Zero::zero()})}  
        else {
            DataType::Int(DataTypeInteger{ v : Zero::zero()})
        }   
    }*/
