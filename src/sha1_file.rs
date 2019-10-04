use std::fs;
use std::path::{Component, PathBuf};

// todo: implement adjusting shared permissions. See sha1-file.c:278-330
pub fn safe_create_leading_directories(path: &PathBuf) -> Result<(), std::io::Error> {
    let mut components: Vec<Component> = path.components().collect();
    components.pop();
    let dirname: PathBuf = components.iter().collect();
    fs::create_dir_all(dirname)
}
