use crate::collections::LibraryType::Country;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
use std::ptr::hash;

static DATA: [i32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

pub fn array_demo1() {
    // array type
    let array1 = ["one", "two"];
    let array2 = ["one", "two", "three"];

    //array declare
    let my_array = ["a"; 10];
    println!("{:?}", my_array);
}

pub fn array_slice_demo() {
    let mut array_of_ten = [0; 10];
    for i in 0..10 {
        array_of_ten[i] = i;
    }

    let three_to_five = &array_of_ten[2..5];
    let star_at_two = &array_of_ten[1..];
    let end_to_five = &array_of_ten[..5];
    let everything = &array_of_ten[..];
    let six_to_eight = &array_of_ten[5..=7]; // inclusive index
}

pub fn vector_demo() {
    let name1 = String::from("Windy");
    let name2 = String::from("Gnome");

    let names = vec![&name1, &name2];

    // vec!
    let mut numbers = vec![1, 2, 4, 4];

    // convert from array
    let my_vec: Vec<i32> = [1, 2, 3].into();
    let my_vec1: Vec<_> = ['1', '2', '3'].into();
}

pub fn vector_capacity() {
    let mut num_vec = Vec::new();
    println!("{}", num_vec.capacity());
    num_vec.push('a');
    num_vec.push('a');
    num_vec.push('a');
    num_vec.push('a');
    println!("{}", num_vec.capacity()); // 4 elements print 4
    num_vec.push('a');

    num_vec.push('a');
    num_vec.push('a');

    let mut num_vec_with_capacity = Vec::with_capacity(5);
    num_vec_with_capacity.push(1);
    num_vec_with_capacity.push(1);
    num_vec_with_capacity.push(1);
    num_vec_with_capacity.push(1);
    num_vec_with_capacity.push(1);
    num_vec_with_capacity.push(1); // capacity is double 5
    println!("{}", num_vec_with_capacity.capacity());
}

pub fn loop_demo() {
    let mut counter1 = 0;
    let mut counter2 = 0;
    'first_loop: loop {
        counter1 += 1;
        'second_loop: loop {
            counter2 += 1;
            if counter2 > 100 {
                break 'second_loop;
            }
        }
        if counter1 > 10 {
            break 'first_loop;
        }
    }

    println!("counter1 is {} and counter2 is{}", counter1, counter2);
}

// break return value
pub fn loop_break_return_value() {
    let mut counter = 5;
    let my_number = loop {
        counter += 1;
        if counter % 53 == 3 {
            break counter;
        }
    };

    println!("{}", my_number);
}

//hashmap

struct City {
    name: String,
    population: BTreeMap<u32, u32>,
}

pub fn hashmap_demo1() {
    let mut tallinn = City {
        name: "Tallinn".to_string(),
        population: BTreeMap::new(),
    };

    tallinn.population.insert(1372, 3_250);
    tallinn.population.insert(1851, 24_000);
    tallinn.population.insert(2020, 437_169);

    for (year, population) in tallinn.population {
        println!(
            "In the year {} the city of {} had a population of {}",
            year, tallinn.name, population
        );
    }
}

pub fn hashmap_demo2() {
    let candian_cities = vec!["Calgary", "Vancouver", "Gimli"];
    let german_cities = vec!["Karlsruhe", "Bad Doberan", "Bielefeld"];

    let mut city_map = HashMap::new();

    for city in candian_cities {
        city_map.insert(city, "Canada");
    }
    for city in german_cities {
        city_map.insert(city, "German");
    }

    println!("{:?}", city_map["Bielefeld"]);
    println!("{:?}", city_map.get("Bielefeld"));
    println!("{:?}", city_map.get("Bielefelld"));
    // println!("{:?}",city_map["Bielefelld"]); // entry not found for key -> panic
}

pub fn hashmap_overwrite() {
    let mut book_map = HashMap::new();
    book_map.insert(1, "L'Allemagne Moderne");

    if book_map.get(&1).is_none() {
        book_map.insert(1, "Wind has gone");
    }
    book_map.insert(1, "L'Allemagne Moderne");
    book_map.insert(1, "L'Allemagne Moderne");

    let old_value = book_map.insert(1, "Eye of the world");

    println!("old value is {}", old_value.unwrap());

    println!("current value is {}", book_map[&1]);
}

pub fn hashmap_entry_demo() {
    let book_list = [
        "L'Allemagne Moderne",
        "Le Petit Prince",
        "Eye of the world",
        "Eye of the world",
    ];

    let mut book_map = HashMap::new();
    for book in book_list {
        let book_num = book_map.entry(book).or_insert(0);
        *book_num += 1;
    }

    for (book, number) in book_map {
        println!("The book {book} 's number is {number}");
    }
}

pub fn hashmap_demo3() {
    let data = [
        ("male", 9),
        ("female", 5),
        ("male", 0),
        ("female", 6),
        ("female", 5),
        ("male", 10),
    ];

    let mut survey_hash = HashMap::new();
    for item in data {
        survey_hash.entry(item.0).or_insert(Vec::new()).push(item.1);
    }

    for (gender, numbers) in survey_hash {
        println!("{:?} -> {:?}", gender, numbers);
    }
}

