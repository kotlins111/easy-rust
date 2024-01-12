use crate::collections::loop_demo;
use rand::{thread_rng, Rng};
use std::fmt::{Debug, Display, Formatter};
use std::ops::{Deref, DerefMut};

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

// Default Trait

// fn to print default value of i8,String,bool

pub fn default_value() {
    let default_i8: i8 = Default::default();
    let default_string: String = Default::default();
    let default_bool: bool = Default::default();

    println!("{default_i8} {default_string} {default_bool}");
}

#[derive(Debug)]
struct Character {
    name: String,
    age: u8,
    height: u32,
    weight: u32,
    lifestate: LifeState,
    can_use: bool,
}

#[derive(Debug)]
enum LifeState {
    Alive,
    Dead,
    NeverAlive,
    Uncertain,
}

impl Character {
    fn new() -> Self {
        Self {
            name: String::from("Billy"),
            age: 15,
            height: 170,
            weight: 70,
            lifestate: LifeState::Alive,
            can_use: true,
        }
    }

    fn height(mut self, height: u32) -> Self {
        self.height = height;
        self.can_use = false;
        self
    }

    fn weight(mut self, weight: u32) -> Self {
        self.weight = weight;
        self.can_use = false;
        self
    }

    fn name(mut self, name: &str) -> Self {
        self.name = name.to_string();
        self.can_use = false;
        self
    }

    fn build(mut self) -> Result<Character, String> {
        if self.height < 200 && self.weight < 300 && !self.name.to_lowercase().contains("smurf") {
            self.can_use = true;
            Ok(self)
        } else {
            Err("Could not create character. Characters must have:
                    1) Height below 200
                    2) Weight below 300
                    3) A name that is not Smurf (that is a bad word)"
                .to_string())
        }
    }
}

impl Default for Character {
    fn default() -> Self {
        Self {
            name: String::from("Billy"),
            age: 15,
            height: 170,
            weight: 70,
            lifestate: LifeState::Alive,
            can_use: true,
        }
    }
}

pub fn default_struct() {
    // let character_1 = Character::new("Billy".to_string(), 15, 170, 70, true);
    let character_2 = Character::default();
    let character_3 = Character::default().height(180).weight(60).name("Bobby");
    let character_with_smurf = Character::new().name("Lol I am Smurf!!").build();
    let character_too_tall = Character::new().height(400).build();
    let character_too_heavy = Character::new().weight(500).build();
    let okay_character = Character::new()
        .name("Billybrobby")
        .height(180)
        .weight(100)
        .build();

    let character_vec = vec![
        character_with_smurf,
        character_too_tall,
        character_too_heavy,
        okay_character,
    ];

    for character in character_vec {
        match character {
            Ok(character_info) => println!("{:?}", character_info),
            Err(err_info) => println!("{}", err_info),
        }
        println!();
    }
    // println!("{:?}", character_3);
}

// Deref and DerefMut Trait

struct HoldsANumber(u8);

impl HoldsANumber {
    fn prints_the_number_times_two(&self) {
        println!("{}", self.0 * 2);
    }
}

impl Deref for HoldsANumber {
    type Target = u8;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for HoldsANumber {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub fn add_struct_with_i32() {
    let mut my_number = HoldsANumber(10);
    *my_number = 110;
    println!("{:?}", my_number.checked_sub(100)); // smart pointer , use method of u8
    println!("{:?}", *my_number + 2);
}

// only impl Deref trait for smart pointer

#[derive(Debug)]
struct Character1 {
    name: String,
    strength: u8,
    dexterity: u8,
    health: u8,
    intelligence: u8,
    wisdom: u8,
    charm: u8,
    hit_points: i8,
    alignment: Alignment,
}

impl Character1 {
    fn new(
        name: String,
        strength: u8,
        dexterity: u8,
        health: u8,
        intelligence: u8,
        wisdom: u8,
        charm: u8,
        hit_points: i8,
        alignment: Alignment,
    ) -> Self {
        Self {
            name,
            strength,
            dexterity,
            health,
            intelligence,
            wisdom,
            charm,
            hit_points,
            alignment,
        }
    }

    fn new_dice(dice: Dice) -> Self {
        match dice {
            Dice::Three => Self {
                name: "".to_string(),
                strength: three_die_six(),
                dexterity: three_die_six(),
                health: 0,
                intelligence: three_die_six(),
                wisdom: three_die_six(),

                charm: 0,
                hit_points: 0,
                alignment: Alignment::Good,
            },
            Dice::Four => Self {
                name: "".to_string(),
                strength: four_die_six(),
                dexterity: four_die_six(),
                health: 0,
                intelligence: four_die_six(),
                wisdom: four_die_six(),
                charm: 0,
                hit_points: 0,
                alignment: Alignment::Good,
            },
        }
    }
}

#[derive(Debug)]
enum Alignment {
    Good,
    Neutral,
    Evil,
}

impl Deref for Character1 {
    // impl Deref for Character. Now we can do any integer math we want!
    type Target = i8;

    fn deref(&self) -> &Self::Target {
        &self.hit_points
    }
}

pub fn only_impl_deref_on_smart_pointers() {
    let billy = Character1::new("Billy".to_string(), 9, 8, 7, 10, 19, 19, 5, Alignment::Good); // Create two characters, billy and brandy
    let brandy = Character1::new(
        "Brandy".to_string(),
        9,
        8,
        7,
        10,
        19,
        19,
        5,
        Alignment::Good,
    );

    let hit_points_vec = vec![*billy, *brandy]; // Push *brandy? Push *billy?

    println!("{:?}", hit_points_vec);

    let weak_billy = Character1::new_dice(Dice::Three);
    let strong_billy = Character1::new_dice(Dice::Four);

    println!("{:?}", weak_billy);
}

fn three_die_six() -> u8 {
    // A "die" is the thing you throw to get the number
    let mut generator = thread_rng(); // Create our random number generator
    let mut stat = 0; // This is the total
    for _ in 0..3 {
        stat += generator.gen_range(1..=6); // Add each time
    }
    stat // Return the total
}

fn four_die_six() -> u8 {
    let mut generator = thread_rng();
    let mut results = vec![]; // First put the numbers in a vec
    for _ in 0..4 {
        results.push(generator.gen_range(1..=6));
    }
    results.sort(); // Now a result like [4, 3, 2, 6] becomes [2, 3, 4, 6]
    results.remove(0); // Now it would be [3, 4, 6]
    results.iter().sum() // Return this result
}

enum Dice {
    Three,
    Four,
}
