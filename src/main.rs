use std::collections::HashMap;
use std::io;

fn main() {
    let mut registry: HashMap<String, u32> = HashMap::new();
    registry.insert(String::from("Gig"), 123456789);
    println!("{registry:?}");

    let mut k_input: String = String::new();
    let mut v_input: String = String::new();
    println!("Name: ");
    io::stdin().read_line(&mut k_input).expect("Failed to read name");
    println!("Telephone: ");
    io::stdin().read_line(&mut v_input).expect("Failed to read telephone");

    let key = k_input.trim().to_string(); 
    let value: u32 = match v_input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("错误：请输入有效的数字！");
            return;
        }
    };

    registry.insert(key, value);
    println!("{registry:?}");
}