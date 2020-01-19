extern crate clap;
use clap::{App, AppSettings, Arg};

pub fn get_args() -> clap::ArgMatches<'static> {
    App::new("LiQuid Screen Dim")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .setting(AppSettings::ArgRequiredElseHelp)
        .arg(
            Arg::with_name("dim")
                .long("dim")
                .short("d")
                .takes_value(false)
                .help("Dims the screen to idle level set in configuration"),
        )
        .arg(
            Arg::with_name("resume")
                .long("resume")
                .short("r")
                .takes_value(false)
                .help("Sets the backlight to the value it was before dimming"),
        )
        .arg(
            Arg::with_name("config")
                .long("config")
                .short("c")
                .takes_value(true)
                .help("Sets a custom config file"),
        )
        .get_matches()
}
