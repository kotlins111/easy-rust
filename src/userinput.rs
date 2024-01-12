use std::io::stdin;

pub fn scan_user_input() {
    println!("Please type something, or x to escape:");
    let mut input_string = String::new();
    while input_string.trim() != "x" {
        input_string.clear();
        stdin().read_line(&mut input_string).unwrap();
        println!("You typed: {}", input_string);
    }
    println!("See you later!");
}

pub fn args_from_command_line() {
    let args: Vec<String> = std::env::args().collect();
    println!("{:?}", args);
}

pub fn get_all_env() {
    std::env::set_var("USER", "your_username");
    for item in std::env::vars() {
        println!("{:?}", item);
    }

    // println!("{}", env!("USER"));
    println!("{}", option_env!("ROOT").unwrap_or("Can't find ROOT"));
    println!("{}", option_env!("CARGO").unwrap_or("Can't find CARGO"));
}


pub fn match_inputs(input: &str) {
    match input {
        "x" => { std::process::exit(0); }
        "a" | "s" | "w" | "d" => { println!("Moving {}", input); }
        "1" | "2" | "3" => { println!("Number {}", input); }
        key if key.chars().all(|c| c.is_lowercase()) => { println!("Lowercase {}", key); }

        _ => println!("Unknown input")
    }
}