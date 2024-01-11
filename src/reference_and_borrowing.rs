pub fn reference_demo1() {
    let country = "USA".to_string();
    let ref_one = &country;
    let ref_two = &country;
    println!("{}", ref_one);
}

pub fn mut_reference_demo1() {
    //mutable ref
    let mut my_number = 10;
    let num_ref = &mut my_number;
    *num_ref += 10;
    let num_ref1 = &my_number;
    println!("{}", num_ref1);

    let second_number = 100;
    let triple_reference = &&&second_number;
    println!(
        "Second number == triple_reference? {}",
        second_number == ***triple_reference
    );
}

/// shadowing doesn't destroy the value
pub fn shadowing_reference() {
    let country = "USA".to_string();
    let country_ref = &country;
    let country = 8;
    println!("{country_ref} {country}"); // USA 8
}

/// a value can only have a owner
#[cfg(Error)]
pub fn reference_demo2() {
    fn print_country(country_name: String) {
        print!("{country_name}");
    }

    let country = "German".to_string();
    print_country(country); // fine the ownership moved to the function
    print_country(country); // error
}

pub fn reference_demo3() {
    fn add_hungary(country_name: &mut String) {
        country_name.push_str("-Hungary");
        println!("Now it says:{}", country_name);
    }

    fn add_hungary_take(mut country: String) {
        // take ownership
        country.push_str("-Hungary");
    }

    let mut country = String::from("Austria");
    add_hungary(&mut country);
}

pub fn copy_clone_demo() {
    pub fn get_length_ref(input: &String) -> usize {
        input.split_whitespace().count()
    }

    fn get_length_copy(input: String) -> usize {
        input.clone().split_whitespace().count()
    }

    let mut astr = String::new();
    for _ in 0..50 {
        astr.push_str("Some words");
        let b = get_length_copy(astr.clone()); //clone a new String everytime waste of memory
        let a = get_length_ref(&astr);
    }
}

pub fn variable_init() {
    let my_number;
    fn loop_and_return(mut counter: i32) -> i32 {
        loop {
            counter += 1;
            if counter % 50 == 0 {
                break;
            }
        }
        counter
    }

    {
        let number = {
            //some process
            40
        };
        my_number = loop_and_return(number);
    }

    println!("{}", my_number);
}

// auto dereference
struct Item {
    number: u8,
}

impl Item {
    fn compare_number(&self, other_number: u8) {
        println!(
            "Are {} and {other_number} equal? [{}]",
            self.number,
            self.number == other_number
        );
    }
}
fn reference_demo() {
    let item = Item { number: 10 };

    let reference_item = &item;
    let reference_number = &item.number;
    //println!("{}",reference_number == 10);      // error
    println!("{}", reference_item.number == 10); // fine
}
