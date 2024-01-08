use easy_rust::collections::{loop_demo, vector_capacity};
use easy_rust::print::print_demo4;
use easy_rust::reference_and_borrowing::{mut_reference_demo1, reference_demo1, variable_init};
use easy_rust::strings::strings_demo1;
use easy_rust::types::{city_demo, match_demo3, Point, show_moods, stars_size, struct_demo1, tuple_demo1};

// static allocate fixed memory
static SEASONS : [&str;4] = ["Spring","Summer","Fall","Winter"];
const NUMBER_OF_MONTHS : u8 = 12;


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
    struct_demo1();
    show_moods();
    stars_size();
    city_demo();
}

