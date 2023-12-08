use rpi_stock_checker::application;
use std::process;

fn main() {
    ctrlc::set_handler(move || {
        println!("Closed with ctrl-c");
        process::exit(0);
    })
    .expect("Error setting Ctrl-C handler");

    if let Err(e) = application::run() {
        println!("Error: {}", e);
        process::exit(1);
    }
}
