// question13の続きです。構造体とimplを修正してperson.name()が表示できるように修正してください
#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
}
impl Person<'_> {
    fn name(&self) -> &str {
        self.name
    }
}

fn main() {
    let person = Person { name: "zakku" };
    println!("{:?}", person.name());
}
