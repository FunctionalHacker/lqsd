# LiQuid Screen Dim
A simple utility which dims your screen. It saves the previous brightness too, so you can restore to the point before dimming.

This is useful if you are running a standalone screen locking setup like swayidle/swaylock. Check out [my configuration files](https://git.reekynet.com/ReekyMarko/dotfiles/src/branch/master/home/Scripts/swayidle.sh) for an example use case.

## Usage
```nosyntax
USAGE:
    lqsd [FLAGS] [OPTIONS]

FLAGS:
    -d, --dim            Dims the screen to idle level set in configuration
    -r, --resume         Sets the backlight to the value it was before dimming
        --copy-config    Copies the default config file to $XDG_CONFIG_HOME/lqsd
    -h, --help           Prints help information
    -V, --version        Prints version information

OPTIONS:
    --config <FILE>      Sets a custom config file
```
The configuration file resides at `~/.config/lqsd/config.toml`. To copy the default configuration file in place, you can use `--copy-config`.

In the configuration file you can set the following values:

| Key 			   | Explanation 																				  | Default 			|
| ---------------- | -------------------------------------------------------------------------------------------- | :-----------------: |
| resume_file 	   | The location where the previous brightness is saved                                          | /tmp/lqsd-resume    |
| idle_level       | The minimum brightness that will be dimmed to. Can be a value between 0-100                  | 0 					|
| dim_speed 	   | This sets the "sleep time" between each backlight command. It's in milliseconds              | 50 					|
| resume_speed     | Same as dim_speed, but for the resume operation `-r`              						  	  | 25  				|

## Installation

### Dependencies
The only external dependency is [light](https://github.com/haikarainen/light).

### Building
Clone this repository and run `cargo build --release` inside the project to compile a static binary.

### Arch Linux
So far, I have only packaged this for Arch Linux. Packages are in [AUR](https://aur.archlinux.org/packages/?K=lqsd).

To install it, use your favorite AUR helper, yay for example:
```nosyntax
yay -S lqsd
```
Select if you want to [1] build from source [2] install a precompiled binary:
```nosyntax
:: There are 2 providers available for lqsd:
:: Repository AUR
    1) lqsd 2) lqsd-bin

Enter a number (default=1):
```

## Releases
Release notes can be found in [GitHub](https://github.com/ReekyMarko/lqsd/releases) (Gitea does not support editing tags yet).

Prebuilt binaries, their checksums and signatures can also be found there.

## Why Rust?
I wanted start a small project to learn Rust. Before lqsd, I was using [this script](https://github.com/Bonnee/dotfiles/blob/wayland/scripts/bin/dim.sh) and figured, why not rewrite it in Rust.

# Mirrors
This repository lives at [My Git server](https://git.reekynet.com/ReekyMarko/lqsd), but it is also mirrored to [GitHub](https://github.com/ReekyMarko/lqsd) and [GitLab](https://gitlab.com/ReekyMarko/lqsd)
