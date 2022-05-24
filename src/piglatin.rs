pub fn to_pig_latin(word: &str) -> Option<String> {
    let mut pig_latined_word = String::from(word);
    let vowels = vec!['a', 'e', 'i', 'o', 'u'];
    match word.chars().next() {
        Some(c) => {
            if vowels.contains(&c) {
                pig_latined_word.push_str("-hay");
                Some(pig_latined_word)
            } else {
                pig_latined_word.push_str("-fay");
                Some(pig_latined_word[1..].to_owned())
            }
        }
        None => None,
    }
}
