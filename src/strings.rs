
use std::mem::{size_of,size_of_val};

pub fn strings_demo1(){
    let name = "😊";
    println!("My name is {name}");
    println!("A String is always {:?} bytes.It is sized", size_of::<String>());
    println!("And an i8 is always {:?} bytes.It is sized", size_of::<i8>());
    println!("But a &str can be anything.'😊' is {:?} bytes.It is not sized", size_of_val("😊"));
    println!("But a &str can be anything.'😂🤣' is {:?} bytes.It is not sized", size_of_val("🤣😂"));
    println!("But a &str can be anything.'Grüß dich!' is {:?} bytes.It is not sized", size_of_val("Grüß dich!"));
}

pub fn string_demo_covert(){
    let my_string : String = "Try to make this a String".into(); // From Trait
    let my_string1 = "some str".to_string();
    let my_string2 = String::from("some str");
}