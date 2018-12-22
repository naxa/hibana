use clap::ArgMatches;
use failure::{Error, Fail};
use std::fs::{create_dir, File};
use std::path::Path;

#[derive(Debug, Fail)]
#[fail(display = "Project exists!")]
struct ProjectExists;

pub fn cmd_new(matches: &ArgMatches) -> Result<(), Error> {
    let name = matches.value_of("name").unwrap();
    let path = Path::new(name);

    if path.exists() {
        return Err(Error::from(ProjectExists));
    }

    create_project_dir(path)
}

fn create_project_dir(path: &Path) -> Result<(), Error> {
    create_dir(path)?;
    create_dir(path.join("contents"))?;
    create_dir(path.join("layouts"))?;
    create_dir(path.join("public"))?;
    create_dir(path.join("assets"))?;

    File::create(path.join("config.toml"))?;

    Ok(())
}
