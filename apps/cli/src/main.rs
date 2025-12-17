use clap::Parser;
use std::path::PathBuf;
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
    let steam_dir: PathBuf;

    #[cfg(target_os = "windows")]
    {
        steam_dir = steam::windows::is_installed().unwrap();
    }

    #[cfg(target_os = "macos")]
    {
        steam::macos::is_installed();
    }

    #[cfg(target_os = "linux")]
    {
        steam_dir = steam::linux::is_installed();
    }

    let libraries = steam::read_libraries(steam_dir);
    let _ = steam::read_games(libraries.unwrap());

    // io::stdin().read_line(&mut String::new()).unwrap();

    // println!("{}", test)

    // let args = Args::parse();

    // for _ in 0..args.count {
    //     println!("Hello {}!", args.name);
    // }
}
