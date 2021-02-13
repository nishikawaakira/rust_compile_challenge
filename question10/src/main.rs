// コンパイルが通るように修正してください
fn main() {
    let values = [1, 2, 5, 10, 15];
    let value = get(&values, 4);
    println!("{:?}", value);
}

fn get(&ary: &[i64; 5], index: usize) -> Option<i64> {
    if ary.len() == 0 {
        None
    } else {
        Some(ary[index])
    }
}
