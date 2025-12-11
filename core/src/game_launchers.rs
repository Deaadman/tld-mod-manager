pub mod steam {
    #[cfg(target_os = "windows")]
    pub mod windows {
        use std::io;
        use winreg::*;

        pub fn is_installed() -> io::Result<()> {

            let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);

            #[cfg(target_arch = "x86")]
            let key = hklm.open_subkey("SOFTWARE\\Valve\\Steam")?;
            #[cfg(target_arch = "x86_64")]
            let key = hklm.open_subkey("SOFTWARE\\WOW6432NODE\\Valve\\Steam")?;

            let steam_path: String = key.get_value("InstallPath")?;

            Ok(())
        }
    }

    #[cfg(target_os = "macos")]
    pub mod macos {
        pub fn is_installed() {
            println!("Hello From macOS!");
        }
    }

    #[cfg(target_os = "linux")]
    pub mod linux {
        pub fn is_installed() {
            println!("Hello From Linux!");
        }
    }
}
