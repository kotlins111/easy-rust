use std::ops::{Add, AddAssign, Div, Mul, Sub};
pub fn print_demo1() {
    let s = "hello world";
    let arr = [1, 3, 4, 5, 6];
    println!("{}", s);
    println!("{:?}", arr);
    println!("{:#?}", arr); //pretty print
    println!("{:?}", b"this is my string");
    println!("{:X}", 255);
    println!("{:p}", &arr);
    println!("{:06}", 43); // padding zero at start make the display length is 6

    //all of these print Hello x____!
    println!("Hello {:5}!", "x");
    let s = format!("Hello {:5}!", "x");
    println!("The format sting len is {}", s.chars().count());
    println!("Hello {:1$}!", "x", 5);
    println!("Hello {1:0$}!", 5, "x");
    println!("Hello {:width$}!", "x", width = 5);
    let width = 5;
    println!("Hello {:width$}!", "x");

    assert_eq!(format!("Hello {:<5}!", "x"), "Hello x    !"); // left aligned
    assert_eq!(format!("Hello {:-<5}!", "x"), "Hello x----!"); // left aligned with -
    assert_eq!(format!("Hello {:^5}!", "x"), "Hello   x  !"); // center aligned with ^
    assert_eq!(format!("Hello {:>5}!", "x"), "Hello     x!"); // right aligned

    assert_eq!(format!("Hello {:+}!", 5), "Hello +5!"); // sign a number with +
    assert_eq!(format!("{:#x}!", 27), "0x1b!"); // #x x decimal
    println!("{:#?}", b"this is my string"); // #? debug pretty print
    assert_eq!(format!("Hello {:05}!", 5), "Hello 00005!"); // 05 padding to 5 with 0
    assert_eq!(format!("Hello {:05}!", -5), "Hello -0005!"); // signal aware
    assert_eq!(format!("{:#010x}!", 27), "0x0000001b!");
    assert_eq!(format!("{:#010b}", 27), "0b00011011"); // #b binary #o octal

    //format trait
    println!("{:?}", 1); // ? Debug trait
    println!("{:x?}", 234); // x? Debug with lower-case x decimal
    println!("{:X?}", 234); // X? Debug with upper-case x decimal
    println!("{:o}", 27); // o octal
    println!("{:b}", 27); // b binary
    println!("{:x}", 1); // x Lower Hex
    println!("{:X}", 1); // X Upper Hex
    println!("{:p}", &1); // p Pointer
    println!("{:e}", 1001); // e lower Exp
    println!("{:E}", 1001); // E Upper Exp
}

/// print demo
/// illustrate the position format args
pub fn print_demo2() {
    let father_name = "Vlad";
    let son_name = "Adrian Fahrenheit";
    let family_name = "Tep";
    println!(
        "This is {1} {2}, the son of {0} {2}",
        father_name, son_name, family_name
    );
}

/// print demo
/// illustrate the named format args
/// also can declare in println macro
pub fn print_demo3() {
    let city1 = "Seoul";
    let city2 = "Busan";
    let city3 = "Tokyo";
    let country = "Korea";

    println!(
        "
    {city1} is in {country} and {city2} is also in {country}
    but {city3} is not in {country}."
    )
}

/// a complex print demo
pub fn print_demo4() {
    let title = "Today's News";
    println!("{:-^30}", title);
    let bar = "|";
    println!("{:<15}{:>15}", bar, bar);
    let a = "SEOUL";
    let b = "TOKYO";
    println!("{city1:-<15}{city2:->15}", city1 = a, city2 = b)
}

pub fn random_i32_to_unicode() {
    use rand::{thread_rng, Rng};
    let some_character = char::from(99); // This one is easy - no need for TryFrom
    println!("{}", some_character);

    let mut random_generator = thread_rng();
    // This will try 40,000 times to make a char from a u32.
    // The range is 0 (std::u32::MIN) to u32's highest number (std::u32::MAX). If it doesn't work, we will give it '-'.
    for _ in 0..40_000 {
        let bigger_character =
            char::try_from(random_generator.gen_range(10000..50000u32)).unwrap_or('-');
        print!("{}", bigger_character)
    }
}

fn check_method_number() {
    let some_number = 200_u8;
    let other_number = 200_u8;

    println!("{:?}", some_number.checked_add(other_number)); //None cause overflow
    println!("{:?}", some_number.checked_add(1)); // Some(201)
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Self;
    fn add(self, other: Point) -> Self {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

pub fn print_random_chars() {
    use rand::seq::SliceRandom;
    let my_letters = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];

    let mut rng = rand::thread_rng();
    for _ in 0..6 {
        print!("{} ", my_letters.choose(&mut rng).unwrap());
    }
}
