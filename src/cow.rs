use std::borrow::Cow;
use std::fmt::format;
use std::ops::Index;

pub fn cow_demo() {
    let sentence_string =
        "Once upon a time, there was a friendly curious crab named Ferris".to_string();
    for word in sentence_string.split(' ') {
        if three_vowels(word) {
            println!("{} has three consecutive vowels!", word);
        }
    }

    for i in 1..7 {
        match modulo_3(i) {
            ref c @ Cow::Borrowed(message) => {
                println!(
                    "{} went in,the cow is borrowed with this message",
                    c.clone().into_owned()
                )
            }
            Cow::Owned(message) => {
                println!("{} went in .The cow is owned with this message", message);
            }
        }
    }

    let cow_str = modulo_3(3);
    let my_string = cow_str.into_owned();
}

// use &str as parameter is better
fn three_vowels(word: &str) -> bool {
    let mut vowel_count = 0;
    for c in word.chars() {
        match c {
            'a' | 'e' | 'i' | 'o' | 'u' => {
                vowel_count += 1;
                if vowel_count >= 3 {
                    return true;
                }
            }
            _ => vowel_count = 0,
        }
    }
    false
}

fn modulo_3(input: u8) -> Cow<'static, str> {
    match input % 3 {
        0 => "Remainder is 0".into(),
        1 => "Remainder is 1".into(),
        remainder => format!("Remainder is {}", remainder).into(),
    }
}
