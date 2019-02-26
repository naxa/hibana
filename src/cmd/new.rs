use clap::ArgMatches;
use failure::{Error, Fail};
use std::fs::{create_dir, File};
use std::io::{Write};
use std::path::Path;

#[derive(Debug, Fail)]
#[fail(display = "Project directory already exists!")]
struct ProjectExists;

const INDEX_SAMPLE: &str = "---
title: Hibana
---

Hello from [**Hibana**](https://github.com/student-kyushu/hibana)!

This is the sample page, you can create your own world. Enjoy!
";

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
    let mut index = File::create(path.join("contents").join("index.md"))?;
    write!(index, "{}", INDEX_SAMPLE)?;
    index.flush()?;


    Ok(())
}
