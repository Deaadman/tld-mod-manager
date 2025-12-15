pub mod steam {
    use keyvalues_parser::Vdf;
    use keyvalues_serde::from_vdf;
    use serde::Deserialize;
    use std::borrow::Cow;
    use std::collections::HashMap;
    use std::fs;
    use std::path::Path;
    use std::path::PathBuf;

    #[cfg(target_os = "windows")]
    pub mod windows {
        use std::io;
        use std::path::PathBuf;
        use winreg::RegKey;
        use winreg::enums::*;

        pub fn is_installed() -> io::Result<PathBuf> {
            let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);

            #[cfg(target_arch = "x86_64")]
            let key = hklm.open_subkey("SOFTWARE\\WOW6432NODE\\Valve\\Steam")?;
            #[cfg(target_arch = "x86")]
            let key = hklm.open_subkey("SOFTWARE\\Valve\\Steam")?;

            let steam_path: String = key.get_value("InstallPath")?;

            Ok(PathBuf::from(steam_path))
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
        use std::env;
        use std::path::Path;
        use std::path::PathBuf;

        pub fn is_installed() -> PathBuf {
            let home_dir = env::home_dir();
            let steam_dir = Path::new(".steam/steam");
            let full_dir = home_dir.unwrap().join(steam_dir);

            return full_dir;
        }
    }

    #[derive(Deserialize, Debug)]
    #[allow(dead_code)]
    struct LibraryFolders {
        libraries: Vec<Library>,
    }

    #[derive(Deserialize, Debug)]
    #[allow(dead_code)]
    struct Library {
        path: PathBuf,
        apps: HashMap<u64, u64>,
    }

    pub fn read_library(steam_dir: PathBuf) -> keyvalues_serde::Result<()> {
        let library_folders_dir = Path::new("config/libraryfolders.vdf");
        let full_dir = steam_dir.join(library_folders_dir);
        let vdf_text = fs::read_to_string(full_dir)?;

        let mut vdf = Vdf::parse(&vdf_text)?;
        let obj = vdf.value.get_mut_obj().unwrap();

        // Switch all the entries with keys that are an index (0, 1, ...) to `"libraries"`
        let mut index = 0;
        while let Some(mut library) = obj.remove(index.to_string().as_str()) {
            obj.entry(Cow::from("libraries"))
                .or_insert(Vec::new())
                .push(library.pop().unwrap());

            index += 1;
        }

        let deserialized: LibraryFolders = from_vdf(vdf)?;

        for i in deserialized.libraries {
            let steamapps_dir = Path::new("steamapps");
            let full_dir = i.path.join(steamapps_dir);

            if !full_dir.exists() {
                continue;
            }

            println!("{:#?}", full_dir);
        }

        Ok(())
    }
}
