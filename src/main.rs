mod core;

fn main() {
    println!("{}", core::encode::to_string("1234.25").unwrap());
}
