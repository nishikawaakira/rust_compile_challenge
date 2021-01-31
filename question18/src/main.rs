// コンパイルが通るように修正してください。
struct Standard {
    speed: i32,
}

trait Car {
    fn speed(&self) -> i32;
}

impl Car for Standard {
    fn speed(&self) -> i32 {
        self.speed
    }
}

fn hoge(c: T) -> i32 {
    c.speed()
}

fn main() {
    println!("{}", hoge(Standard { speed: 30 }));
}
