pub fn take_fifth(values: Vec<i32>) -> Option<i32> {
    if values.len() < 5 { None } else { Some(values[4]) }
}