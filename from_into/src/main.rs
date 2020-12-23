#[derive(Debug)]
struct Int {
    val: i32
}

impl From<i32> for Int {
    fn from(from: i32) -> Int {
        Int { val: from }
    }
}

fn main() {
    let v = Int::from(5);
    println!("{:?}", v);

    let v: Int = 3.into();
    println!("{:?}", v);
}
