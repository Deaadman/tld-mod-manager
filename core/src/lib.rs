use bitness::Bitness;
use std::env;
use std::io;
use winreg::RegKey;
use winreg::enums::*;

pub fn testing() -> io::Result<()> {
    let current_os = env::consts::OS;
    println!("The operating system is: {}", current_os);

    if current_os == "windows" {
        let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
        let key: RegKey;
        let bitness: Bitness = bitness::os_bitness().unwrap();

        println!("The bitness is: {:?}", bitness);

        if bitness == Bitness::X86_64 {
            key = hklm.open_subkey("SOFTWARE\\WOW6432NODE\\Valve\\Steam")?;
        } else {
            key = hklm.open_subkey("SOFTWARE\\Valve\\Steam")?;
        }

        println!("Steam is installed");

        let steam_path: String = key.get_value("InstallPath")?;

        println!("Steam install path is located at: {}", steam_path);
    }

    Ok(())
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
