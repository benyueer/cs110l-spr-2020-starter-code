use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Too few arguments.");
        process::exit(1);
    }
    let filename = &args[1];
    // Your code here :)
}

export https_proxy=http://192.168.3.140:7890 http_proxy=http://192.168.3.140:7890 all_proxy=socks5://192.168.3.140:7890