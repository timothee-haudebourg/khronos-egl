# Rust bindings for EGL

<table><tr>
  <td><a href="https://docs.rs/khronos-egl">Documentation</a></td>
  <td><a href="https://crates.io/crates/khronos-egl">Crate informations</a></td>
  <td><a href="https://github.com/timothee-haudebourg/khronos-egl">Repository</a></td>
</tr></table>

This crate provides a binding for the Khronos EGL 1.5 API.
It was originally a clone of the [egl](https://crates.io/crates/egl) crate,
which is left unmaintained.

## Usage

Here is a simple example showing how to use this library to create an EGL context.

```rust
extern crate khronos_egl as egl;

fn create_context(display: egl::Display) -> Result<egl::Context, egl::Error> {
	let attributes = [
		egl::RED_SIZE, 8,
		egl::GREEN_SIZE, 8,
		egl::BLUE_SIZE, 8,
		egl::NONE
	];

	let config = egl::choose_first_config(display, &attributes)?.expect("unable to find an appropriate ELG configuration");

	let context_attributes = [
		egl::CONTEXT_MAJOR_VERSION, 4,
		egl::CONTEXT_MINOR_VERSION, 0,
		egl::CONTEXT_OPENGL_PROFILE_MASK, egl::CONTEXT_OPENGL_CORE_PROFILE_BIT,
		egl::NONE
	];

	egl::create_context(display, config, None, &context_attributes)
}
```

The creation of a `Display` instance is not detailed here since it depends on your display server.
It is created using the `get_display` function with a pointer to the display server connection handle.
For instance, if you are using the [wayland-client](https://crates.io/crates/wayland-client) crate,
you can get this pointer using the `Display::get_display_ptr` method.

### NixOS

A `shell.nix` file is present for nix users to build the crate easily.
Just enter a new nix shell using the given configuration file,
and `cargo build` should work.

## Troubleshooting

### Linking with OpenGL ES

When using OpenGL ES with rust-egl, it is necessary to place a dummy extern at the top of your
application which links libEGL first, then GLESv1/2. This is because libEGL provides symbols
required by GLESv1/2. Here's how to work around this:

```
#[link(name = "EGL")]
#[link(name = "GLESv2)]
extern {}
```

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

If the original `egl` crate was licensed only under the Apache 2.0 license,
I believe I have made enough breaking changes so that no relevant code from the
original code remains and the rest can be relicensed.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
