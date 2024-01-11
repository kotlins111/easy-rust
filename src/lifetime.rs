use std::fmt::{Display, Formatter};

fn str_lifetime() {
    let my_str = "I am a &str";  // &'static
    let my_string = String::from("I am a string");
    print_str(&my_string);  // reference from String without lifetime

    let my_string = String::new();
    let city = City {
        name: "Berlin",
        population: 1_000_000,
    };
}

fn print_str(input: &str) {
    println!("{}", input);
}

// missing lifetime
fn return_ref() -> &'static str {
    let s = String::new();
    "I am a str"
}


#[derive(Debug)]
struct City<'a> {
    name: &'a str,
    //get ref for the name lives as long as City
    population: u32,
}


struct Adventure<'a> {
    name: &'a str,
    hit_points: u32,
}

impl Adventure<'_> {
    fn take_damage(&mut self) {
        self.hit_points -= 20;
        println!("{} has {} hit points left!", self.name, self.hit_points);
    }
}

impl Display for Adventure<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} has {} hit points",self.name,self.hit_points)
    }
}



