pub fn count_digits(data_numeric : f64) -> i32 {
    let mut i : i32 = 3;
    let mut d=data_numeric;

    while d>10.0 {
        i+=1;
        d/=10 as f64;
    }

    i
}

