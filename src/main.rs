pub mod pkl;

use crate::pkl::app_config::load_from_path;

fn main() {
    println!("Hello, world!");
    let config = load_from_path("pkl/local/local_config.pkl".to_string());

    println!("Running on {:?}:{:?}", config.as_ref().unwrap().host, config.as_ref().unwrap().port);
}
