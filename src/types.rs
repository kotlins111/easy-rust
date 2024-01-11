use crate::types::Mood::Happy;
use crate::types::Number::{Int, UInt};
use crate::types::Star::BrownDwarf;
use std::fmt::{Debug, Display, Formatter, Pointer};
use std::iter::{Skip, Take};
use std::slice::Iter;

type CharacterVec = Vec<char>;
type SkipFourTakeFive<'a,T> = Take<Skip<Iter<'a,T>>>;

fn returns<'a,T:Display>(input: &'a Vec<T>) ->SkipFourTakeFive<'a,T>{
    input.iter().skip(4).take(5)
}
// tuple
pub fn tuple_demo1() {
    let random_tuple = (2, 2., vec![1, 2, 3], '&', [1, 2, 3], &[0, 1, 2]);
    println!("The last element of the tuple is {:#p}", random_tuple.5);
    let (_, _, _, my_char, my_array, my_ref) = random_tuple; // deconstruct
}

//match
pub fn match_demo1() {
    let my_number = 5;
    match my_number {
        0 => {
            println!("it is zero")
        }
        1 => println!("it is one"),
        _ => println!("it is something else"),
    };

    let mapped_number = match my_number {
        0 => 1,
        1 => 10,
        3 => 100,
        _ => -1,
    };

    let sky = "cloudy";
    let temp = "warm";
    match (sky, temp) {
        ("cloudy", "cold") => println!("It is dark and unpleasant today"),
        ("cloudy", "warm") => println!("It is dark but not bad"),
        ("clear", "warm") => println!("It is a nice day"),
        _ => println!("Not sure about the weather"),
    }
}

pub fn match_guard() {
    let children_num = 3;
    let married = true;
    match (married, children_num) {
        (married, num) if married == false => println!("Not married with {} children", num),
        (married, num) if num == 0 && married => println!("Married but no children"),
        (married, num) if num < 4 && married => println!("Children is less than 4, illegal family"),
        _ => println!("Married? {married}, Number of children is {children_num}"),
    }
}

pub fn match_demo2() {
    let red = Color(255, 0, 0);
    match red {
        Color(r, _, _) if r < 50 => {
            println!("not too red")
        }
        Color(_, _, b) if b < 50 => println!("not too blue"),
        Color(_, g, _) if g < 50 => println!("not too green"),
        _ => println!("fine"),
    }
}

pub fn match_demo3(point: Point) {
    // do not need to reference cause the struct can be copied
    match point {
        p @ Point { x: 0, y: _ } => println!("The point is at y axis, the y coordinate is {}", p.y),
        p @ Point { x: _, y: 0 } => println!("The point is at x axis, the x coordinate is {}", p.x),
        Point { .. } => println!("Do not at any axis"),
    }
}

// struct
struct Color(u8, u8, u8);

#[derive(Clone, Copy)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

struct FileDirectory;

struct Country {
    population: u64,
    capital: String,
    leader_name: String,
}

pub fn struct_demo1() {
    let population = 1_400_000_000_u64;
    let capital = "Beijing".to_string();
    let leader_name = "刁".to_string();

    let china = Country {
        population,
        capital,
        leader_name,
    };

    println!("{}", china);
}

impl Display for Country {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "The country's capital is {}, it has population of {}, and their leader is {}.",
            self.capital, self.population, self.leader_name
        )
    }
}
// enum

pub enum Season {
    Spring(u8),
    Summer(u8),
    Fall(u8),
    Winter(u8),
}

pub fn create_season(month: u8) -> Season {
    match month {
        1..=3 => Season::Spring(15),
        4..=6 => Season::Summer(30),
        7..=9 => Season::Fall(20),
        10..=12 => Season::Winter(0),
        _ => panic!("invalid provided month"),
    }
}

pub fn check_season(season: &Season) {
    match season {
        Season::Spring(temp) => {
            println!("It is a good time to photo,the temp is {}", temp)
        }
        Season::Summer(temp) => {
            println!("It is a really hot days,the temp is {}", temp)
        }
        Season::Fall(temp) => {
            println!("It is a quite fine weather,the temp is {}", temp)
        }
        Season::Winter(temp) => {
            println!("Time to keep you warm,the temp is {}", temp)
        }
    }
}

