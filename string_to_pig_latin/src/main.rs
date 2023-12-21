fn convert_to_pig_latin(data: &mut String) {
    let mut uppercase_alphabets: Vec<char> = Vec::new();
    let mut lowercase_alphabets: Vec<char> = Vec::new();
    let uppercase_vowels = vec!['A', 'E', 'I', 'O', 'U'];
    let lowercase_vowels = vec!['a', 'e', 'i', 'o', 'u'];

    for i in 0..26 {
        let mut character = i + 65;
        if let Some(c) = char::from_u32(character) {
            uppercase_alphabets.push(c);
        }

        character += 32;
        if let Some(c) = char::from_u32(character) {
            lowercase_alphabets.push(c);
        }
    }

    let first_ch = data
        .chars()
        .next()
        .expect("Couln't get the first character.");

    if uppercase_vowels.contains(&first_ch) || lowercase_vowels.contains(&first_ch) {
        data.push_str("-hay");
    } else {
        let _ = data.remove(0);
        data.push_str(&format!("-{first_ch}ay"));
    }

    println!("{}", data);
}

fn main() {
    let mut word = String::from("apple");
    let mut word_2 = String::from("first");
    let mut word_3 = String::from("नमस्ते");
    let mut word_4 = String::from("Elephant");

    convert_to_pig_latin(&mut word);
    convert_to_pig_latin(&mut word_2);
    convert_to_pig_latin(&mut word_3); // need to handle UTF-8 encoding here
    convert_to_pig_latin(&mut word_4);
}
