use std::io;
use winreg::enums::*;
use winreg::RegKey;

fn main() {
    println!("Hello, world!");
    
    if cfg!(windows) {
        println!("This is Windows");

        let bitness = bitness::os_bitness().unwrap();
        println!("{:?}", bitness);

        match bitness {
            bitness::Bitness::X86_64 => println!("This is x64"),
            bitness::Bitness::X86_32 => println!("This is x86"),
            bitness::Bitness::Unknown => println!("Unknown Bitness")
        }
        
    } else if cfg!(target_os = "macos") {
        println!("This is macOS");
    } else {
        println!("This is Linux");
    }

    testing();
}

fn testing() -> io::Result<()> {
    println!("Reading some system info...");

    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let key = hklm.open_subkey("SOFTWARE\\WOW6432NODE\\Valve\\Steam")?;
    // let key = hklm.open_subkey("SOFTWARE\\Valve\\Steam");
    let value: String = key.get_value("InstallPath")?;

    println!("{}", value);
    Ok(())
}