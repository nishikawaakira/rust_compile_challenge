// コンパイルが通るように修正してください
struct Point {
    x: i32,
    y: i32,
}

fn addpoint(mut p1: Point, p2: Point) -> Point {
    p1.x += p2.x;
    p1.y += p2.y;

    p1
}

fn main() {
    let p1: Point = Point { x: 1, y: 3 };
    let p2: Point = Point { x: 2, y: 4 };

    let p3 = addpoint(p1, p2);
    println!("{} {}", p3.x, p3.y);
}
