pub fn print_demo1() {
    let s = "hello world";
    let arr = [1,3,4,5,6];
    println!("{}",s);
    println!("{:?}",arr);
    println!("{:#?}",arr); //pretty print
    println!("{:?}",b"this is my string");
    println!("{:X}",255);
    println!("{:p}",&arr);
    println!("{:06}",43); // padding zero at start make the display length is 6

    //all of these print Hello x____!
    println!("Hello {:5}!", "x");
    let s = format!("Hello {:5}!", "x");
    println!("The format sting len is {}",s.chars().count() );
    println!("Hello {:1$}!", "x", 5);
    println!("Hello {1:0$}!", 5, "x");
    println!("Hello {:width$}!", "x", width = 5);
    let width = 5;
    println!("Hello {:width$}!", "x");

    assert_eq!(format!("Hello {:<5}!", "x"),  "Hello x    !");  // left aligned
    assert_eq!(format!("Hello {:-<5}!", "x"), "Hello x----!");  // left aligned with -
    assert_eq!(format!("Hello {:^5}!", "x"),  "Hello   x  !");  // center aligned with ^
    assert_eq!(format!("Hello {:>5}!", "x"),  "Hello     x!");  // right aligned

    assert_eq!(format!("Hello {:+}!", 5), "Hello +5!");         // sign a number with +
    assert_eq!(format!("{:#x}!", 27), "0x1b!");                 // #x x decimal
    println!("{:#?}",b"this is my string");                     // #? debug pretty print
    assert_eq!(format!("Hello {:05}!", 5),  "Hello 00005!");    // 05 padding to 5 with 0
    assert_eq!(format!("Hello {:05}!", -5), "Hello -0005!");    // signal aware
    assert_eq!(format!("{:#010x}!", 27), "0x0000001b!");
    assert_eq!(format!("{:#010b}",27),"0b00011011");                    // #b binary #o octal

    //format trait
    println!("{:?}",1);             // ? Debug trait
    println!("{:x?}",234);          // x? Debug with lower-case x decimal
    println!("{:X?}",234);          // X? Debug with upper-case x decimal
    println!("{:o}",27);            // o octal
    println!("{:b}",27);            // b binary
    println!("{:x}",1);             // x Lower Hex
    println!("{:X}",1);             // X Upper Hex
    println!("{:p}",&1);            // p Pointer
    println!("{:e}",1001);          // e lower Exp
    println!("{:E}",1001);          // E Upper Exp

}


/// print demo
/// illustrate the position format args
pub fn print_demo2(){

    let father_name = "Vlad";
    let son_name = "Adrian Fahrenheit";
    let family_name = "Tep";
    println!("This is {1} {2}, the son of {0} {2}", father_name,son_name,family_name);

}

/// print demo
/// illustrate the named format args
/// also can declare in println macro
pub fn print_demo3(){
    let city1 = "Seoul";
    let city2 = "Busan";
    let city3 = "Tokyo";
    let country = "Korea";

    println!("
    {city1} is in {country} and {city2} is also in {country}
    but {city3} is not in {country}.")
}

/// a complex print demo
pub fn print_demo4(){
    let title = "Today's News";
    println!("{:-^30}", title);
    let bar = "|";
    println!("{:<15}{:>15}",bar,bar);
    let a = "SEOUL";
    let b = "TOKYO";
    println!("{city1:-<15}{city2:->15}",city1 = a,city2 = b)
}
