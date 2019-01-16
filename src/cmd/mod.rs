pub mod build;
pub mod new;
pub mod serve;

use clap::{App, AppSettings, Arg, SubCommand};

pub use self::build::cmd_build;
pub use self::new::cmd_new;
pub use self::serve::cmd_serve;

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
        .subcommand(SubCommand::with_name("build").about("build"))
        .subcommand(SubCommand::with_name("serve")
            .about("serve local server"))
}
