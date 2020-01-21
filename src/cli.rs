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
                .display_order(1)
                .help("Dims the screen to idle level set in configuration"),
        )
        .arg(
            Arg::with_name("resume")
                .long("resume")
                .short("r")
                .takes_value(false)
                .display_order(2)
                .help("Sets the backlight to the value it was before dimming"),
        )
        .arg(
            Arg::with_name("copy-config")
                .long("copy-config")
                .takes_value(false)
                .display_order(3)
                .help("Copies the default config file to $XDG_CONFIG_HOME/lqsd"),
        )
        .arg(
            Arg::with_name("config")
                .long("config")
                .takes_value(true)
                .number_of_values(1)
                .value_name("FILE")
                .help("Sets a custom config file"),
        )
        .get_matches()
}
