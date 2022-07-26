pub mod piglatin;
pub mod structs;

fn main() {
    structs::do_something_with_structs();

    let words = vec![
        "not_vowel",
        "a_vowel",
        "just",
        "normal",
        "stuff",
        "and",
        "sun",
    ];
    for word in words.into_iter() {
        if let Some(pig_latin_word) = piglatin::to_pig_latin(word) {
            println!("{pig_latin_word}")
        }
    }
}
