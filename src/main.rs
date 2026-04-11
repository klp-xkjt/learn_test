struct Message<'a> {
    p_name: &'a str,
    text: &'a str,
    r_name: &'a str
}
impl<'a> Message<'a> {
    fn output(&self) {
        println!("{}: {}. To {}", self.p_name, self.text, self.r_name)
    }
    // 规则3
    // fn who_sends(&self) -> &'a str {
    //     self.p_name
    // }
    fn who_sends(&self) -> &str {
        self.p_name
    }
    fn who_receives(&self) -> &str {
        self.r_name
    }
    fn what_contexts(&self) -> &str {
        self.text
    }
}

fn main() {
    let a: &str = "让我们看看如何通过传递拥有不同具体生命周期的引用来限制 longest 函数的使用。示例 10-22 是一个很直观的例子。";
    {
        let b: &str = "因为我们用相同的生命周期参数 'a 标注了返回的引用值，所以返回的引用值就能保证在 x 和 y 中较短的那个生命周期结束之前保持有效。";
        min_str(a, b);
    }

    let f1: &str = "Hello";
    {
        let f2: &str = "World";
        let fp: &str = first_par(f1, f2);
        println!("{fp}");
    }
    // println!("{fp}"); Avoid it.

    let f_m: Message = Message {
        p_name: "Alice",
        text: "adfsdfdfdfasdafsdf",
        r_name: "Bob"
    };
    println!("Publish: {}, context: {}, receiver: {}", f_m.who_sends(), f_m.what_contexts(), f_m.who_receives());
    f_m.output();

    let s_a: &'static str = "kdjfakjkjd";
    {
        println!("{s_a}");
    }
    println!("{s_a}");
}

// 规则1
// fn min_str<'a>(a: &'a str, b: &'a str) {
//     if a.is_empty() | b.is_empty() {
//         println!("有一个或都是空的");
//     }

//     if a.len() < b.len() {
//         println!("a 更少");
//     } else if b.len() < a.len() {
//         println!("b 更少");
//     } else {
//         println!("相等");
//     }
// }
fn min_str(a: &str, b: &str) {
    if a.is_empty() | b.is_empty() {
        println!("有一个或都是空的");
    }

    if a.len() < b.len() {
        println!("a 更少");
    } else if b.len() < a.len() {
        println!("b 更少");
    } else {
        println!("相等");
    }
}

// 不可不写生命周期
fn first_par<'a>(a: &'a str, _b: &'a str) -> &'a str {
    a
}