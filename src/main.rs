trait Shape {
    fn area(&self) -> f32;
    fn cf(&self) -> f32; // circumference
}

struct Triangle {
    a: f32,
    b: f32,
    c: f32,
}

impl Shape for Triangle {
    fn area(&self) -> f32 {
        let p = (self.a + self.b + self.c) / 2.0;
        let in_sqrt = p * (p - self.a) * (p - self.b) * (p - self.c);
        in_sqrt.sqrt()
    }

    fn cf(&self) -> f32 {
        self.a + self.b + self.c
    }
}

// 平行四边形：底、侧边、夹角（弧度）
struct Parallelogram {
    base: f32,
    side: f32,
    angle: f32,
}

impl Shape for Parallelogram {
    fn area(&self) -> f32 {
        self.base * self.side * self.angle.sin()
    }

    fn cf(&self) -> f32 {
        2.0 * (self.base + self.side)
    }
}

// 矩形：宽、高
struct Rectangle {
    width: f32,
    height: f32,
}

impl Shape for Rectangle {
    fn area(&self) -> f32 {
        self.width * self.height
    }

    fn cf(&self) -> f32 {
        2.0 * (self.width + self.height)
    }
}

// 菱形：两条对角线
struct Rhombus {
    d1: f32,
    d2: f32,
}

impl Shape for Rhombus {
    fn area(&self) -> f32 {
        0.5 * self.d1 * self.d2
    }

    fn cf(&self) -> f32 {
        let side = ((self.d1 / 2.0).powi(2) + (self.d2 / 2.0).powi(2)).sqrt();
        4.0 * side
    }
}

// 正方形：边长
struct Square {
    side: f32,
}

impl Shape for Square {
    fn area(&self) -> f32 {
        self.side * self.side
    }

    fn cf(&self) -> f32 {
        4.0 * self.side
    }
}

// ------------------------------
// 测试一下全部形状
// ------------------------------
fn main() {
    let tri: Triangle = Triangle {
        a: 3.0,
        b: 4.0,
        c: 5.0,
    };
    let para: Parallelogram = Parallelogram {
        base: 4.0,
        side: 3.0,
        angle: std::f32::consts::PI / 2.0, // 90度，其实就是矩形
    };
    let rect: Rectangle = Rectangle {
        width: 4.0,
        height: 3.0,
    };
    let rhom: Rhombus = Rhombus {
        d1: 6.0,
        d2: 8.0,
    };
    let square: Square = Square { side: 4.0 };

    show_all(tri);
    show_all(para);
    show_all(rect);
    show_all(rhom);
    show_all(square);
}

fn show_all<T: Shape>(shape: T) {
    println!("面积：{}", shape.area());
    println!("周长：{}", shape.cf());
}