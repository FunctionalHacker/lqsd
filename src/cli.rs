extern crate clap;
use clap::{Command, Arg, ArgAction};

pub fn app() -> Command {
    Command::new("LiQuid Screen Dim")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg_required_else_help(true)
        .arg(
            Arg::new("dim")
                .long("dim")
                .short('d')
                .action(ArgAction::SetTrue)
                .display_order(1)
                .help("Dims the screen to idle level set in configuration"),
        )
        .arg(
            Arg::new("resume")
                .long("resume")
                .short('r')
                .action(ArgAction::SetTrue)
                .display_order(2)
                .help("Sets the backlight to the value it was before dimming"),
        )
        .arg(
            Arg::new("copy-config")
                .long("copy-config")
                .action(ArgAction::SetTrue)
                .display_order(3)
                .help("Copies the default config file to $XDG_CONFIG_HOME/lqsd"),
        )
        .arg(
            Arg::new("config")
                .short('c')
                .long("config")
                .num_args(1)
                .number_of_values(1)
                .value_name("FILE")
                .help("Sets a custom config file"),
        )
}

#[test]
fn verify_app() {
    app().debug_assert();
}
