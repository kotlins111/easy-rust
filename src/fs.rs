

use std::fs::{write, File, OpenOptions};
use std::io;
use std::io::{Read, Result, Write};

pub fn write_txt() -> Result<()> {
    let mut file = File::create("hello.txt")?;
    file.write_all("Hello rust".as_bytes())?;

    File::create("myfilename.txt")?.write_all(b"Let's put this in the file")?;

    write("calvin_with_dad.txt",
          "Calvin: Dad, how come old photographs are always black and white? Didn't they have color film back then?
Dad: Sure they did. In fact, those photographs *are* in color. It's just the *world* was black and white then.
Calvin: Really?
Dad: Yep. The world didn't turn color until sometimes in the 1930s...")?;
    Ok(())
}


pub fn read_txt() -> io::Result<()>{
    // let mut calvin_file = File::open("calvin_with_dad.txt")?;
    let mut calvin_file = OpenOptions::new()
        .append(true) // Now we can write without deleting it
        .read(true)
        .open("calvin_with_dad.txt")?;
    // let mut new_file = File::create("new_calvin_with_dad.txt")?;
    let mut calvin_string = String::new();
    calvin_file.read_to_string(&mut calvin_string)?;
    let mut calvin_string : String = calvin_string.split_whitespace().map(|word| word.to_uppercase()).collect();
    calvin_string.insert(0, '\n');
    calvin_file.write_all(calvin_string.as_bytes())?;
    // let new_calvin_file = OpenOptions::new().write(true).create_new(true).open("new_calvin_with_dad.txt")?;
    Ok(())
}