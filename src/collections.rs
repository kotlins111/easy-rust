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
            if counter2 > 100 { break 'second_loop; }
        }
        if counter1 > 10 { break 'first_loop; }
    }

    println!("counter1 is {} and counter2 is{}", counter1, counter2);
}


// break return value
pub fn loop_break_return_value() {
    let mut counter = 5;
    let my_number = loop {
        counter += 1;
        if counter % 53 == 3 { break counter; }
    };

    println!("{}", my_number);
}