use std::num::ParseIntError;

pub fn take_fifth(values: Vec<i32>) -> Option<i32> {
    if values.len() < 5 {
        None
    } else {
        Some(values[4])
    }
}

pub fn give_result(input: i32) -> Result<(), ()> {
    if input % 2 == 0 {
        Ok(())
    } else {
        Err(())
    }
}

pub fn check_if_five(input: i32) -> Result<i32, String> {
    match input {
        5 => Ok(input),
        _ => Err("The number is not five".to_string()),
    }
}

pub fn parse_str(input: &str) -> Result<i32, ParseIntError> {
    let parsed_number = input.parse::<i32>()?;
    Ok(parsed_number)
}

pub fn prints_three_things(vector: Vec<i32>) {
    if vector.len() != 3 {
        panic!("Input length is not 3");
    }
    for x in vector {
        println!("{}", x);
    }
}

pub fn get_fourth(input: &[i32]) -> i32 {
    let number = input
        .get(3)
        .expect("Input vector needs at least 4 elements");
    let number = input.get(3).unwrap_or(&0);
    *number
}
