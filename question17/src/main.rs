// mainのコードが実行できるように構造体を修正してください
struct Hoge<T, U> {
    a: T,
    b: U,
}
fn main() {
    let x = Hoge { a: 1, b: "aiueo" };

    println!("{}, {}", x.a, x.b);
}
