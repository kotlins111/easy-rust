use std::collections::HashMap;
use std::fmt::format;

pub fn closure_demo1() {
    let my_closure = |x: fn(i32) -> f32| println!("the value is {:?}", x(10));
    my_closure(|x| x as f32 * 0.2); // high level function

    let outre_variable = 12;
    let my_closure2 = || {
        let number = 7;
        println!("The two number are {number} and {outre_variable}");
    };

    my_closure2();
}

pub fn take_fourth() {
    let my_vec = [8, 9, 10, 11, 12];
    let my_vec = my_vec.iter().map(|x| x * 2).collect::<Vec<i32>>();
    my_vec
        .iter()
        .enumerate()
        .for_each(|(index, number)| println!("Index number {index} has number {number}"));
    let fourth = my_vec.get(3).unwrap_or_else(|| {
        if my_vec.first().is_some() {
            &my_vec[0]
        } else {
            &0
        }
    });

    println!("{}", fourth);
}

pub fn collect_hashmap() {
    let some_numbers = vec![0, 1, 2, 3, 4, 5];
    let some_words = vec!["zero", "one", "two", "three", "four", "five"];
    let number_word_hashmap = some_numbers
        .into_iter()
        .zip(some_words)
        .collect::<HashMap<_, _>>();
    println!("{:?}", number_word_hashmap);
}

pub fn collect_chars() {
    let numbers = "140399923481800622623218009598281";
    for (index, number) in numbers.char_indices() {
        match (index % 3, number) {
            (0..=1, number) => {
                print!("{}", number);
            }
            _ => println!("{}\t", number),
        }
    }
}

pub fn filter_month() {
    let months = vec![
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    let filtered = months
        .into_iter()
        .filter(|month| month.len() < 5)
        .filter(|month| month.contains('u'))
        .collect::<Vec<&str>>();
    println!("{:?}", filtered);
}

struct Company {
    name: String,
    ceo: Option<String>,
}

impl Company {
    fn new(name: &str, ceo: &str) -> Self {
        let ceo = match ceo {
            "" => None,
            ceo => Some(ceo.to_string()),
        };

        Self {
            name: name.to_string(),
            ceo,
        }
    }

    fn get_ceo(&self) -> Option<String> {
        self.ceo.clone()
    }
}

pub fn filter_map_demo1() {
    let company_vec = vec![
        Company::new("Umbrella Corporation", "Unknown"),
        Company::new("Ovintiv", "Doug Suttles"),
        Company::new("The Red-Headed League", ""),
        Company::new("Stark Enterprises", ""),
    ];

    let all_the_ceos = company_vec
        .iter()
        .filter_map(|company| company.get_ceo())
        .collect::<Vec<String>>();

    let mut result_vec = vec![];
    let mut result_vec1 = vec![];
    company_vec
        .iter()
        .for_each(|company| result_vec.push(company.get_ceo().ok_or("No Ceo Found!")));
    company_vec.iter().for_each(|company| {
        result_vec1.push(company.get_ceo().ok_or_else(|| {
            let err_message = format!("No Ceo Found for {}", company.name);
            err_message
        }))
    });
    for item in result_vec {
        println!("{:?}", item);
    }

    for item in result_vec1 {
        println!("{:?}", item);
    }
}

pub fn filter_map_demo2() {
    let user_input = vec![
        "8.9",
        "Nine point nine five",
        "8.0",
        "7.6",
        "eleventy-twelve",
    ];
    let actual_number = user_input
        .into_iter()
        .filter_map(|input| input.parse::<f32>().ok())
        .collect::<Vec<f32>>();
}

pub fn closure_demo2() {
    let new_vec = vec![8, 9, 10];
    let number_to_add = 5;
    let mut empty_vec = vec![];
    for index in 0..5 {
        empty_vec.push(
            new_vec
                .get(index)
                .and_then(|number| Some(number + 1))
                .and_then(|number| Some(number + number_to_add)),
        );
    }
    for x in empty_vec {
        println!("{:?}", x);
    }
}

pub fn closure_demo3() {
    let first_try = vec![
        Some("success!"),
        None,
        Some("success!"),
        Some("success!"),
        None,
    ];
    let second_try = vec![
        None,
        Some("success!"),
        Some("success!"),
        Some("success!"),
        Some("success!"),
    ];
    let third_try = vec![
        Some("success!"),
        Some("success!"),
        Some("success💕!"),
        Some("success👍!"),
        None,
    ];
    for i in 0..first_try.len() {
        println!("{:?}", first_try[i].and(second_try[i]).and(third_try[i]));
    }
}

fn in_char_vec(char_vec: &Vec<char>, check: char) {
    println!(
        "Is {} inside? [{}]",
        check,
        char_vec.iter().any(|&char| char == check)
    );
}

pub fn closure_demo4() {
    let char_vec = ('a'..'働').collect::<Vec<char>>();
    println!("The length of char vec is {}", char_vec.len());
    in_char_vec(&char_vec, 'a');
    in_char_vec(&char_vec, '老');
    in_char_vec(&char_vec, '😒');

    let smaller_vec = ('A'..='Z').collect::<Vec<char>>();
    println!(
        "All alphabetic? {}",
        smaller_vec.iter().all(|e| e.is_alphabetic())
    );
    println!(
        "All less than the character 国? {}",
        smaller_vec.iter().all(|e| e < &'国')
    );
}

pub fn iter_rev_demo() {
    let mut big_vec = vec![6; 1_000];
    big_vec.push(5);
    let mut iter = big_vec.iter().rev();
    println!("{:?}", iter.next());
    println!("{:?}", iter.next());
}

pub fn find_and_position() {
    let num_vec = vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100];
    let mut iter = num_vec.iter();
    println!("{:?}", iter.find(|&number| number % 3 == 0));
    println!("{:?}", iter.next());
    // do not use the iter after it was iterated, next statement will start at where it was at, not from the start
    let position: usize = iter.position(|e| e * 3 == 30).unwrap_or(1);
    println!("{:?}", position);

    println!("{:?}", iter.next());
    let sum = num_vec.iter().fold(1, |current, next| current + next);
    println!("{}", sum);
}

