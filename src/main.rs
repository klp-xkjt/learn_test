#[derive(Debug, Clone)] 
enum LifeStatus {
    Alive,
    Dead
}
enum AgeStage {
    Kitten,   // <= 1
    Adult,    // 2 - 7
    Senior,   // > 7
}

#[derive(Debug)]
struct Cat {
    name: String,
    color: String,
    meow: String,
    age: u32,
    status: LifeStatus
}
impl Cat {
    fn new(name: &str, color: &str, age: u32) -> Self {
        Cat {
            name: name.to_string(),
            color: color.to_string(),
            meow: String::from("Meow~"),
            age,
            status: LifeStatus::Alive
        }
    }
    fn say_meow(&self) {
        match self.status {
            LifeStatus::Alive => println!("{}", self.meow),
            LifeStatus::Dead => println!("💤 {} is sleeping forever...", self.name)
        }
    }
    fn judge_age(&self) -> AgeStage {
        match self.age {
            0..=1 => AgeStage::Kitten,
            2..=7 => AgeStage::Adult,
            _ => AgeStage::Senior
        }
    }
    fn age_description(&self) -> &str {
        match self.judge_age() {
            AgeStage::Kitten => "This is a kitten.",
            AgeStage::Adult => "This is an adult.",
            _ => "This is an old cat"
        }
    }
    fn about_cat(&self) {
        if let LifeStatus::Alive = self.status {
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
        if let LifeStatus::Alive = self.status {
            println!("💔 {} has passed away...", self.name);
            self.status = LifeStatus::Dead;
        } else {
            println!("⚠️ {} is already gone.", self.name);
        }
    }
}
fn main() {
    let mut my_cat_1 = Cat::new("Gig", "Yellow", 10);
    
    my_cat_1.say_meow();
    println!("{} is a {}", my_cat_1.name, my_cat_1.age_description());
    my_cat_1.about_cat();

    my_cat_1.pass_away();
    my_cat_1.say_meow();
    my_cat_1.about_cat();
    
    my_cat_1.pass_away(); 

    println!("{my_cat_1:?}");
}