use std::env;

use clap::Parser;
use tld_mod_manager_core::testing;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    name: String,

    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

fn main() {
    let current_os = env::consts::OS;
    match current_os {
        "windows" => println!("User is on Windows!"),
        "macos" => println!("User is on macOS!"),
        "linux" => println!("User is on Linux!"),
        _ => println!("Unknown operating system"),
    }

    let test = testing();
    if let Err(_) = test {
        eprintln!("Failed to find steam path");
    } else {
        println!("{:?}", test)
    }

    // println!("{}", test)

    // let args = Args::parse();

    // for _ in 0..args.count {
    //     println!("Hello {}!", args.name);
    // }
}
