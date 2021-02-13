// hello worldが出力されるように修正してください
fn helloworld() -> &'static str {
    "hello world"
}

fn main() {
    println!("{}", helloworld());
}
