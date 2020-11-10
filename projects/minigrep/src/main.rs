use minigrep::Config;
use std::env;
use std::process;

fn main() {
    // cargo run How  pome.txt > output.txt
    // let args: Vec<String> = env::args().collect();

    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("args error :{}", err);
        process::exit(1);
    });
    if let Err(e) = minigrep::run(config) {
        eprintln!("error {}", e);
        process::exit(1);
    }
}
