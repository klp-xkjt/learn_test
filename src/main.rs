#[derive(Debug)]
enum Infos {
    Name(String),
    Age(u32),
    Height(f32)
}

fn main() {
 // let v1: Vec<i32> = Vec::new();
 /* let v2: Vec<i32> = vec![0, 1, 2];
       v2.push(3); */

    let mut v2: Vec<i32> = vec![0, 1, 2];
    v2.push(3);

    let mut v3: Vec<i32> = vec![60, 11, 24];
    v3.push(21);
    v3.push(43);

    let vec3_first: &i32 = &v3[0];
    println!("First of v3: {}", vec3_first);

    let vec2_third: Option<&i32> = v2.get(2);
    match vec2_third {
        Some(a) => println!("Third of v2: {}", a),
        None => ()
    }

    for i in &v3 {
        println!("{}", i);
    }

    let info_a = vec![
        Infos::Name(String::from("Alice")),
        Infos::Age(23),
        Infos::Height(169.8)
    ];
    for i in &info_a {
        match i {
            Infos::Name(i) => println!("Name: {}", i),
            Infos::Age(i) => println!("Age: {}", i),
            Infos::Height(i) => println!("Height: {}", i)
        }
    }
    println!("{info_a:?}");
    
}