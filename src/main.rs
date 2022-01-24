mod load_config;
mod util;

use util::constant::GLOBAL_CONFIG;

fn main() {
    let config = &GLOBAL_CONFIG;
    println!("{:?}", config.mysql);
}