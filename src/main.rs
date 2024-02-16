use std::{path::PathBuf, env};

fn main() {
    println!("Conventional Commits CLI");
    let current_dir = get_current_working_dir().expect("Error getting current directory");

    println!("Current directory: {:?}", current_dir);
}

// Get current directory
fn get_current_working_dir() -> std::io::Result<PathBuf> {
    env::current_dir()
}
