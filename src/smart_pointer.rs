use std::error::Error;
use std::fmt::{Display, Formatter};
use std::mem::size_of;
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

// finally, the Box
struct MyStruct {
    name: String,
    age: u32,
    height: u32,
}

#[derive(Debug)]
enum MyEnum {
    Variable1,
    Variable2,
}

pub fn box_at_heap() {
    let my_struct = Box::new(MyStruct {
        name: "John".to_string(),
        age: 30,
        height: 180,
    });

    let my_enum = Box::new(MyEnum::Variable1);

    //access fields of my_struct
    println!("Name: {}", my_struct.name);

    //access fields of my_enum
    println!("Value: {:?}", my_enum);
}

struct List {
    item: Option<Box<List>>,
}

impl List {
    fn new() -> Self {
        List {
            item: Some(Box::new(List { item: None })), //could be infinite size
        }
    }
}

trait JustATrait {}

enum EnumOfNumbers {
    I8(i8),
    AnotherI8(i8),
    OneMoreI8(i8),
}

impl JustATrait for EnumOfNumbers {}

struct StructOfNumbers {
    an_i8: i8,
    another_i8: i8,
    one_more_i8: i8,
}
impl JustATrait for StructOfNumbers {}

enum EnumOfOtherTypes {
    I8(i8),
    AnotherI8(i8),
    Collection(Vec<String>),
}
impl JustATrait for EnumOfOtherTypes {}

struct StructOfOtherTypes {
    an_i8: i8,
    another_i8: i8,
    a_collection: Vec<String>,
}
impl JustATrait for StructOfOtherTypes {}

struct ArrayAndI8 {
    array: [i8; 1000], // This one will be very large
    an_i8: i8,
    in_u8: u8,
}
impl JustATrait for ArrayAndI8 {}

pub fn print_size_of_types() {
    println!(
        "{} {} {} {} {}",
        size_of::<EnumOfNumbers>(),
        size_of::<StructOfNumbers>(),
        size_of::<EnumOfOtherTypes>(),
        size_of::<StructOfOtherTypes>(),
        size_of::<ArrayAndI8>(),
    );
}

fn returns_just_a_trait() -> Box<dyn JustATrait> {
    // sized which have a size same as raw pointers
    let some_enum = EnumOfNumbers::I8(1);

    Box::new(some_enum)
}

#[derive(Debug)]
struct ErrorOne;

impl Display for ErrorOne {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "ErrorOne")
    }
}

impl Error for ErrorOne {}

#[derive(Debug)] // Do the same thing with ErrorTwo
struct ErrorTwo;

impl Error for ErrorTwo {}

impl Display for ErrorTwo {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "You got the second error!")
    }
}

// Make a function that just returns a String or an error
pub fn returns_error(input: u8) -> Result<String, Box<dyn Error>> {
    // compile time aware size
    // if input is 0, return an ErrorOne
    // if input is 1 ,return an ErrorTwo
    // if input is another thing, return a String

    if input == 0 {
        return Err(Box::new(ErrorOne));
    }

    if input == 1 {
        return Err(Box::new(ErrorTwo));
    }

    Ok("Hello".to_string())
}