pub fn cycle_vec() {
    let even_odd = vec!["even", "odd"];
    let even_odd_vec = (0..6)
        .zip(even_odd.into_iter().cycle())
        .collect::<Vec<(i32, &str)>>();

    println!("{:?}", even_odd_vec);
}

pub fn chars_vec_demo() {
    let ten_chars = ('a'..).take(10).collect::<Vec<char>>();
    let skip_then_ten_chars = ('💕'..).skip(3).take(10).collect::<Vec<char>>();

    println!("{:?}", ten_chars);
    println!("{:?}", skip_then_ten_chars);
}

pub fn reduce_or_fold() {
    let a_string = " I don't have any dash in me.";

    let dash_string = a_string.chars().fold("-".to_string(), |mut current, next| {
        current.push(next);
        current.push('-');
        current
    });

    println!("{}", dash_string);
}

pub fn while_iter() {
    let vec1 = [-1i32, -2, -3, 4, 5, 6];

    let iter1 = vec1.iter().take_while(|e| e.is_negative());
    let mut iter2 = iter1.cloned(); // clone a iter

    let iter3 = iter2.by_ref(); // get a mut ref from an iterator
    let my_vec: Vec<_> = iter3.take(1).collect();

    let positive_numbers: Vec<_> = vec1.iter().skip_while(|e| e.is_negative()).collect();
    let negeative_numbers: Vec<_> = vec1.iter().map_while(|e| less_than_zero(*e)).collect(); // yield when evaluate Some(_) map().take_while(|e| e.is_some())
    println!("{:?}", iter2.next());
    println!("{:?}", positive_numbers);
    println!("{:?}", negeative_numbers);
}

pub fn vector_cut() {
    let num_vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0];

    for chunk in num_vec.chunks(3) {
        println!("{:?}", chunk);
    }
    println!();
    for window in num_vec.windows(3) {
        println!("{:?}", window);
    }
}

#[derive(Debug)]
struct Names {
    one_word: Vec<String>,
    two_word: Vec<String>,
    three_word: Vec<String>,
}

pub fn match_indices_demo() {
    let rules = "Rule number 1: No fighting. Rule number 2: Go to bed at 8 pm. Rule number 3: Wake up at 6 am.";
    let rule_locations: Vec<(_, _)> = rules.match_indices("Rule").collect();
    println!("{:?}", rule_locations);
    let vec_of_names = [
        "Caesar",
        "Frodo Baggins",
        "Bilbo Baggins",
        "Jean-Luc Picard",
        "Data",
        "Rand Al'Thor",
        "Paul Atreides",
        "Barack Hussein Obama",
        "Bill Jefferson Clinton",
    ];

    let mut iter = vec_of_names.iter().peekable();

    let mut all_names = Names {
        one_word: vec![],
        two_word: vec![],
        three_word: vec![],
    };

    while let Some(name) = iter.peek() {
        // println!("{}", name);
        match name.match_indices(' ').collect::<Vec<_>>().len() {
            0 => all_names.one_word.push(name.to_string()),
            1 => all_names.two_word.push(name.to_string()),
            _ => all_names.three_word.push(name.to_string()),
        }
        iter.next();
    }

    println!("{:?}", all_names);
}

