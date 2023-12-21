fn main() {
    for _ in (1..1000).rev() {
        println!("f = {}", a());
    }
}

fn a() -> i32 {
    3
}
