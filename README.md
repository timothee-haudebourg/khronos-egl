# Rust bindings for EGL

[![](http://meritbadge.herokuapp.com/egl)](https://crates.io/crates/egl)

### Linking

When using OpenGL ES with rust-egl, it is necessary to place a dummy extern at the top of your
application which links libEGL first, then GLESv1/2. This is because libEGL provides symbols
required by GLESv1/2. Here's how to work around this:

```
#[link(name = "EGL")]
#[link(name = "GLESv2)]
extern {}
```