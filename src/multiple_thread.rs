use std::cell::RefCell;
use std::sync::{Arc, Mutex};
use std::thread;

pub fn thread_one_task() {
    thread::spawn(|| println!("I am in thread {:?}", thread::current().name()));
    thread::spawn(|| println!("I am in thread {:?}", thread::current().name()));
    thread::Builder::new().name("thread1".to_string()).spawn(move || { println!("I am in thread {}", thread::current().name().unwrap()) });
}

pub fn thread_shared_reference() {
    let number = Arc::new(Mutex::new(1));
    let mut handle_vec = vec![];
    for _ in 0..2 {
        let number_clone = Arc::clone(&number);
        let handle = thread::spawn(move || {
            for _ in 0..10 {
                concurrent_update_number(&number_clone);
            }
        });

        handle_vec.push(handle);
    }

    println!("{:?}", number);
}

type Shareable<T> = Arc<Mutex<T>>;

fn concurrent_update_number(input: &Shareable<i32>) {
    *input.lock().unwrap() += 1;
}

pub fn multiple_add_number() {
    let my_number = Arc::new(Mutex::new(0));
    let mut handle_vec = vec![]; // JoinHandles will go in here

    for _ in 0..2 { // do this twice
        let my_number_clone = Arc::clone(&my_number); // Make the clone before starting the thread
        let handle = thread::spawn(move || { // Put the clone in
            for _ in 0..10 {
                *my_number_clone.lock().unwrap() += 1;
            }
        });
        handle_vec.push(handle); // save the handle so we can call join on it outside of the loop
        // If we don't push it in the vec, it will just die here
    }

    handle_vec.into_iter().for_each(|handle| handle.join().unwrap()); // call join on all handles
    println!("{:?}", my_number);
}