use crate::impls::AnimalType::{Cat, Dog};

#[derive(Debug)]
struct Animal {
    age: u8,
    animal_type: AnimalType,
}

#[derive(Debug)]
enum AnimalType {
    Cat,
    Dog,
}

impl Animal {
    fn new() -> Self {
        Self {
            age: 10,
            animal_type: Cat,
        }
    }

    fn change_to_dog(&mut self) {
        println!("change animal to dog");
        self.animal_type = Dog;
    }

    fn change_to_cat(&mut self) {
        println!("change animal to cat");
        self.animal_type = Cat;
    }

    fn check_type(&self) {
        // println!("The animal type is{:?}", self.animal_type);
        match self.animal_type {
            Cat => {
                println!("this is a cat")
            }
            Dog => {
                println!("this is a dog")
            }
        }
    }
}
