# sl-rust

Because the train needs to run on.

## Building

* Install Rust and Cargo: [http://doc.crates.io/](http://doc.crates.io/)
* Install ncurses through your systems package manager
* Run `cargo build`. The resulting binary is `target/sl`.

## Usage

```sh
$ sl (-lcaf)
```

Relax, take a deep breath.

## Compatibility with sl.c

* Unconditionally traps SIGINT
* All train types implemented (D51, C51, Logo)

## Open issues

* Smoke missing
* Accident animations (for `-a` option) missing
* Fly option (`-f`) missing
* The renderer is not my best piece of code. I hate ncurses.
* The exit condition is still a bit naive. I blame it on the coffe and the late nights.

## Details for Beginners

This is implemented as a library and a binary. The library provides the trains, along with all important information (does it have a tender, wagons, and if yes, how many?), the binary does the rendering. The library is independent of any external libraries, the binary needs `ncurses` and `libc` (you hopefully have the latter installed ;)).

## License

ChooChoo0, see LICENSE for details.
