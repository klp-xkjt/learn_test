use std::fmt;


trait CatEatable {
    fn taste(&self) {}
}
#[derive(Debug)]
struct DriedFish {}
impl CatEatable for DriedFish {
    fn taste(&self) {
        println!("Cat eats the dried fish.");
    }
}

#[derive(Debug)]
struct Cat<'a, Eat> {
    name: &'a str,
    color: &'a str,
    eat: &'a Eat,
    mood: i32,
    energy: u32
}

impl<'a, Eat> Cat<'a, Eat>
where 
    Eat: CatEatable,
{
    fn new(name: &'a str, color: &'a str, eat: &'a Eat) -> Option<Self> {
        if name.is_empty() || color.is_empty() {
            None
        } else {
            Some(Self {
                name,
                color,
                eat,
                mood: 50,
                energy: 50
            })
        }
    }
    fn play_toy(&mut self) {
        if self.mood == 100 {
            println!("小猫心情很好，不需要玩玩具。🐱");
        } else if self.energy <=10 {
            println!("{} 没能量玩玩具！😿", self.name);
        } else {
            self.mood += 25;
            self.energy -= 10;
            println!("{} 开心地玩玩具！😺", self.name);
            if self.mood > 100 {
                self.mood = 100;
            }
        }
    }
    fn eat_food(&mut self, food: &'a Eat) {
        if self.energy == 100 {
            println!("小猫不饿，不需要吃东西。🐱")
        } else if self.mood <= 10 {
            println!("小猫心情不好，吃不下。😿")
        } else {
            println!("{}吃得很开心！😺", self.name);
            food.taste();
            self.energy += 10;
            self.mood += 10;
            if self.mood > 100 {
                self.mood = 100;
            }
        }
    }
    fn time_pass(&mut self) {
        println!("⏳ 一会儿过去了...🐱");
        
        if self.mood > 50 {
            self.mood -= 20;
        } else if self.mood > 20 {
            self.mood -= 10;
        } else {
            self.mood -= 5;
        }
    
        if self.mood < 0 {
            self.mood = 0;
        }
    }
}

impl<'a, Eat> fmt::Display for Cat<'a, Eat>
where
    Eat: CatEatable + fmt::Debug,
    {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "小猫名字：{} | 颜色：{} | 心情：{} | 能量：{} | 爱吃：{:?}", self.name, self.color, self.mood, self.energy, self.eat)
        }
    }

fn main() {
    let fish: DriedFish = DriedFish {};
    let mut my_cat: Cat<'_, DriedFish> = Cat::new("Miao", "Yellow", &fish).unwrap();

    println!("==== 小猫刚诞生 ====");
    println!("{my_cat}"); 

    println!("\n==== 小猫开始玩玩具 4 次 ====");
    for _ in 0..4 {
        my_cat.play_toy();
    }
    println!("{my_cat}");

    println!("\n==== 小猫继续玩玩具 4 次 ====");
    for _ in 0..4 {
        my_cat.play_toy();
    }
    println!("{my_cat}");

    println!("\n==== 小猫开始吃饭 5 次 ====");
    for _ in 0..5 {
        my_cat.eat_food(&fish);
    }
    println!("{my_cat}");

    println!("\n==== 最后玩一次玩具 ====");
    my_cat.play_toy();
    println!("{my_cat}");

    println!("\n==== 时间流逝 ====");
    for _ in 0..10 {
        my_cat.time_pass();
    }
    my_cat.eat_food(&fish);
    println!("{my_cat}");
    for _ in 0..4 {
        my_cat.play_toy();
    }
    println!("{my_cat}");
}