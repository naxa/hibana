extern crate rocket;
extern crate rocket_contrib;

use rocket_contrib::serve::StaticFiles;

pub fn cmd_serve() {
    rocket().launch();
}

fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/", StaticFiles::from("public"))
}
