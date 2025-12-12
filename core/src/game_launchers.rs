pub mod steam {
    use serde::Deserialize;
    use std::collections::HashMap;
    use std::path::PathBuf;
    use keyvalues_parser::Vdf;
    use keyvalues_serde::from_vdf;
    use std::borrow::Cow;

    #[cfg(target_os = "windows")]
    pub mod windows {
        use std::io;
        use winreg::RegKey;
        use winreg::enums::*;

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
        use std::{env, path::Path};

        pub fn is_installed() -> Option<String> {
            let home_dir = env::home_dir();
            let steam_dir = Path::new(".steam/steam");
            let full_dir = home_dir.unwrap().join(steam_dir);

            if full_dir.exists() {
                return Some(String::from(full_dir.to_str().unwrap()));
            } else {
                return None;
            }
        }
    }

    #[derive(Deserialize, Debug)]
    #[allow(dead_code)]
    struct LibraryFolders  {
        libraries: Vec<Library>,
    }

    #[derive(Deserialize, Debug)]
    #[allow(dead_code)]
    struct Library {
        path: PathBuf,
        apps: HashMap<u64, u64>,
    }

    pub fn read_library(steam_dir: &str) -> keyvalues_serde::Result<()> {



        const VDF_TEXT: &str = r##"
        "libraryfolders"
        {
            "0"
            {
                "path"		"C:\\Program Files (x86)\\Steam"
                "label"		""
                "contentid"		"1897266010619334199"
                "totalsize"		"0"
                "update_clean_bytes_tally"		"53289278"
                "time_last_update_verified"		"1752746886"
                "apps"
                {
                    "228980"		"277865478"
                }
            }
            "1"
            {
                "path"		"D:\\SteamLibrary"
                "label"		""
                "contentid"		"2631954653617166088"
                "totalsize"		"2000396742656"
                "update_clean_bytes_tally"		"2148485699"
                "time_last_update_verified"		"1764387472"
                "apps"
                {
                    "108600"		"7305916760"
                    "305620"		"4416174046"
                    "359550"		"69877094912"
                    "739630"		"36128289462"
                    "1066890"		"132047191318"
                    "1905180"		"477214184"
                    "2124490"		"36320370348"
                    "2592160"		"15406705489"
                    "2668510"		"10113266277"
                }
            }
        }
        "##;

        let mut vdf = Vdf::parse(VDF_TEXT)?;
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
        println!("Deserialized output:\n{:#?}", deserialized);

        Ok(())
    }
}
