// コンパイルエラーを修正してください
fn main() {
    let mut a = vec![3, 2, 1, 4, 5, 2];
    a.push(3);
    let mut b = a;
    println!("{:?}", b.pop());
}
