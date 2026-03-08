#[derive(Debug)]
struct Cat {
    name: String,
    color: String,
    meow: String,
    age: u32,
    is_alive: bool
}
impl Cat {
    fn new(name: &str, color: &str, age: u32) -> Self {
        Cat {
            name: name.to_string(),
            color: color.to_string(),
            meow: String::from("Meow~"),
            age,
            is_alive: true
        }
    }
    fn say_meow(&self) {
        println!("{}",self.meow);
    }
    fn judge_age(&self) -> String {
        if self.age <= 1 {
            String::from("This is a kitten.")
        } else if self.age <= 7 {
            String::from("This is an Adult cat.")
        } else {
            String::from("This is an old cat.")
        }
    }
    fn about_cat(&self) {
        if self.is_alive {
            println!(
                "This is a(n) {} named {}.It's {} now.", self.color, self.name, self.age
            );
        } else {
            println!(
                "This is a(n) {} named {}.Sadly, it's {} forever.", self.color, self.name, self.age
            );
        }
    }
    fn pass_away(&mut self) {
        if self.is_alive {
            self.is_alive = false;
            println!("💔 {} has passed away...", self.name);
        }
    }
}
fn main() {
    let mut my_cat_1: Cat = Cat::new(
        "Gig",
        "Yellow",
        10
    );
    let age_word: String = my_cat_1.judge_age();
    my_cat_1.say_meow();
    println!("{}", age_word); 
    my_cat_1.about_cat();

    my_cat_1.pass_away();
    my_cat_1.about_cat();
    println!("{my_cat_1:?}")
}