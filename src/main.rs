use std::io;
//use std::collections::HashMap;
fn main() {
    let mut words: String = String::new();
    io::stdin().read_line(&mut words).expect("Please Input");
    let clean_word = words.trim().to_string(); 

    if clean_word.is_empty() {
        println!("请输入一个单词！");
        return;
    }

    let pig_latin = to_pig_latin(clean_word);
    println!("{pig_latin}");
}

fn to_pig_latin(word: String) -> String {
    if word.is_empty() {
        return String::new();
    }
    let first_letter: char = word.chars().next().unwrap();
    match first_letter {
        'a' | 'o' | 'e' | 'i' | 'u' => {
            return format!("{}hay", word);
        },
        _ => {
            let other_letters = word.chars().skip(1).collect::<String>();
            return format!("{}{}ay", other_letters, first_letter);
        }
    }
}