pub enum Mood {
    Happy,
    Sleepy,
    NotBad,
    Angry,
}

pub fn happiness_level(mood: &Mood) -> u8 {
    match mood {
        Mood::Happy => 10,
        Mood::Sleepy => 6,
        Mood::NotBad => 7,
        Mood::Angry => 2,
    };
    use Mood::*;
    match mood {
        Happy => 10,
        Sleepy => 6,
        NotBad => 7,
        Angry => 2,
    }
}

pub fn show_moods() {
    use Mood::*;
    let moods = vec![Happy, NotBad, Sleepy, Angry];
    for mood in moods {
        println!("{}", mood as u32)
    }
}

#[derive(Copy, Clone)]
pub enum Star {
    BrownDwarf = 10,
    RedDwarf = 50,
    YellowStar = 100,
    RedGiant = 1_000,
    DeadStar,
}

pub fn evaluate_star(stars: Vec<Star>) {
    for star in stars {
        match star as u32 {
            size if size < 80 => println!("Not big enough"),
            // size if size >= 80 => println!("The size is good"),
            size => println!("That star is pretty big. It's size is {size}"),
        }
    }
}

pub fn stars_size() {
    use Star::*;
    let star_vec = vec![BrownDwarf, RedDwarf, YellowStar, DeadStar];
    evaluate_star(star_vec);
}

pub enum Number {
    UInt(u32),
    Int(i32),
}

pub fn get_number(input: i32) -> Number {
    match input.is_positive() {
        true => UInt(input as u32),
        false => Int(input),
    }
}

pub fn wrap_int() {
    let num_vec = vec![get_number(-800), get_number(8)];

    for item in num_vec {
        match item {
            UInt(num) => {
                println!("It is an u32 with the value {num}")
            }
            Int(num) => {
                println!("It is an i32 with the value of {num}")
            }
        }
    }
}

pub struct Person {
    name: String,
    real_name: String,
    height: u8,
    happiness: bool,
}

pub fn destruct_demo() {
    let papa_doc = Person {
        name: "papa dod".to_string(),
        real_name: "Clarence".to_string(),
        height: 183,
        happiness: true,
    };

    // destruct
    let Person {
        name: a, height: b, ..
    } = papa_doc;

    println!("The name is {a} and the height is {b}");
}

struct City {
    name: String,
    name_before: String,
    population: u32,
    date_founded: u32,
}

impl City {
    fn new(name: String, name_before: String, population: u32, date_founded: u32) -> Self {
        Self {
            name,
            name_before,
            population,
            date_founded,
        }
    }
}

fn process_with_city(city: &City) {
    let City {
        name, name_before, ..
    } = city;

    let two_names = vec![name, name_before];
    println!("{:?}", two_names);
}

pub fn city_demo() {
    let tallinn = City::new("Tallinn".into(), "Reval".into(), 426_538, 1219);
    process_with_city(&tallinn);
}

// generic type

fn return_number<T>(number: T) -> T {
    println!("Generic example.");
    number
}

// type constrains
fn print_number<T: Debug>(number: T) {
    println!("{:?}", number)
}

fn compare_and_display<T: Display, U: Display + Ord>(statement: T, num1: U, num2: U) {
    println!(
        "{}! Is {} greater than {}? [{}]",
        statement,
        num1,
        num2,
        num1 > num2
    );
}

fn compare_and_display1<T, U>(statement: T, num1: U, num2: U)
where
    T: Display,
    U: Display + PartialOrd,
{
    println!(
        "{}! Is {} greater than {}? [{}]",
        statement,
        num1,
        num2,
        num1 > num2
    );
}


// type alias
enum FileState{
    CannotAccess,
    FileOpenAndReady,
    NoSuchFile,
}

fn give_filestate(input:&FileState){
    use FileState::{
        CannotAccess as NoAccess,
        FileOpenAndReady as Good,
        NoSuchFile as NotFound
    };
    match input {
        NoAccess => {}
        Good => {}
        NotFound => {}
    }
}

fn todo_function() -> String{
    todo!()  // no need to return String for now, do it later
}