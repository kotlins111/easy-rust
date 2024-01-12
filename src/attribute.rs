#![allow(dead_code)]
#![allow(unused_variables)]
/// If you write an attribute with just # then it will affect the code on the next line.
/// But if you write it with #! then it will affect everything in its own space.

struct Struct1 {} // Create five structs
struct Struct2 {}
struct Struct3 {}
struct Struct4 {}
struct Struct5 {}

fn unused_variables() {
    let char1 = 'ん'; // and four variables. We don't use any of them but the compiler is quiet
    let char2 = ';';
    let some_str = "I'm just a regular &str";
}
