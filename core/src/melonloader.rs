pub mod melonloader {
    use std::path::PathBuf;

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
}
