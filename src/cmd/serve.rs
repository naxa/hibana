extern crate rocket;
extern crate rocket_contrib;

use crate::build::cmd_build;
use failure::Error;
use rocket_contrib::serve::StaticFiles;

pub fn cmd_serve() -> Result<(), Error> {
    cmd_build()?;

    rocket::ignite().mount("/", StaticFiles::from("public"))
        .launch();

    Ok(())
}
