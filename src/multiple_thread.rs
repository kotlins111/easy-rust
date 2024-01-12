use std::cell::RefCell;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread;

pub fn thread_one_task() {
    thread::spawn(|| println!("I am in thread {:?}", thread::current().name()));
    thread::spawn(|| println!("I am in thread {:?}", thread::current().name()));
    thread::Builder::new()
        .name("thread1".to_string())
        .spawn(move || println!("I am in thread {}", thread::current().name().unwrap()));
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

    for _ in 0..2 {
        // do this twice
        let my_number_clone = Arc::clone(&my_number); // Make the clone before starting the thread
        let handle = thread::spawn(move || {
            // Put the clone in
            for _ in 0..10 {
                *my_number_clone.lock().unwrap() += 1;
            }
        });
        handle_vec.push(handle); // save the handle so we can call join on it outside of the loop
                                 // If we don't push it in the vec, it will just die here
    }

    handle_vec
        .into_iter()
        .for_each(|handle| handle.join().unwrap()); // call join on all handles
    println!("{:?}", my_number);
}

// channel

pub fn simple_channel() {
    let (tx, rx) = channel();

    tx.send(5).expect("Fail to send message");
    println!("{:?}", rx.recv());
}

pub fn two_producer_channel() {
    let (tx, rx): (Sender<&str>, Receiver<&str>) = channel();

    let mut handle_vec = vec![];
    for i in 0..2 {
        let tx_clone = tx.clone();
        let handle = thread::spawn(move || {
            tx_clone.send("This is a str").expect("TODO: panic message");
        });
        handle_vec.push(handle);
    }

    for _ in handle_vec {}
    println!("{}", rx.recv().unwrap());

    let (sender, receiver) = channel();
    let sender_clone = sender.clone();
    let mut handle_vec = vec![];
    let mut results_vec = vec![];

    handle_vec.push(thread::spawn(move || {
        sender.send("Send a &str this time").unwrap();
    }));

    handle_vec.push(thread::spawn(move || {
        sender_clone.send("And here is another &str").unwrap();
    }));

    for _ in handle_vec {
        results_vec.push(receiver.recv().unwrap());
    }

    println!("{:?}", results_vec);
}

pub fn one_billion_zero_to_one() {
    let (tx, rx): (Sender<Vec<u8>>, Receiver<Vec<u8>>) = channel();

    let huge_vec = vec![0; 1_000_000_000];
    let mut newvec = vec![];
    let mut handel_vec = Vec::with_capacity(100);
    for i in 0..100 {
        let tx_clone = tx.clone();
        let mut work: Vec<u8> = Vec::with_capacity(huge_vec.len() / 100);
        work.extend(&huge_vec[i * 10_000_000..(i + 1) * 10_000_000]);
        let handle = thread::spawn(move || {
            for number in work.iter_mut() {
                *number += 1;
            }
            tx_clone.send(work).unwrap();
        });

        handel_vec.push(handle);
    }
    for handle in handel_vec {
        handle.join().unwrap();
    }

    while let Ok(results) = rx.try_recv() {
        newvec.push(results);
    }

    // flatten the result vec
    let newvec: Vec<u8> = newvec.into_iter().flatten().collect();

    println!(
        "{:?} ,{:?}, total length:{}",
        &newvec[0..10],
        &newvec[newvec.len() - 10..],
        newvec.len()
    );

    for number in newvec {
        if number != 1 {
            panic!("There are zero value in buffer");
        }
    }
}
