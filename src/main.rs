use std::env;
mod core;

fn main() {
    let args: Vec<String> = env::args().collect();

    if let Some(input) = args.get(1) {
        match core::encode::to_string(input) {
            Ok(res) => println!("Guldhjerte code: {}", res),
            Err(e) => panic!("Error: {}", e),
        }
    } else {
        panic!("No input provided");
    }
}
