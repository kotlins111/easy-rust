use std::fmt::{Debug, Display, Formatter};
use crate::collections::loop_demo;

struct Animal {
    name: String,
}

trait Dog {
    fn bark(&self);

    fn run(&self) {
        println!("The dog is running");
    }
}

impl Dog for Animal {
    fn bark(&self) {
        println!("{} is barking", self.name);
    }

    fn run(&self) {
        println!("{} is running", self.name);
    }
}

pub fn trait_demo1() {
    let rover = Animal {
        name: "Rover".to_string(),
    };

    rover.bark();
    rover.run();
}


struct Position {
    longitude: f32,
    latitude: f32,
}

impl Display for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} {})", self.longitude, self.latitude)
    }
}


fn print_vec<T: Display>(input: &Vec<T>) {
    for item in input {
        print!("{}#", item);
    }
    println!();
}

pub fn vec_from_trait_demo() {
    let array_vec = Vec::from([8, 9, 10]);
    print_vec(&array_vec);

    let str_vec = Vec::from("Hello world! 你好💕!");
    print_vec(&str_vec);

    let string_vec = Vec::from("The kind of vec will a String be".to_string());
    print_vec(&string_vec);
}


#[derive(Debug)] // So we can print City
struct City {
    name: String,
    population: u32,
}

impl City {
    fn new(name: &str, population: u32) -> Self { // just a new function
        Self {
            name: name.to_string(),
            population,
        }
    }
}

#[derive(Debug)] // Country also needs to be printed
struct Country {
    cities: Vec<City>, // Our cities go in here
}

impl From<Vec<City>> for Country {
    fn from(cities: Vec<City>) -> Self {
        Self {
            cities
        }
    }
}

impl Country {
    fn print_cities(&self) {
        for city in &self.cities {
            println!("{:?} has a population of {}", city.name, city.population);
        }
    }
}

pub fn city_vec_to_country() {
    let helsinki = City::new("Helesinki", 631_695);
    let turku = City::new("Turku", 186_756);

    let finland_cities = vec![helsinki, turku];
    let finland = Country::from(finland_cities);

    finland.print_cities();
}


pub struct EvenOddVec(pub Vec<Vec<i32>>);

impl From<Vec<i32>> for EvenOddVec {
    fn from(value: Vec<i32>) -> Self {
        let mut even_odd_vec = vec![vec![], vec![]];
        for item in value {
            match item % 2 == 0 {
                true => { even_odd_vec[0].push(item) }
                false => { even_odd_vec[1].push(item) }
            }
        }
        Self(even_odd_vec)
    }
}


fn print_it<T>(input: T)
    where T: Debug + Display + AsRef<str>
{
    println!("{}", input);
    if input.as_ref().contains(".NET") {
        println!(".NET is the programming language of 2023")
    }
}

fn test_print_it() {
    print_it("Print me");
    print_it("Also print me".to_string());
    // print_it(9);
}

































