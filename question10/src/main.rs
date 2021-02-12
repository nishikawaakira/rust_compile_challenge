// コンパイルが通るように修正してください
fn main() {
    let values = [1, 2, 5, 10, 15];
    let value = get(&values, 4);
    println!("{:?}", value);
}

fn get(&ary: &[i32], index: i64) -> Option<i64> {
    if ary.len() == 0 {
        None
    } else {
        ary.get(index)
    }
}
