// 干支が表示できるように修正してください。
const ETO: [&str; 12] = [
    "さる",
    "とり",
    "いぬ",
    "い",
    "ネズミ",
    "うし",
    "とら",
    "う",
    "たつ",
    "み",
    "うま",
    "未",
];
fn eto(year: u32) {
    let org = 0;
    let diff = ((year - org) % 12) as usize;
    println!("{}", ETO[diff]);
}
fn main() {
    // 干支表示
    eto(2031);
}
