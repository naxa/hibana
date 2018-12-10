use clap::ArgMatches;
use std::fs::{create_dir, File};
use std::path::Path;

pub fn cmd_new(matches: &ArgMatches) -> Result<(), String> {
    let name = matches.value_of("name").unwrap();
    let path = Path::new(name);

    if path.exists() {
        return Err("project exists!".to_string());
    }

    match create_project_dir(path) {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string()),
    }
}

fn create_project_dir(path: &Path) -> Result<(), std::io::Error> {
    create_dir(path)?;
    create_dir(path.join("contents"))?;
    create_dir(path.join("layouts"))?;
    create_dir(path.join("public"))?;
    create_dir(path.join("assets"))?;

    File::create(path.join("config.toml"))?;

    Ok(())
}
