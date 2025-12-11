use clap::Parser;
use tld_mod_manager_core::game_launchers::*;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    name: String,

    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

fn main() {

    #[cfg(target_os = "windows")]
    if cfg!(target_os = "windows") {
        use std::io;
        let test = steam::windows::is_installed();

        if let Err(e) = test {
            eprintln!("Failed to find steam path. Error: {}", e);
        } else {
            println!("{:?}", test)
        }
    }

    #[cfg(target_os = "macos")]
    if cfg!(target_os = "macos") {
        steam::macos::is_installed();
    }

    #[cfg(target_os = "linux")]
    if cfg!(target_os = "linux") {
        let steam_dir = steam::linux::is_installed();
        let test2 = match steam_dir {
            Some(test2) => test2,
            None => return eprintln!("No steam directory found")
        };

        steam::read_library(&test2);
    }

    // io::stdin().read_line(&mut String::new()).unwrap();

    // println!("{}", test)

    // let args = Args::parse();

    // for _ in 0..args.count {
    //     println!("Hello {}!", args.name);
    // }
}
