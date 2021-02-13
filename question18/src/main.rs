// コンパイルが通るように修正してください。
struct Standard {
    speed: i32,
}

struct SportsCar {
    speed: i32,
    turbo: i32,
}

trait Car {
    fn speed(&self) -> i32;
}

impl Car for Standard {
    fn speed(&self) -> i32 {
        self.speed
    }
}

impl Car for SportsCar {
    fn speed(&self) -> i32 {
        self.speed
    }
}

fn hoge<T>(c: T) -> i32
where
    T: Car,
{
    c.speed()
}

fn main() {
    println!("{}", hoge(Standard { speed: 30 }));
    println!("{}", hoge(SportsCar { speed: 30, turbo: 40 }));
}
