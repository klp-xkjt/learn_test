#[derive(Debug)]
struct Triangle {
    side1: f64,
    side2: f64,
    side3: f64
}
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}
impl Triangle {
    fn area(&self) -> f64 {
        let p: f64 = (self.side1 + self.side2 + self.side3) / 2.0;
        let in_sqrt: f64 = p * (p - self.side1) * (p - self.side2) * (p - self.side3);
        let result: f64 = in_sqrt.sqrt();
        result
    }
    fn is_tri(&self) -> bool {
        self.side1 + self.side2 > self.side3 && self.side2 + self.side3 > self.side1 && self.side1 + self.side3 > self.side2
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    let tri1: Triangle = Triangle {
        side1: 3.0,
        side2: 4.0,
        side3: 5.0
    };
    if tri1.is_tri() {
        println!(
            "The area of the triangle is {}.",
            tri1.area()
        );
    } else {
        println!("Please check your triangle")
    }

    let tri2: Triangle = Triangle {
        side1: 9.0,
        side2: 4.0,
        side3: 3.0
    };
    if tri2.is_tri() {
        println!(
            "The area of the triangle is {}.",
            tri2.area()
        );
    } else {
        println!("Please check your triangle.");
        println!("Invalid triangle detected: {:?}", tri2);
    }
}