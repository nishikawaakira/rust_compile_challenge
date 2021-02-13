// scoreに値を加算できるようにしてください。
fn main() {
    let mut scores = vec![100, 92, 84, 75, 98, 81];
    for score in scores.iter_mut() {
        *score += 10;
    }
    println!("{:?}", scores);
}
