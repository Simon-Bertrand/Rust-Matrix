pub fn count_digits(data_numeric : f64) -> i32 {
    (data_numeric.to_string().len() + 3) as i32

    /*
    if d > 1.0 {
        while d>10.0 {
            i+=1;
            d/=10 as f64;
        }
        i
    }
    else {
        while d<=1.0 {
            i+=1;
            d*=10 as f64;
        }
        i
    }
    */
}

