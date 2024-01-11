use std::cell::{Cell, RefCell};
use std::fs::read;
use std::sync::{Mutex, RwLock};

struct PhoneModel {
    company_name: String,
    model_name: String,
    screen_size: f32,
    memory: usize,
    date_issued: u32,
    on_sale: Cell<bool>,
}

pub fn interior_mut_demo1() {
    let super_phone_3000 = PhoneModel {
        company_name: "YY Electronics".to_string(),
        model_name: "Super Phone 3000".to_string(),
        screen_size: 7.5,
        memory: 4_000_000,
        date_issued: 2020,
        on_sale: Cell::new(true),
    };

    let user_1 = User {
        id: 1,
        registered: 2020,
        username: "User 1".to_string(),
        active: RefCell::new(true),
    };

    // user_1.active.replace(false);
    let date = 1999;
    user_1.active.replace_with(|_| date > 2000);
    println!("{:?}", user_1);
}

#[derive(Debug)]
struct User {
    id: u32,
    registered: u32,
    username: String,
    active: RefCell<bool>,
}

pub fn running_panic() {
    let user_1 = User {
        id: 1,
        registered: 2020,
        username: "User 1".to_string(),
        active: RefCell::new(true),
    };

    let borrow_one = user_1.active.borrow_mut(); // first mutable borrow - okay
    let borrow_two = user_1.active.borrow_mut(); // second mutable borrow - not okay
}

pub fn mutex_demo() {
    let my_mutex = Mutex::new(5);
    {
        let mut mutex_changer = my_mutex.lock().unwrap();
        println!("{:?}", my_mutex); // data is locked
        println!("{:?}", mutex_changer);
        *mutex_changer = 6;
    } // out scope the lock released

    *my_mutex.lock().unwrap() = 5; // directly deref no need to release lock
    for _ in 0..5 {
        *my_mutex.lock().unwrap() += 1;
    }
    // or use std::mem::drop to release the lock make this go out of scope

    let mut mutex_changer = my_mutex.lock().unwrap();
    println!("{:?}", my_mutex);
    drop(mutex_changer);
    println!("{:?}", my_mutex);

    let mut mutex_changer1 = my_mutex.lock().unwrap(); // mutex_changer has the lock
                                                       //let mut other_mutex_changer = my_mutex.lock().unwrap(); // other_mutex_changer wants the lock
                                                       // the program is waiting
                                                       // and waiting
                                                       // and will wait forever.

    println!("Dead lock.");

    //use try_lock()
    let my_lock = my_mutex.try_lock();
    if let Err(err) = my_lock {
        println!("{:?}", err);
    }
}

pub fn rwlock_demo() {
    let my_rw_lock = RwLock::new(5);

    //any read is fine
    let read1 = my_rw_lock.read().unwrap();
    let read2 = my_rw_lock.read().unwrap();
    // dbg!(read1);
    // dbg!(read2);
    println!("{:?}", my_rw_lock);
    println!("{:?} {:?}", read1, read2);
    drop(read1);
    drop(read2);

    let mut write1 = my_rw_lock.write().unwrap();
    *write1 = 6;
    println!("{:?}", write1);

    let my_rwlock1 = RwLock::new(5);

    let read1 = my_rwlock1.read().unwrap();
    let read2 = my_rwlock1.read().unwrap();

    if let Ok(mut number) = my_rwlock1.try_write() {
        *number += 10;
        println!("Now the number is {}", number);
    } else {
        println!("Couldn't get write access, sorry!")
    };
    let read3 = my_rwlock1.try_read(); // try to acquire lock
    println!("{:?}", read3); // Ok(5)
}
