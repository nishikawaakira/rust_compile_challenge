// 43行目のコンパイルエラーを修正してください
struct Latlng<T> {
    lat: T,
    lng: T,
}
struct Hoge {
    a: i32,
}

trait Printable {
    fn print(&self);
}
impl<T> Printable for Latlng<T>
where
    T: std::fmt::Display,
{
    fn print(self: &Latlng<T>) {
        println!("lat:{} lng:{}", self.lat, self.lng);
    }
}

impl std::fmt::Display for Hoge {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.a)
    }
}

fn main() {
    let hoge = Hoge { a: 100 };
    let ll1: Latlng<f64> = Latlng {
        lat: 35.6804,
        lng: 139.7690,
    };
    let ll2: Latlng<i32> = Latlng {
        lat: 12187059,
        lng: 47071786,
    };
    let ll3: Latlng<&str> = Latlng {
        lat: "35.6804",
        lng: "47071786",
    };
    let ll4: Latlng<&Hoge> = Latlng {
        lat: &hoge,
        lng: &hoge,
    };
    ll1.print();
    ll2.print();
    ll3.print();
    ll4.print();
}
