use clap::Parser;
use std::io;
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
    let test = testing();
    if let Err(_) = test {
        eprintln!("Failed to find steam path");
    } else {
        println!("{:?}", test)
    }

    io::stdin().read_line(&mut String::new()).unwrap();

    // println!("{}", test)

    // let args = Args::parse();

    // for _ in 0..args.count {
    //     println!("Hello {}!", args.name);
    // }
}