pub fn peek_vector_demo() {
    let just_numbers = vec![1, 5, 100];
    let mut iter = just_numbers.iter().peekable();

    println!("I love the number {}", iter.peek().unwrap());
    iter.next();
    if let Some(pointer) = iter.peek_mut() {
        // get a mut reference to next element
        *pointer = &1000;
    }
    println!(
        "The second number was updated, now it is {}",
        iter.peek().unwrap()
    );

    let locations = [
        ("Nevis", 25),
        ("Taber", 8428),
        ("Markerville", 45),
        ("Cardston", 3585),
    ];

    let mut location_iter = locations.iter().peekable();
    while location_iter.peek().is_some() {
        match location_iter.peek() {
            None => {
                break;
            }
            Some((name, number)) if *number < 100 => {
                println!("Found a hamlet: {} with {} people.", name, number);
            }
            Some((name, number)) => println!("Found a town: {name} with {number} people."),
        }
        location_iter.next();
    }
}

fn less_than_zero(input: i32) -> Option<i32> {
    match input < 0 {
        true => Some(input),
        false => None,
    }
}

pub fn inspect_demo() {
    let lines = ["1", "2", "a"];

    let sum: i32 = lines
        .iter()
        .map(|line| line.parse::<i32>())
        .inspect(|num| {
            if let Err(ref e) = *num {
                println!("Parsing error: {e}");
            }
        })
        .filter_map(Result::ok)
        .sum();

    println!("Sum: {sum}");
}

pub fn condition_ref_binding() {
    let maybe_name = Option::from(String::from("Alice"));
    // The variable 'maybe_name' is consumed here ...
    match maybe_name {
        Some(ref n) => println!("Hello, {n}"),
        _ => println!("Hello, world"),
    } // use ref to borrow 'maybe_name'
      // ... and is now unavailable.
    println!("Hello again, {}", maybe_name.unwrap_or("World".into()));
}

fn do_something<F>(f: F)
where
    F: FnOnce(),
{
    f();
}

fn call_do_something() {
    let some_vec = vec![8, 9, 10];
    do_something(|| {
        some_vec
            .into_iter()
            .for_each(|x| println!("The number is {x}"));
    })
}

#[derive(Debug)]
struct City {
    name: String,
    years: Vec<u32>,
    populations: Vec<u32>,
}

impl City {
    fn new(name: &str, years: Vec<u32>, populations: Vec<u32>) -> Self {
        Self {
            name: name.to_string(),
            years,
            populations,
        }
    }

    fn city_data<F>(&mut self, mut f: F)
    where
        F: FnMut(&mut Vec<u32>, &mut Vec<u32>),
    {
        f(&mut self.years, &mut self.populations)
    }
}

pub fn closure_function_demo() {
    let years = vec![
        1372, 1834, 1851, 1881, 1897, 1925, 1959, 1989, 2000, 2005, 2010, 2020,
    ];
    let populations = vec![
        3_250, 15_300, 24_000, 45_900, 58_800, 119_800, 283_071, 478_974, 400_378, 401_694,
        406_703, 437_619,
    ];
    // Now we can create our city
    let mut tallinn = City::new("Tallinn", years, populations);

    tallinn.city_data(|city_years, city_populations| {
        let new_vec: Vec<(_, _)> = city_years
            .iter()
            .zip(city_populations.iter())
            .take(5)
            .collect();
        println!("{:?}", new_vec);
    });

    // mut the data
    tallinn.city_data(|x, y| {
        x.push(2030);
        y.push(500_000);
    });

    // remove 1834 data
    tallinn.city_data(|x, y| {
        let position_option = x.iter().position(|x| *x == 1834);
        if let Some(position) = position_option {
            println!("Going to remove {} at position {:?}", x[position], position);

            x.remove(position);
            y.remove(position);
        }
    });

    println!(
        "Years left are {:?} \n populations left are {:?}",
        tallinn.years, tallinn.populations
    );
}
