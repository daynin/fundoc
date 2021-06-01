use std::fs;

pub fn recreate_dir(path: &str) -> Result<(), std::io::Error> {
    fs::remove_dir_all(path).ok();
    fs::create_dir_all(path)
}
