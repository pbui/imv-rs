// imv-rs: Interactive Move Utility

use std::env;
use std::fs;
use std::fs::read_to_string;
use std::io::Write;
use std::process::Command;

use tempfile::NamedTempFile;

// Functions

fn save_paths(paths: &[String]) -> Result<String, std::io::Error> {
    let mut temp_file = NamedTempFile::new()?;

    for path in paths {
        writeln!(temp_file, "{}", path)?;
    }

    let (_, path) = temp_file.keep()?;

    Ok(path.to_str().unwrap().to_string())
}

fn edit_paths(temp_path: &String) -> Result<(), std::io::Error> {
    let editor = env::var("EDITOR").unwrap_or("vim".into());

    let status = Command::new(editor).arg(temp_path).spawn()?.wait()?;

    if !status.success() {
        panic!("Editor failure")
    }

    Ok(())
}

fn move_paths(old_paths: &[String], temp_path: &String) -> Result<(), std::io::Error> {
    let new_paths = read_to_string(temp_path).unwrap();

    for (old_path, new_path) in old_paths.iter().zip(new_paths.lines()) {
        println!("{} -> {}", old_path, new_path);
        fs::rename(old_path, new_path)?
    }

    Ok(())
}

// Main execution

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let old_paths = &args[1..];

    let temp_path = save_paths(old_paths).expect("Unable to save paths");
    edit_paths(&temp_path).expect("Unable to edit paths");
    move_paths(old_paths, &temp_path).expect("Unable to move paths");

    // TODO: Make sure this runs on exit
    fs::remove_file(temp_path)?;
    Ok(())
}
