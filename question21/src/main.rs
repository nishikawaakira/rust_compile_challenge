// コンパイルエラーを修正し、hashの中身が表示されるようにしてください
use std::collections::HashMap;
fn hoge(a: &mut HashMap<String, String>) {
    a.insert(String::from("Hoge"), String::from("Huga"));
}

fn main() {
    let mut hash: HashMap<String, String> = HashMap::new();
    hoge(& mut hash);
    println!("{:?}", hash);
}
