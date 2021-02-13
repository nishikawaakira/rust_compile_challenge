// sayが実行できるように56行目を修正してください
struct Nishikawa<'a> {
    name: &'a str,
}

struct Zakku<'a> {
    name: &'a str,
}
trait Owner {
    fn name(&self) -> &str;
}
impl<'a> Owner for Nishikawa<'a> {
    fn name(&self) -> &str {
        self.name
    }
}

impl<'a> Owner for Zakku<'a> {
    fn name(&self) -> &str {
        self.name
    }
}

struct Standard {
    speed: i32,
}
struct Van {
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

impl Car for Van {
    fn speed(&self) -> i32 {
        self.speed
    }
}

struct CarOwner<T: Car, O: Owner> {
    car: T,
    owner: O,
}

trait CarOwnerSay<T: Car, O: Owner> {
    fn say(&self);
}

impl<T, O> CarOwnerSay<T, O> for CarOwner<T, O>
where
    T: Car,
    O: Owner,
{
    fn say(&self) {
        println!("{}, {}", self.car.speed(), self.owner.name());
    }
}

fn main() {
    let owner = Nishikawa { name: "nishikawa" };
    let car = Van { speed: 100 };
    let car_owner = CarOwner {
        car: car,
        owner: owner,
    };
    car_owner.say();
}
