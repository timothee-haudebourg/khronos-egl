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

use egl::{EGLDisplay, EGLContext};

fn create_context(display: EGLDisplay) -> EGLContext {
  let attributes = [
    egl::EGL_RED_SIZE, 8,
    egl::EGL_GREEN_SIZE, 8,
    egl::EGL_BLUE_SIZE, 8,
    egl::EGL_NONE
  ];

  let config = egl::choose_config(display, &attributes, 1).expect("unable to find an appropriate ELG configuration");

  let context_attributes = [
    egl::EGL_CONTEXT_MAJOR_VERSION, 4,
    egl::EGL_CONTEXT_MINOR_VERSION, 0,
    egl::EGL_CONTEXT_OPENGL_PROFILE_MASK, egl::EGL_CONTEXT_OPENGL_CORE_PROFILE_BIT,
    egl::EGL_NONE
  ];

  egl::create_context(display, config, egl::EGL_NO_CONTEXT, &context_attributes).expect("unable to create an EGL context")
}
```

The creation of an `EGLDisplay` instance is not detailed here since it depends on your display server.
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
