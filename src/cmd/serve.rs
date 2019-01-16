extern crate rocket;
extern crate rocket_contrib;

use failure::Error;
use rocket_contrib::serve::StaticFiles;

pub fn cmd_serve() -> Result<(), Error> {
    rocket().launch();

    Ok(())
}

fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/", StaticFiles::from("public"))
}
