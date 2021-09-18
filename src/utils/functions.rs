pub fn count_digits<T : std::string::ToString>(data_numeric : T) -> usize {
    data_numeric.to_string().len() + 3
}

