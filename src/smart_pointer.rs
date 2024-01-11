use std::rc::Rc;

//RC
fn takes_a_string(string: String) {
    println!("{}", string);
}

fn takes_a_string_again(string: String) {
    println!("{}", string);
}

fn takes_string() {
    let s = "I am a string".to_string();
    takes_a_string(s);
    // takes_a_string_again(s);  // s is not available
}

#[derive(Debug)]
struct City1 {
    name: String,
    population: u32,
    city_history: String,
}

#[derive(Debug)]
struct CityData1 {
    names: Vec<String>,
    histories: Vec<String>,
}

pub fn rc_demo1() {
    let calgary = City1 {
        name: "Calgary".to_string(),
        population: 1_200_000,
        // Pretend that this string is very very long
        city_history: "Calgary began as a fort called Fort Calgary that...".to_string(),
    };

    let canada_cities = CityData1 {
        names: vec![calgary.name], // This is using calgary.name, which is short
        histories: vec![calgary.city_history], // But this String is very long
    };

    // println!("{} {}", calgary.city_history,calgary.name);
}

// use rc

#[derive(Debug)]
struct City {
    name: String,
    population: u32,
    city_history: Rc<String>,
}

#[derive(Debug)]
struct CityData {
    names: Vec<String>,
    histories: Vec<Rc<String>>,
}

pub fn rc_demo2() {
    let calgary = City {
        name: "Calgary".to_string(),
        population: 1_200_000,
        // Pretend that this string is very very long
        city_history: Rc::from("Calgary began as a fort called Fort Calgary that...".to_string()),
    };

    let canada_cities = CityData {
        names: vec![calgary.name], // This is using calgary.name, which is short
        histories: vec![calgary.city_history.clone()], // But this String is very long
    };

    println!("Calgary's history is: {}", calgary.city_history);
    println!("{}", Rc::strong_count(&calgary.city_history));
    let new_owner = calgary.city_history.clone();

    println!("{}", Rc::strong_count(&calgary.city_history));
}
