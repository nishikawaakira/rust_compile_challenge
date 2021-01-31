// mainのコードが実行できるように構造体を修正してください
struct Hoge<T, T> {
    a: T,
    b: T,
}
fn main() {
    let x = Hoge { a: 1, b: "aiueo" };

    println!("{}, {}", x.a, x.b);
}
