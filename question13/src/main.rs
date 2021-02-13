// personが出力できるように構造体だけ修正してください
#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
}

fn main() {
    let person = Person { name: "zakku" };
    println!("{:?}", person);
}
