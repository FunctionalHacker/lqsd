# LiQuid Screen Dim
A simple utility which dims your screen. It saves the previous brightness too, so you can restore to the point before dimming.

## Why Rust?
I wanted start a small project to learn Rust. Before this, I was using [this script]("https://github.com/Bonnee/dotfiles/blob/wayland/scripts/bin/dim.sh") and figured, why not rewrite it in Rust.

## Usage
```
USAGE:
    lqsd [FLAGS] [OPTIONS]

FLAGS:
    -d, --dim        Dims the screen to idle level set in configuration
    -h, --help       Prints help information
    -r, --resume     Sets the backlight to the value it was before dimming
    -V, --version    Prints version information

OPTIONS:
    -c, --config <config>    Sets a custom config file
```
The configuration file resides at `$XDG_CONFIG_HOME/lqsd/config.toml`. There you can set these values:

`resume_file_path`: the location where the previous brightness is saved (default: `/tmp/lqsd-resume`)

`idle_level`: the minimum brightness that will be dimmed to. Can be a value between 0-100 (default 0)

`dim_speed`: this sets the "sleep time" between each backlight command. It's in milliseconds (default 50)

`resume_speed`: same as the last one, only for the resume operation `-r` (default 25)
