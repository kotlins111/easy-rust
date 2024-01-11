use crate::collections::loop_demo;
use std::fmt::{Debug, Display, Formatter};

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
    fn new(name: &str, population: u32) -> Self {
        // just a new function
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
        Self { cities }
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
                true => even_odd_vec[0].push(item),
                false => even_odd_vec[1].push(item),
            }
        }
        Self(even_odd_vec)
    }
}

fn print_it<T>(input: T)
where
    T: Debug + Display + AsRef<str>,
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

// impl trait (object)

fn prints_it(input: impl Into<String> + Display) {
    println!("Print {}", input.into());
}

fn returns_a_closure(input: &str) -> impl FnMut(i32) -> i32 {
    match input {
        "double" => |mut number| {
            number *= 2;
            println!("Doubling the number. Now it is {number}");
            number
        },
        "triple" => |mut number| {
            number *= 3;
            number
        },
        _ => |number| number,
    }
}

pub fn use_closure(input: i32, op: &str) {
    let result = returns_a_closure(op)(input);
    println!("{}", result);
}

enum TimeOfDay {
    Dawn,
    Day,
    Sunset,
    Night,
}

fn change_fear(input: TimeOfDay) -> impl FnMut(f64) -> f64 {
    //use closure to change the fear and return back
    use TimeOfDay::*;

    match input {
        Dawn => |x| {
            println!(
                "The morning sun has vanquished the horrible night. You no longer feel afraid."
            );
            println!("Your fear is now {}", x * 0.5);
            x * 0.5
        },
        Day => |x| {
            println!("What a nice day. Maybe put your feet up and rest a bit.");
            println!("Your fear is now {}", x * 0.2);
            x * 0.2
        },
        Sunset => |x| {
            println!("The sun is almost down! This is not good.");
            println!("Your fear is now {}", x * 1.4);
            x * 1.4
        },
        Night => |x| {
            println!("What a horrible night to have a curse.");
            println!("Your fear is now {}", x * 5.0);
            x * 5.0
        },
    }
}

pub fn start_game() {
    use TimeOfDay::*;
    let mut character_fear = 10.0; // Start Simon with 10

    let mut daytime_buff = change_fear(Day); // Make four closures here to call every time we want to change Simon's fear.
    let mut sunset_buff = change_fear(Sunset);
    let mut night_buff = change_fear(Night);
    let mut morning_buff = change_fear(Dawn);

    character_fear = daytime_buff(character_fear); // Call the closures on Simon's fear. They give a message and change the fear number.
                                                   // In real life we would have a Character struct and use it as a method instead,
                                                   // like this: character_fear.daytime_buff()
    character_fear = sunset_buff(character_fear);
    character_fear = night_buff(character_fear);
    character_fear = morning_buff(character_fear);
}
