// 7行目を直してコンパイルエラーを解消してください。
#[derive(Debug)]
struct Person {
    name: String,
}

fn name_longer<'a>(a: &'a Person, b: &'a Person) -> &'a Person {
    if a.name.len() > b.name.len() {
        a
    } else {
        b
    }
}

fn main() {
    let a = Person {
        name: String::from("zakku"),
    };
    let b = Person {
        name: String::from("nishikawa"),
    };

    println!("{:?}", name_longer(&a, &b));
}
