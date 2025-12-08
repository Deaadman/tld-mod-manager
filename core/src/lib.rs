use std::io;
use winreg::enums::*;
use winreg::RegKey;

pub fn testing() -> io::Result<()> {
    println!("Reading some system info...");

    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let key = hklm.open_subkey("SOFTWARE\\WOW6432NODE\\Valve\\Steam")?;
    // let key = hklm.open_subkey("SOFTWARE\\Valve\\Steam");
    let value: String = key.get_value("InstallPath")?;

    println!("{}", value);
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
