pub mod melonloader {
    use std::path::PathBuf;
    use glob::glob;

    #[cfg(target_os = "windows")]
    pub fn get_version(game_dir: PathBuf) -> VersionInfo {
        use win32_version_info::VersionInfo;

        let melonloader_dir = game_dir.join("MelonLoader");
        let melonloader_assembly_dir = game_dir.join("MelonLoader.dll");

        let info = VersionInfo::from_file(melonloader_assembly_dir).expect("Failed to retrieve version information");
        return info.file_version;
    }

    pub fn installed(game_dir: PathBuf) -> bool {
        let melonloader_dir = game_dir.join("MelonLoader");

        if melonloader_dir.exists() {
            return true;
        }

        return false;
    }

    pub fn read_installed_mods(game_dir: PathBuf) {
        let mod_dir = game_dir.join("Mods");

        let dir_display = mod_dir.display();
        let dir_pattern = format!("{dir_display}*.dll");

        for i in glob(&dir_pattern).unwrap() {
            println!("Name: {}", i.unwrap().display())
        }
    }
}
