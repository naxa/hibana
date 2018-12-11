pub mod new;

use clap::{App, AppSettings, Arg, SubCommand};

pub use self::new::cmd_new;

pub fn build_app() -> App<'static, 'static> {
    App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .setting(AppSettings::DeriveDisplayOrder)
        .subcommand(
            SubCommand::with_name("new")
                .about("create new project")
                .arg(
                    Arg::with_name("name")
                        .help("new project name")
                        .required(true),
                ),
        )
}
