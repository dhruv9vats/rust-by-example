pub fn to_pig_latin(word: &str) -> Option<String> {
    let mut pig_latined_word = String::from(word);
    let vowels = vec!['a', 'e', 'i', 'o', 'u'];
    let first_char = word.chars().next()?;

    if vowels.contains(&first_char) {
        pig_latined_word.push_str("-hay");
        Some(pig_latined_word)
    } else {
        pig_latined_word.push_str("-fay");
        Some(pig_latined_word[1..].to_owned())
    }
}
