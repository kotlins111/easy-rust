// use easy_rust::closure::closure_demo1;

use easy_rust::closure::{
    chars_vec_demo, closure_demo1, closure_demo3, closure_demo4, closure_function_demo,
    collect_chars, collect_hashmap, cycle_vec, filter_map_demo1, filter_month, find_and_position,
    inspect_demo, iter_rev_demo, match_indices_demo, peek_vector_demo, reduce_or_fold, take_fourth,
    vector_cut, while_iter,
};
use easy_rust::cow::cow_demo;
use easy_rust::interior_mut::{interior_mut_demo1, mutex_demo, running_panic, rwlock_demo};
use easy_rust::multiple_thread::{
    multiple_add_number, one_billion_zero_to_one, simple_channel, thread_one_task,
    thread_shared_reference, two_producer_channel,
};
use easy_rust::smart_pointer::rc_demo2;

use easy_rust::fs::{read_txt, write_txt};
use easy_rust::print::{print_random_chars, random_i32_to_unicode};
use easy_rust::traits::{
    add_struct_with_i32, default_struct, default_value, only_impl_deref_on_smart_pointers,
    use_closure,
};
use easy_rust::types::swap_demo;
use easy_rust::userinput::{args_from_command_line, get_all_env, scan_user_input};
use std::thread;
use std::time::{Duration, Instant};

// use std::num::{IntErrorKind, ParseIntError};
// use easy_rust::anti_null_exception::parse_str;
// use easy_rust::collections::{binary_heap_demo, hashmap_demo1, hashmap_demo2, hashmap_demo3, hashmap_entry_demo, hashmap_overwrite, hashset_demo1, iterator_demo2, loop_demo, vec_deque_remove_demo, vec_deque_task_demo, vector_capacity};
// use easy_rust::print::print_demo4;
// use easy_rust::reference_and_borrowing::{mut_reference_demo1, reference_demo1, variable_init};
// use easy_rust::strings::strings_demo1;
// use easy_rust::traits::{city_vec_to_country, vec_from_trait_demo};
// use easy_rust::types::{
//     city_demo, match_demo3, show_moods, stars_size, struct_demo1, tuple_demo1, Point,
// };
//
// static allocate fixed memory
static SEASONS: [&str; 4] = ["Spring", "Summer", "Fall", "Winter"];
const NUMBER_OF_MONTHS: u8 = 12;

macro_rules! return_six {
    () => {
        6
    };
}

macro_rules! check {
    ($input1:ident, $input2:expr) => {
        println!(
            "Is {:?} equal to {:?}? {:?}",
            $input1,
            $input2,
            $input1 == $input2
        );
    };
}

macro_rules! print_anything {
    ($($input1:tt),*) => {
        let output = stringify!($($input1),*);
        println!("{}", output);
    };
}
macro_rules! make_a_function {
    ($name:ident, $($input:tt),*) => { // First you give it one name for the function, then it checks everything else
        fn $name() {
            let output = stringify!($($input),*); // It makes everything else into a string
            println!("{}", output);
        }
    };
}

fn main() {
    //print_demo1();
    // print_demo4();
    // strings_demo1();
    // reference_demo1();
    // mut_reference_demo1();
    // variable_init();
    // vector_capacity();
    // loop_demo();
    // tuple_demo1();
    // let p = Point{ x: 1,y:0};
    // match_demo3(p);
    // match_demo3(p);
    // struct_demo1();
    // show_moods();
    // stars_size();
    // city_demo();
    // hashmap_overwrite();
    // hashmap_demo2();
    // hashmap_entry_demo();
    // hashmap_demo3();
    // hashset_demo1();
    // binary_heap_demo();
    // vec_deque_task_demo();
    // let result = parse_str("aaa").err();
    // match result {
    //     None => {}
    //     Some(err) => {
    //         match err.kind() {
    //             IntErrorKind::Empty => { println!("given empty str") }
    //             IntErrorKind::InvalidDigit => { println!("invalid digit character given") }
    //             IntErrorKind::PosOverflow => { println!("the number is too largo to store in this type") }
    //             IntErrorKind::NegOverflow => { println!("the number is too small to store in this type, try to use i64") }
    //             IntErrorKind::Zero => { println!("the str value is zero") }
    //             &_ => println!("general error")
    //         }
    //     }
    // }

    // vec_from_trait_demo();
    // city_vec_to_country();
    // iterator_demo2();
    // take_fourth();
    // collect_hashmap();
    // collect_chars();
    // closure_demo1();
    //  filter_month();
    //  filter_map_demo1();
    //  cycle_vec();
    // chars_vec_demo();
    // find_and_position();
    // reduce_or_fold();
    // while_iter();
    // vector_cut();
    // match_indices_demo();
    // peek_vector_demo();
    // inspect_demo();

    // interior_mut_demo1();
    // running_panic();
    // mutex_demo();
    // rwlock_demo();
    // cow_demo();
    // rc_demo2();

    // closure_function_demo();
    // use_closure(10, "double");
    // thread_one_task();
    // simple_channel();

    // two_producer_channel();
    // let now = Instant::now();
    // one_billion_zero_to_one();
    // let end = now.elapsed().as_millis();
    // println!("Time elapsed: {} ms", end);
    // thread_shared_reference();
    // multiple_add_number();
    // println!("{}", thread::current().name().unwrap());

    // thread::sleep(Duration::from_secs(5));
    // default_value();
    // default_struct();

    // add_struct_with_i32();
    // only_impl_deref_on_smart_pointers();
    // random_i32_to_unicode();
    // swap_demo();

    //macros

    // let six = return_six!();
    // println!("{}", six);
    //
    // check!(six, 6);
    //
    // make_a_function!(print_it, 5, 5, 6, I);
    // print_it();
    // print_random_chars();

    // scan_user_input();
    // args_from_command_line();

    // get_all_env();

    read_txt();
}
