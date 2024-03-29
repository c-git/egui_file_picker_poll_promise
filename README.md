## egui file browser (using poll-promise)

This example is based on the [original egui file browser](https://github.com/woelper/egui_pick_file) by [woelper](https://github.com/woelper) but modified to use poll-promise instead.
It makes the app a bit more complex but closer to what I personally prefer to use as it prevents the user from clicking load or save while the picker is still open.

For an example of poll_promise in use in a more full featured application see this [log viewer](https://github.com/c-git/log-viewer) (full featured in terms of being able to report errors and that kind of thing).

Description from original:

> Example app showing how to pick a file on both web and desktop.

The web application can be accessed here:

https://c-git.github.io/egui_file_picker_poll_promise/

The web application for the original can be found at:

https://woelper.github.io/egui_pick_file/

For native:

On linux install dependencies of [rfd](https://docs.rs/rfd/latest/rfd/index.html) as applicable.
Copied from https://docs.rs/rfd/latest/rfd/#linux--bsd-backends

> GTK backend is used with the `gtk3` Cargo feature which is enabled by default. The GTK3
> backend requires the C library and development headers to be installed to build RFD. The package
> names on various distributions are:
> | Distribution | Installation Command |
> | --------------- | ------------ |
> | Fedora | dnf install gtk3-devel |
> | Arch | pacman -S gtk3 |
> | Debian & Ubuntu | apt install libgtk-3-dev |

`cargo run`

For web:

`rustup target add wasm32-unknown-unknown`

`cargo install --locked trunk`

`trunk serve --open`

## License

All code in this repository is dual-licensed under either:

- Apache License, Version 2.0
- MIT license

at your option.
This means you can select the license you prefer!
This dual-licensing approach is the de-facto standard in the Rust ecosystem and there are very good reasons to include both as noted in
this [issue](https://github.com/bevyengine/bevy/issues/2373) on [Bevy](https://bevyengine.org)'s repo.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
