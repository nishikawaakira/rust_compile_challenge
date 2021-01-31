// matchを使用したままコンパイルが通るように修正してください。
fn main() {
    println!("8 * 8 = {}", square(Some(8)).unwrap_or(0));
}

fn square(x: Option<i64>) -> Option<i64> {
    match x {
        Ok(i) => Ok(i * i),
    }
}
