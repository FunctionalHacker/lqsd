# LiQuid Screen Dim
A simple utility which dims your screen. It saves the previous brightness too, so you can restore to the point before dimming.

This is useful if you are running a standalone screen locking setup like swayidle/swaylock. Check out [my configuration files](https://git.reekynet.com/ReekyMarko/dotfiles/src/branch/master/home/Scripts/swayidle.sh) for an example use case.

## Usage
```nosyntax
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
The configuration file resides at `~/.config/lqsd/config.toml`. There you can set these values:

| Key 			   | Explanation 																				  | Default 			|
| ---------------- | -------------------------------------------------------------------------------------------- | :-----------------: |
| resume_file 	   | The location where the previous brightness is saved                                          | /tmp/lqsd-resume    |
| idle_level       | The minimum brightness that will be dimmed to. Can be a value between 0-100                  | 0 					|
| dim_speed 	   | This sets the "sleep time" between each backlight command. It's in milliseconds              | 50 					|
| resume_speed     | Same as dim_speed, but for the resume operation `-r`              						  	  | 25  				|

## Installation

### Dependencies
The only external dependency is [light](https://github.com/haikarainen/light)

### Building
Clone this repository and run `cargo build --release` inside the project to compile a static binary.

Prebuilt binaries, their checksums and signatures live in [GitLab](https://gitlab.com/ReekyMarko/lqsd/releases)

### Arch Linux
So far, I have only packaged this for Arch Linux. Packages are in [AUR](https://aur.archlinux.org/packages/?K=lqsd)

To install it, use your favorite AUR helper, yay for example:
```nosyntax
yay -S lqsd
```

Or if you don't want to build it from source, a binary version is also available:
```nosyntax
yay -S lqsd-bin
```

## Why Rust?
I wanted start a small project to learn Rust. Before lqsd, I was using [this script](https://github.com/Bonnee/dotfiles/blob/wayland/scripts/bin/dim.sh) and figured, why not rewrite it in Rust.

# Mirrors
This repository lives at [ReekyNET Git](https://git.reekynet.com/ReekyMarko/lqsd), but it is also mirrored to [GitLab](https://gitlab.com/ReekyMarko/lqsd) and [GitHub](https://github.com/ReekyMarko/lqsd)
