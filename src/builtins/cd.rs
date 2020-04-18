use std::env;
use std::io;
use std::path::Path;

pub fn cd(path: &str) -> io::Result<()> {
    env::set_current_dir(Path::new(&path))
}
