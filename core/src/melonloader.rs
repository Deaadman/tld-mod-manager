pub mod melonloader {
    use std::path::PathBuf;

    pub mod version {
        pub fn get_version() {}
    }

    pub fn installed(game_dir: PathBuf) -> bool {
        let melonloader_dir = game_dir.join("MelonLoader");

        if melonloader_dir.exists() {
            return true;
        }

        return false;
    }
}