pub fn hashset_demo1() {
    let many_numbers = vec![
        94, 42, 59, 64, 32, 22, 38, 5, 59, 49, 15, 89, 74, 29, 14, 68, 82, 80, 56, 41, 36, 81, 66,
        51, 58, 34, 59, 44, 19, 93, 28, 33, 18, 46, 61, 76, 14, 87, 84, 73, 71, 29, 94, 10, 35, 20,
        35, 80, 8, 43, 79, 25, 60, 26, 11, 37, 94, 32, 90, 51, 11, 28, 76, 16, 63, 95, 13, 60, 59,
        96, 95, 55, 92, 28, 3, 17, 91, 36, 20, 24, 0, 86, 82, 58, 93, 68, 54, 80, 56, 22, 67, 82,
        58, 64, 80, 16, 61, 57, 14, 11,
    ];

    let mut hashset = HashSet::new();
    let mut ord_hashset = BTreeSet::new();
    for x in many_numbers {
        hashset.insert(x);
        ord_hashset.insert(x);
    }

    let hashset_length = hashset.len();
    println!("There are {hashset_length} unique items in the set");

    for x in ord_hashset {
        println!("{}", x);
    }
    //check what numbers are missing
    let mut missing_vec = vec![];
    for number in 0..100 {
        if hashset.get(&number).is_none() {
            // the number is not in the set
            missing_vec.push(number);
        }
    }
    print!("It does not contain");
    for number in missing_vec {
        print!("{number}#");
    }
}

fn show_remainder(input: &BinaryHeap<i32>) -> Vec<i32> {
    let mut remainder_vec = vec![];
    for number in input {
        remainder_vec.push(*number);
    }
    remainder_vec
}

pub fn binary_heap_demo() {
    let many_numbers = [0, 5, 10, 15, 20, 25, 30]; // These numbers are in order
    let mut my_heap = BinaryHeap::new();
    for number in many_numbers {
        my_heap.push(number);
    }

    while let Some(value) = my_heap.pop() {
        // pops from front
        println!(
            "Popped off {value}. Remaining numbers are: {:?}",
            show_remainder(&my_heap)
        );
    }
}

//VecDeque

pub fn vec_remove_demo1() {
    let mut my_vec = vec![0; 6_00_000];
    for i in 0..6_00_000 {
        my_vec.remove(0); // remove first element and shift rest to the left
    }
}

pub fn vec_deque_remove_demo() {
    let mut my_vec = VecDeque::from(vec![0; 6_00_000_000]);
    for i in 0..6_00_000_000_i64 {
        my_vec.pop_front(); // quite fast (about 4 seconds)
    }
}

fn check_remaining(input: &VecDeque<(&str, bool)>) {
    for (task, done) in input {
        if !done {
            println!("You must {}", task);
        }
    }
}

fn done(input: &mut VecDeque<(&str, bool)>) {
    let mut task_done = input.pop_back().unwrap();
    task_done.1 = true;
    input.push_front(task_done);
}

pub fn vec_deque_task_demo() {
    let mut my_deque = VecDeque::with_capacity(10);
    let things_to_do = vec![
        "send email to pm",
        "submit the concur report",
        "phone lucy back",
    ];
    for x in things_to_do {
        my_deque.push_front((x, false));
    }

    done(&mut my_deque);
    done(&mut my_deque);
    check_remaining(&my_deque);

    for task in my_deque {
        println!("{:?}", task);
    }
}

// imperative style
fn sum_vec_imperative() {
    let mut new_vec = vec![];
    let mut counter = 1;
    while counter < 11 {
        new_vec.push(counter);
        counter += 1;
    }

    println!("{:?}", new_vec);
}

// functional style
fn sum_vec_functional() {
    let new_vec = (1..=10).collect::<Vec<i32>>();
    println!("{:?}", new_vec);
}

fn collection_chain() {
    let my_vec = DATA.clone();
    let new_vec = my_vec.into_iter().skip(3).take(4).collect::<Vec<i32>>();

    println!("{:?}", new_vec);
}

// iterators
fn iterator_demo1() {
    let vector1 = vec![1, 2, 3];
    let vector2 = vector1.iter().map(|x| x + 1).collect::<Vec<i32>>();
    let vector3 = vector1.into_iter().map(process).collect::<Vec<i32>>(); // into_iter destroy vector1

    let mut vector4 = vec![5, 1, 2];
    vector4.iter_mut().for_each(|x| *x += 1);
}

fn process(x: i32) -> i32 {
    x * 10
}

// impl Iterator

#[derive(Debug)]
struct Library {
    lib_type: LibraryType,
    books: Vec<String>,
}

#[derive(Debug, Clone)]
enum LibraryType {
    City,
    Country,
}

impl Iterator for Library {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        match self.books.pop() {
            None => None,
            Some(book) => Some(book + " is found"),
        }
    }
}

struct Alternate {
    state: i32,
}

impl Iterator for Alternate {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        let val = self.state;
        self.state += 1;

        if val % 2 == 0 {
            Some(val)
        } else {
            None
        }
    }
}

pub fn iterator_demo2() {
    let lib = Library {
        lib_type: Country,
        books: vec![
            "Eyes for eyes".to_string(),
            "Dimension 2".to_string(),
            "Rust Programming".to_string(),
        ],
    };

    for book in lib {
        println!("{}", book);
    }
}
