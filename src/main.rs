use core::encoding::Guldhjerte;
use std::env;

use web::app::App;
mod core;
mod web;
fn main() {
    let args: Vec<String> = env::args().collect();

    if let Some(input) = args.get(1) {
        match input.as_str() {
            "encode" => println!(
                "Guldhjerte code: {}",
                args.get(2).unwrap().to_guldhjerte().unwrap()
            ),
            "decode" => println!("Price: {}", args.get(2).unwrap().to_price()),
            _ => panic!("Unknown command"),
        }
    } else {
        // run web app
        yew::Renderer::<App>::new().render();
    }
}
