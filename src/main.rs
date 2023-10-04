use dotenv::dotenv;
use std::env;

fn main() {
    dotenv().ok();
    let name = env::var("NAME").expect("NAME not set");
    println!("Hello, {}!", name);

}
