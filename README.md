# rust-conways-game-of-life

This is a recreation of Conway's Game of Life utilizing the [amethyst](https://amethyst.rs/doc) game engine in rust to try out making something with the game engine and continue some experimenting with rust.

As such this was really just planned as a one off, and I've really only tried this on Fedora 33. Use the code as an example at your own risk.

## Configuration

This project contains two configuration files:
* display.ron
  * Used to determine how the amethyst window will open. Size of window, what the title is, etc
* config.ron
  * Used to set values inside the game itself. Board height and width along with the speed at which the game will run. Refer to the ron file for additional information in the comments

Feel free to make adjustments to the config files to try out different variations for your setup.

## Usage

```shell
cargo run [--release]

// On Fedora 32+ and wayland window manager machines
// https://github.com/amethyst/amethyst/issues/1846
// Believe this is because of a upstream rendy dependency
// in amethyst as of 2020/08/08
WINIT_UNIX_BACKEND=x11 cargo run [--release]
```

## Contributing

This project was a one off for me, but if there are updates to rust or things that would make this repository a better example of utilizing the amethyst game engine submit a PR and I'll try and get that out there.

Overall if someone wants to just mess around with something have at forking it and hacking around till your hearts desire. Some ideas for different projects that could be done:
* Actually write some tests
* Move the configuration/cell seed to a user input at the start
* Benchmark the cell system to see where improvements can be made
* Provide a easy way to provide some seed configuration. Maybe see other implementations to see how they are loading in seeds?

## License

[GPL-3.0-or-later](LICENSE.md)
