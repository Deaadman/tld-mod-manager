use std::io;
use winreg::RegKey;
use winreg::enums::*;

pub fn testing() -> Result<RegKey, io::Error> {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let key = hklm.open_subkey("SOFTWARE\\WOW6432NODE\\Valve\\Steam")?;
    Ok(key)

    // let key = hklm.open_subkey("SOFTWARE\\Valve\\Steam");
    // let value = testing2.get_value("InstallPath");
    // let testing = match value {
    //     Ok(test) => test,
    //     Err(e) => return Err(e),
    // };
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
