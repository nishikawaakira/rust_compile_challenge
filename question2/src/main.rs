// matchを使用したままコンパイルが通るように修正してください。
fn main() {
    println!("8 * 8 = {}", square(Some(8)).unwrap_or(0));
}

fn square(x: Option<i64>) -> Option<i64> {
    match x {
        Some(i) => Some(i * i),
        _ => unreachable!(),
    }
}

// オプショナル型を受けて
// 値をオプショナル型で返せるように変更
// エラー型が来た場合等の処理をunreachable!()で実装
