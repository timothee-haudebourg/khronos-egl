//! This crate provides a binding for the Khronos EGL 1.5 API.
//! It was originally a fork of the [egl](https://crates.io/crates/egl) crate,
//! which is left unmaintained.
//! 
//! ## Usage
//! 
//! You can access the EGL API using an [`Instance`]
//! object defined by either statically linking with `libEGL.so` at compile time,
//! or dynamically loading the EGL library at runtime.
//! 
//! ### Static linking
//! 
//! You must enable static linking using the `static` feature in your `Cargo.toml`:
//! ```toml
//! khronos-egl = { version = ..., features = ["static"] }
//! ```
//! 
//! This will add a dependency to the [`pkg-config`](https://crates.io/crates/pkg-config) crate,
//! necessary to find the EGL library at compile time.
//! Here is a simple example showing how to use this library to create an EGL context when static linking is enabled.
//! 
//! ```rust
//! extern crate khronos_egl as egl;
//! 
//! fn main() -> Result<(), egl::Error> {
//! 	// Create an EGL API instance.
//! 	// The `egl::Static` API implementation is only available when the `static` feature is enabled.
//! 	let egl = egl::Instance::new(egl::Static);
//! 
//! 	let wayland_display = wayland_client::Display::connect_to_env().expect("unable to connect to the wayland server");
//! 	let display = egl.get_display(wayland_display.get_display_ptr() as *mut std::ffi::c_void).unwrap();
//! 	egl.initialize(display)?;
//! 
//! 	let attributes = [
//! 		egl::RED_SIZE, 8,
//! 		egl::GREEN_SIZE, 8,
//! 		egl::BLUE_SIZE, 8,
//! 		egl::NONE
//! 	];
//! 
//! 	let config = egl.choose_first_config(display, &attributes)?.expect("unable to find an appropriate ELG configuration");
//! 
//! 	let context_attributes = [
//! 		egl::CONTEXT_MAJOR_VERSION, 4,
//! 		egl::CONTEXT_MINOR_VERSION, 0,
//! 		egl::CONTEXT_OPENGL_PROFILE_MASK, egl::CONTEXT_OPENGL_CORE_PROFILE_BIT,
//! 		egl::NONE
//! 	];
//! 
//! 	egl.create_context(display, config, None, &context_attributes);
//! 
//! 	Ok(())
//! }
//! ```
//! 
//! The creation of a `Display` instance is not detailed here since it depends on your display server.
//! It is created using the `get_display` function with a pointer to the display server connection handle.
//! For instance, if you are using the [wayland-client](https://crates.io/crates/wayland-client) crate,
//! you can get this pointer using the `Display::get_display_ptr` method.
//! 
//! #### Static API Instance
//! 
//! It may be bothering in some applications to pass the `Instance` to every fonction that needs to call the EGL API.
//! One workaround would be to define a static `Instance`,
//! which should be possible to define at compile time using static linking.
//! However this is not yet supported by the stable `rustc` compiler.
//! With the nightly compiler,
//! you can combine the `nightly` and `static` features so that this crate
//! can provide a static `Instance`, called `API` that can then be accessed everywhere.
//! 
//! ```ignore
//! use egl::API as egl;
//! ```
//! 
//! ### Dynamic Linking
//! 
//! You must enable dynamic linking using the `dynamic` feature in your `Cargo.toml`:
//! ```toml
//! khronos-egl = { version = ..., features = ["dynamic"] }
//! ```
//! 
//! This will add a dependency to the [`libloading`](https://crates.io/crates/libloading) crate,
//! necessary to find the EGL library at runtime.
//! You can then load the EGL API into a `Instance<Dynamic<libloading::Library>>` as follows:
//! 
//! ```ignore
//! let lib = libloading::Library::new("libEGL.so").expect("unable to find libEGL.so");
//! let egl = unsafe { egl::Instance::from_lib(lib).expect("unable to load libEGL.so") };
//! ```
//! 
//! ## Troubleshooting
//! 
//! ### Static Linking with OpenGL ES
//! 
//! When using OpenGL ES with `khronos-egl` with the `static` feature,
//! it is necessary to place a dummy extern at the top of your application which links libEGL first, then GLESv1/2.
//! This is because libEGL provides symbols required by GLESv1/2.
//! Here's how to work around this:
//! 
//! ```
//! ##[link(name = "EGL")]
//! ##[link(name = "GLESv2")]
//! extern {}
//! ```
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![cfg_attr(feature = "nightly", feature(const_fn))]

extern crate libc;

use std::convert::{TryFrom, TryInto};
use std::ffi::CStr;
use std::ffi::CString;
use std::fmt;
use std::ptr;

use libc::{c_uint, c_char, c_void};

// ------------------------------------------------------------------------------------------------
// Link
// ------------------------------------------------------------------------------------------------
#[link(name = "EGL")]
extern {}

// ------------------------------------------------------------------------------------------------
// EGL 1.0
// ------------------------------------------------------------------------------------------------

pub type Boolean = c_uint;
pub type Int = i32;
pub type EGLDisplay = *mut c_void;
pub type EGLConfig = *mut c_void;
pub type EGLContext = *mut c_void;
pub type EGLSurface = *mut c_void;
pub type NativeDisplayType = *mut c_void;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Display(EGLDisplay);

impl Display {
	#[inline]
	pub unsafe fn from_ptr(ptr: EGLDisplay) -> Display {
		Display(ptr)
	}

	#[inline]
	pub fn as_ptr(&self) -> EGLDisplay {
		self.0
	}
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Config(EGLConfig);

impl Config {
	#[inline]
	pub unsafe fn from_ptr(ptr: EGLConfig) -> Config {
		Config(ptr)
	}

	#[inline]
	pub fn as_ptr(&self) -> EGLConfig {
		self.0
	}
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Context(EGLContext);

impl Context {
	#[inline]
	pub unsafe fn from_ptr(ptr: EGLContext) -> Context {
		Context(ptr)
	}

	#[inline]
	pub fn as_ptr(&self) -> EGLContext {
		self.0
	}
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Surface(EGLSurface);

impl Surface {
	#[inline]
	pub unsafe fn from_ptr(ptr: EGLSurface) -> Surface {
		Surface(ptr)
	}

	#[inline]
	pub fn as_ptr(&self) -> EGLSurface {
		self.0
	}
}

#[cfg(not(android))]
pub type NativePixmapType = *mut c_void;

#[cfg(not(android))]
pub type NativeWindowType = *mut c_void;

#[repr(C)]
#[cfg(android)]
struct android_native_window_t;

#[repr(C)]
#[cfg(android)]
struct egl_native_pixmap_t;

#[cfg(android)]
pub type NativePixmapType = *mut egl_native_pixmap_t;

#[cfg(android)]
pub type NativeWindowType = *mut android_native_window_t;

pub const ALPHA_SIZE: Int = 0x3021;
pub const BAD_ACCESS: Int = 0x3002;
pub const BAD_ALLOC: Int = 0x3003;
pub const BAD_ATTRIBUTE: Int = 0x3004;
pub const BAD_CONFIG: Int = 0x3005;
pub const BAD_CONTEXT: Int = 0x3006;
pub const BAD_CURRENT_SURFACE: Int = 0x3007;
pub const BAD_DISPLAY: Int = 0x3008;
pub const BAD_MATCH: Int = 0x3009;
pub const BAD_NATIVE_PIXMAP: Int = 0x300A;
pub const BAD_NATIVE_WINDOW: Int = 0x300B;
pub const BAD_PARAMETER: Int = 0x300C;
pub const BAD_SURFACE: Int = 0x300D;
pub const BLUE_SIZE: Int = 0x3022;
pub const BUFFER_SIZE: Int = 0x3020;
pub const CONFIG_CAVEAT: Int = 0x3027;
pub const CONFIG_ID: Int = 0x3028;
pub const CORE_NATIVE_ENGINE: Int = 0x305B;
pub const DEPTH_SIZE: Int = 0x3025;
pub const DONT_CARE: Int = -1;
pub const DRAW: Int = 0x3059;
pub const EXTENSIONS: Int = 0x3055;
pub const FALSE: Boolean = 0;
pub const GREEN_SIZE: Int = 0x3023;
pub const HEIGHT: Int = 0x3056;
pub const LARGEST_PBUFFER: Int = 0x3058;
pub const LEVEL: Int = 0x3029;
pub const MAX_PBUFFER_HEIGHT: Int = 0x302A;
pub const MAX_PBUFFER_PIXELS: Int = 0x302B;
pub const MAX_PBUFFER_WIDTH: Int = 0x302C;
pub const NATIVE_RENDERABLE: Int = 0x302D;
pub const NATIVE_VISUAL_ID: Int = 0x302E;
pub const NATIVE_VISUAL_TYPE: Int = 0x302F;
pub const NONE: Int = 0x3038;
pub const ATTRIB_NONE: Attrib = 0x3038;
pub const NON_CONFORMANT_CONFIG: Int = 0x3051;
pub const NOT_INITIALIZED: Int = 0x3001;
pub const NO_CONTEXT: EGLContext = 0 as EGLContext;
pub const NO_DISPLAY: EGLDisplay = 0 as EGLDisplay;
pub const NO_SURFACE: EGLSurface = 0 as EGLSurface;
pub const PBUFFER_BIT: Int = 0x0001;
pub const PIXMAP_BIT: Int = 0x0002;
pub const READ: Int = 0x305A;
pub const RED_SIZE: Int = 0x3024;
pub const SAMPLES: Int = 0x3031;
pub const SAMPLE_BUFFERS: Int = 0x3032;
pub const SLOW_CONFIG: Int = 0x3050;
pub const STENCIL_SIZE: Int = 0x3026;
pub const SUCCESS: Int = 0x3000;
pub const SURFACE_TYPE: Int = 0x3033;
pub const TRANSPARENT_BLUE_VALUE: Int = 0x3035;
pub const TRANSPARENT_GREEN_VALUE: Int = 0x3036;
pub const TRANSPARENT_RED_VALUE: Int = 0x3037;
pub const TRANSPARENT_RGB: Int = 0x3052;
pub const TRANSPARENT_TYPE: Int = 0x3034;
pub const TRUE: Boolean = 1;
pub const VENDOR: Int = 0x3053;
pub const VERSION: Int = 0x3054;
pub const WIDTH: Int = 0x3057;
pub const WINDOW_BIT: Int = 0x0004;

/// EGL errors.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Error {
	/// EGL is not initialized, or could not be initialized, for the specified
	/// EGL display connection.
	NotInitialized,

	/// EGL cannot access a requested resource (for example a context is bound
	/// in another thread).
	BadAccess,

	/// EGL failed to allocate resources for the requested operation.
	BadAlloc,

	/// An unrecognized attribute or attribute value was passed in the attribute
	/// list.
	BadAttribute,

	/// An Context argument does not name a valid EGL rendering context.
	BadContext,

	/// An Config argument does not name a valid EGL frame buffer configuration.
	BadConfig,

	/// The current surface of the calling thread is a window, pixel buffer or
	/// pixmap that is no longer valid.
	BadCurrentSurface,

	/// An Display argument does not name a valid EGL display connection.
	BadDisplay,

	/// An Surface argument does not name a valid surface (window, pixel buffer
	/// or pixmap) configured for GL rendering.
	BadSurface,

	/// Arguments are inconsistent (for example, a valid context requires
	/// buffers not supplied by a valid surface).
	BadMatch,

	/// One or more argument values are invalid.
	BadParameter,

	/// A NativePixmapType argument does not refer to a valid native pixmap.
	BadNativePixmap,

	/// A NativeWindowType argument does not refer to a valid native window.
	BadNativeWindow,

	/// A power management event has occurred. The application must destroy all
	/// contexts and reinitialise OpenGL ES state and objects to continue
	/// rendering.
	ContextLost,
}

impl std::error::Error for Error {
	fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
		None
	}
}

impl Error {
	pub fn native(&self) -> Int {
		use Error::*;
		match self {
			NotInitialized => NOT_INITIALIZED,
			BadAccess => BAD_ACCESS,
			BadAlloc => BAD_ALLOC,
			BadAttribute => BAD_ATTRIBUTE,
			BadContext => BAD_CONTEXT,
			BadConfig => BAD_CONFIG,
			BadCurrentSurface => BAD_CURRENT_SURFACE,
			BadDisplay => BAD_DISPLAY,
			BadSurface => BAD_SURFACE,
			BadMatch => BAD_MATCH,
			BadParameter => BAD_PARAMETER,
			BadNativePixmap => BAD_NATIVE_PIXMAP,
			BadNativeWindow => BAD_NATIVE_WINDOW,
			ContextLost => CONTEXT_LOST,
		}
	}

	fn message(&self) -> &'static str {
		use Error::*;
		match self {
			NotInitialized => "EGL is not initialized, or could not be initialized, for the specified EGL display connection.",
			BadAccess => "EGL cannot access a requested resource (for example a context is bound in another thread.",
			BadAlloc => "EGL failed to allocate resources for the requested operation.",
			BadAttribute => "An unrecognized attribute or attribute value was passed in the attribute list.",
			BadContext => "An Context argument does not name a valid EGL rendering context.",
			BadConfig => "An Config argument does not name a valid EGL frame buffer configuration.",
			BadCurrentSurface => "The current surface of the calling thread is a window, pixel buffer or pixmap that is no longer valid.",
			BadDisplay => "An Display argument does not name a valid EGL display connection.",
			BadSurface => "An Surface argument does not name a valid surface (window, pixel buffer or pixmap) configured for GL rendering.",
			BadMatch => "Arguments are inconsistent (for example, a valid context requires buffers not supplied by a valid surface.",
			BadParameter => "One or more argument values are invalid.",
			BadNativePixmap => "A NativePixmapType argument does not refer to a valid native pixmap.",
			BadNativeWindow => "A NativeWindowType argument does not refer to a valid native window.",
			ContextLost => "A power management event has occurred. The application must destroy all contexts and reinitialise OpenGL ES state and objects to continue rendering."
		}
	}
}

impl From<Error> for Int {
	fn from(e: Error) -> Int {
		e.native()
	}
}

impl TryFrom<Int> for Error {
	type Error = Int;

	fn try_from(e: Int) -> Result<Error, Int> {
		use Error::*;
		match e {
			NOT_INITIALIZED => Ok(NotInitialized),
			BAD_ACCESS => Ok(BadAccess),
			BAD_ALLOC => Ok(BadAlloc),
			BAD_ATTRIBUTE => Ok(BadAttribute),
			BAD_CONTEXT => Ok(BadContext),
			BAD_CONFIG => Ok(BadConfig),
			BAD_CURRENT_SURFACE => Ok(BadCurrentSurface),
			BAD_DISPLAY => Ok(BadDisplay),
			BAD_SURFACE => Ok(BadSurface),
			BAD_MATCH => Ok(BadMatch),
			BAD_PARAMETER => Ok(BadParameter),
			BAD_NATIVE_PIXMAP => Ok(BadNativePixmap),
			BAD_NATIVE_WINDOW => Ok(BadNativeWindow),
			CONTEXT_LOST => Ok(ContextLost),
			_ => Err(e),
		}
	}
}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		self.message().fmt(f)
	}
}

pub fn check_int_list(attrib_list: &[Int]) -> Result<(), Error> {
	if attrib_list.last() == Some(&NONE) {
		Ok(())
	} else {
		Err(Error::BadParameter)
	}
}

pub fn check_attrib_list(attrib_list: &[Attrib]) -> Result<(), Error> {
	if attrib_list.last() == Some(&ATTRIB_NONE) {
		Ok(())
	} else {
		Err(Error::BadParameter)
	}
}

/// EGL API instance.
/// 
/// An instance wraps an interface to the EGL API and provide
/// rust-friendly access to it.
pub struct Instance<T: Api> {
	api: T
}

impl<T: Api> Instance<T> {
	#[cfg(feature="nightly")]
	pub const fn new(api: T) -> Instance<T> {
		Instance {
			api
		}
	}

	#[cfg(not(feature="nightly"))]
	pub fn new(api: T) -> Instance<T> {
		Instance {
			api
		}
	}
}

impl<T: Api> Instance<T> {
	/// Return the number of EGL frame buffer configurations that atch specified
	/// attributes.
	///
	/// This will call `eglChooseConfig` without `null` as `configs` to get the
	/// number of matching configurations.
	///
	/// This will return a `BadParameter` error if `attrib_list` is not a valid
	/// attributes list (if it does not terminate with `NONE`).
	pub fn matching_config_count(&self, display: Display, attrib_list: &[Int]) -> Result<usize, Error> {
		check_int_list(attrib_list)?;
		unsafe {
			let mut count = 0;

			if self.api.eglChooseConfig(
				display.as_ptr(),
				attrib_list.as_ptr(),
				ptr::null_mut(),
				0,
				&mut count,
			) == TRUE
			{
				Ok(count as usize)
			} else {
				Err(self.get_error().unwrap())
			}
		}
	}

	/// Return a list of EGL frame buffer configurations that match specified
	/// attributes.
	///
	/// This will write as many matching configurations in `configs` up to its
	/// capacity. You can use the function [`matching_config_count`](Self::matching_config_count) to get the
	/// exact number of configurations matching the specified attributes.
	///
	/// ## Example
	///
	/// ```
	/// # extern crate khronos_egl as egl;
	/// # extern crate wayland_client;
	/// # fn main() -> Result<(), egl::Error> {
	/// # let egl = egl::Instance::new(egl::Static);
	/// # let wayland_display = wayland_client::Display::connect_to_env().expect("unable to connect to the wayland server");
	/// # let display = egl.get_display(wayland_display.get_display_ptr() as *mut std::ffi::c_void).unwrap();
	/// # egl.initialize(display)?;
	/// # let attrib_list = [egl::RED_SIZE, 8, egl::GREEN_SIZE, 8, egl::BLUE_SIZE, 8, egl::NONE];
	/// // Get the number of matching configurations.
	/// let count = egl.matching_config_count(display, &attrib_list)?;
	///
	/// // Get the matching configurations.
	/// let mut configs = Vec::with_capacity(count);
	/// egl.choose_config(display, &attrib_list, &mut configs)?;
	/// # Ok(())
	/// # }
	/// ```
	///
	/// This will return a `BadParameter` error if `attrib_list` is not a valid
	/// attributes list (if it does not terminate with `NONE`).
	pub fn choose_config(&self, display: Display, attrib_list: &[Int], configs: &mut Vec<Config>) -> Result<(), Error> {
		check_int_list(attrib_list)?;
		unsafe {
			let capacity = configs.capacity();
			let mut count = 0;

			if self.api.eglChooseConfig(
				display.as_ptr(),
				attrib_list.as_ptr(),
				configs.as_mut_ptr() as *mut EGLConfig,
				capacity.try_into().unwrap(),
				&mut count,
			) == TRUE
			{
				configs.set_len(count as usize);
				Ok(())
			} else {
				Err(self.get_error().unwrap())
			}
		}
	}

	/// Return the first EGL frame buffer configuration that match specified
	/// attributes.
	///
	/// This is an helper function that will call `choose_config` with a buffer of
	/// size 1, which is equivalent to:
	/// ```
	/// # extern crate khronos_egl as egl;
	/// # extern crate wayland_client;
	/// # fn main() -> Result<(), egl::Error> {
	/// # let egl = egl::Instance::new(egl::Static);
	/// # let wayland_display = wayland_client::Display::connect_to_env().expect("unable to connect to the wayland server");
	/// # let display = egl.get_display(wayland_display.get_display_ptr() as *mut std::ffi::c_void).unwrap();
	/// # egl.initialize(display)?;
	/// # let attrib_list = [egl::RED_SIZE, 8, egl::GREEN_SIZE, 8, egl::BLUE_SIZE, 8, egl::NONE];
	/// let mut configs = Vec::with_capacity(1);
	/// egl.choose_config(display, &attrib_list, &mut configs)?;
	/// configs.first();
	/// # Ok(())
	/// # }
	/// ```
	pub fn choose_first_config(&self, display: Display, attrib_list: &[Int]) -> Result<Option<Config>, Error> {
		let mut configs = Vec::with_capacity(1);
		self.choose_config(display, attrib_list, &mut configs)?;
		Ok(configs.first().map(|config| *config))
	}

	/// Copy EGL surface color buffer to a native pixmap.
	pub fn copy_buffers(&self, display: Display, surface: Surface, target: NativePixmapType) -> Result<(), Error> {
		unsafe {
			if self.api.eglCopyBuffers(display.as_ptr(), surface.as_ptr(), target) == TRUE {
				Ok(())
			} else {
				Err(self.get_error().unwrap())
			}
		}
	}

	/// Create a new EGL rendering context.
	///
	/// This will return a `BadParameter` error if `attrib_list` is not a valid
	/// attributes list (if it does not terminate with `NONE`).
	pub fn create_context(&self, display: Display, config: Config, share_context: Option<Context>, attrib_list: &[Int]) -> Result<Context, Error> {
		check_int_list(attrib_list)?;
		unsafe {
			let share_context = match share_context {
				Some(share_context) => share_context.as_ptr(),
				None => NO_CONTEXT,
			};

			let context = self.api.eglCreateContext(
				display.as_ptr(),
				config.as_ptr(),
				share_context,
				attrib_list.as_ptr(),
			);

			if context != NO_CONTEXT {
				Ok(Context(context))
			} else {
				Err(self.get_error().unwrap())
			}
		}
	}

	/// Create a new EGL pixel buffer surface.
	///
	/// This will return a `BadParameter` error if `attrib_list` is not a valid
	/// attributes list (if it does not terminate with `NONE`).
	pub fn create_pbuffer_surface(&self, display: Display, config: Config, attrib_list: &[Int]) -> Result<Surface, Error> {
		check_int_list(attrib_list)?;
		unsafe {
			let surface =
				self.api.eglCreatePbufferSurface(display.as_ptr(), config.as_ptr(), attrib_list.as_ptr());

			if surface != NO_SURFACE {
				Ok(Surface(surface))
			} else {
				Err(self.get_error().unwrap())
			}
		}
	}

	/// Create a new EGL offscreen surface.
	///
	/// This will return a `BadParameter` error if `attrib_list` is not a valid
	/// attributes list (if it does not terminate with `NONE`).
	///
	/// Since this function may raise undefined behavior if the display and native
	/// pixmap do not belong to the same platform, it is inherently unsafe.
	pub unsafe fn create_pixmap_surface(&self, display: Display, config: Config, pixmap: NativePixmapType, attrib_list: &[Int]) -> Result<Surface, Error> {
		check_int_list(attrib_list)?;
		let surface = self.api.eglCreatePixmapSurface(
			display.as_ptr(),
			config.as_ptr(),
			pixmap,
			attrib_list.as_ptr(),
		);

		if surface != NO_SURFACE {
			Ok(Surface(surface))
		} else {
			Err(self.get_error().unwrap())
		}
	}

	/// Create a new EGL window surface.
	///
	/// This will return a `BadParameter` error if `attrib_list` is not a valid
	/// attributes list (if it does not terminate with `NONE`).
	///
	/// Since this function may raise undefined behavior if the display and native
	/// window do not belong to the same platform, it is inherently unsafe.
	pub unsafe fn create_window_surface(&self, display: Display, config: Config, window: NativeWindowType, attrib_list: Option<&[Int]>) -> Result<Surface, Error> {
		let attrib_list = match attrib_list {
			Some(attrib_list) => {
				check_int_list(attrib_list)?;
				attrib_list.as_ptr()
			}
			None => ptr::null(),
		};

		let surface =
			self.api.eglCreateWindowSurface(display.as_ptr(), config.as_ptr(), window, attrib_list);

		if surface != NO_SURFACE {
			Ok(Surface(surface))
		} else {
			Err(self.get_error().unwrap())
		}
	}

	/// Destroy an EGL rendering context.
	pub fn destroy_context(&self, display: Display, ctx: Context) -> Result<(), Error> {
		unsafe {
			if self.api.eglDestroyContext(display.as_ptr(), ctx.as_ptr()) == TRUE {
				Ok(())
			} else {
				Err(self.get_error().unwrap())
			}
		}
	}

	/// Destroy an EGL surface.
	pub fn destroy_surface(&self, display: Display, surface: Surface) -> Result<(), Error> {
		unsafe {
			if self.api.eglDestroySurface(display.as_ptr(), surface.as_ptr()) == TRUE {
				Ok(())
			} else {
				Err(self.get_error().unwrap())
			}
		}
	}

	/// Return information about an EGL frame buffer configuration.
	pub fn get_config_attrib(&self, display: Display, config: Config, attribute: Int) -> Result<Int, Error> {
		unsafe {
			let mut value: Int = 0;
			if self.api.eglGetConfigAttrib(display.as_ptr(), config.as_ptr(), attribute, &mut value) == TRUE
			{
				Ok(value)
			} else {
				Err(self.get_error().unwrap())
			}
		}
	}

	/// Return the number of all frame buffer configurations.
	/// 
	/// You can use it to setup the correct capacity for the configurations buffer in [`get_configs`](Self::get_configs).
	/// 
	/// ## Example
	/// ```
	/// # extern crate khronos_egl as egl;
	/// # extern crate wayland_client;
	/// # fn main() -> Result<(), egl::Error> {
	/// # let egl = egl::Instance::new(egl::Static);
	/// # let wayland_display = wayland_client::Display::connect_to_env().expect("unable to connect to the wayland server");
	/// # let display = egl.get_display(wayland_display.get_display_ptr() as *mut std::ffi::c_void).unwrap();
	/// # egl.initialize(display)?;
	/// let mut configs = Vec::with_capacity(egl.get_config_count(display)?);
	/// egl.get_configs(display, &mut configs);
	/// assert!(configs.len() > 0);
	/// # Ok(())
	/// # }
	/// ```
	pub fn get_config_count(&self, display: Display) -> Result<usize, Error> {
		unsafe {
			let mut count = 0;

			if self.api.eglGetConfigs(display.as_ptr(), std::ptr::null_mut(), 0, &mut count) == TRUE {
				Ok(count as usize)
			} else {
				Err(self.get_error().unwrap())
			}
		}
	}

	/// Get the list of all EGL frame buffer configurations for a display.
	/// 
	/// The configurations are added to the `configs` buffer, up to the buffer's capacity.
	/// You can use [`get_config_count`](Self::get_config_count) to get the total number of available frame buffer configurations,
	/// and setup the buffer's capacity accordingly.
	/// 
	/// ## Example
	/// ```
	/// # extern crate khronos_egl as egl;
	/// # extern crate wayland_client;
	/// # fn main() -> Result<(), egl::Error> {
	/// # let egl = egl::Instance::new(egl::Static);
	/// # let wayland_display = wayland_client::Display::connect_to_env().expect("unable to connect to the wayland server");
	/// # let display = egl.get_display(wayland_display.get_display_ptr() as *mut std::ffi::c_void).unwrap();
	/// # egl.initialize(display)?;
	/// let mut configs = Vec::with_capacity(egl.get_config_count(display)?);
	/// egl.get_configs(display, &mut configs);
	/// # Ok(())
	/// # }
	/// ```
	pub fn get_configs(&self, display: Display, configs: &mut Vec<Config>) -> Result<(), Error> {
		unsafe {
			let capacity = configs.capacity();
			let mut count = 0;

			if self.api.eglGetConfigs(
				display.as_ptr(),
				configs.as_mut_ptr() as *mut EGLConfig,
				capacity.try_into().unwrap(),
				&mut count,
			) == TRUE
			{
				configs.set_len(count as usize);
				Ok(())
			} else {
				Err(self.get_error().unwrap())
			}
		}
	}

	/// Return the display for the current EGL rendering context.
	pub fn get_current_display(&self) -> Option<Display> {
		unsafe {
			let display = self.api.eglGetCurrentDisplay();

			if display != NO_DISPLAY {
				Some(Display(display))
			} else {
				None
			}
		}
	}

	/// Return the read or draw surface for the current EGL rendering context.
	pub fn get_current_surface(&self, readdraw: Int) -> Option<Surface> {
		unsafe {
			let surface = self.api.eglGetCurrentSurface(readdraw);

			if surface != NO_SURFACE {
				Some(Surface(surface))
			} else {
				None
			}
		}
	}

	/// Return an EGL display connection.
	pub fn get_display(&self, display_id: NativeDisplayType) -> Option<Display> {
		unsafe {
			let display = self.api.eglGetDisplay(display_id);

			if display != NO_DISPLAY {
				Some(Display(display))
			} else {
				None
			}
		}
	}

	/// Return error information.
	///
	/// Return the error of the last called EGL function in the current thread, or
	/// `None` if the error is set to `SUCCESS`.
	///
	/// Note that since a call to `eglGetError` sets the error to `SUCCESS`, and
	/// since this function is automatically called by any wrapper function
	/// returning a `Result` when necessary, this function may only return `None`
	/// from the point of view of a user.
	pub fn get_error(&self) -> Option<Error> {
		unsafe {
			let e = self.api.eglGetError();
			if e == SUCCESS {
				None
			} else {
				Some(e.try_into().unwrap())
			}
		}
	}

	/// Return a GL or an EGL extension function.
	pub fn get_proc_address(&self, procname: &str) -> Option<extern "C" fn()> {
		unsafe {
			let string = CString::new(procname).unwrap();

			let addr = self.api.eglGetProcAddress(string.as_ptr());
			if !(addr as *const ()).is_null() {
				Some(addr)
			} else {
				None
			}
		}
	}

	/// Initialize an EGL display connection.
	pub fn initialize(&self, display: Display) -> Result<(Int, Int), Error> {
		unsafe {
			let mut major = 0;
			let mut minor = 0;

			if self.api.eglInitialize(display.as_ptr(), &mut major, &mut minor) == TRUE {
				Ok((major, minor))
			} else {
				Err(self.get_error().unwrap())
			}
		}
	}

	/// Attach an EGL rendering context to EGL surfaces.
	pub fn make_current(&self, display: Display, draw: Option<Surface>, read: Option<Surface>, ctx: Option<Context>) -> Result<(), Error> {
		unsafe {
			let draw = match draw {
				Some(draw) => draw.as_ptr(),
				None => NO_SURFACE,
			};
			let read = match read {
				Some(read) => read.as_ptr(),
				None => NO_SURFACE,
			};
			let ctx = match ctx {
				Some(ctx) => ctx.as_ptr(),
				None => NO_CONTEXT,
			};

			if self.api.eglMakeCurrent(display.as_ptr(), draw, read, ctx) == TRUE {
				Ok(())
			} else {
				Err(self.get_error().unwrap())
			}
		}
	}

	/// Return EGL rendering context information.
	pub fn query_context(&self, display: Display, ctx: Context, attribute: Int) -> Result<Int, Error> {
		unsafe {
			let mut value = 0;
			if self.api.eglQueryContext(display.as_ptr(), ctx.as_ptr(), attribute, &mut value) == TRUE {
				Ok(value)
			} else {
				Err(self.get_error().unwrap())
			}
		}
	}

	/// Return a string describing properties of the EGL client or of an EGL display
	/// connection.
	pub fn query_string(&self, display: Option<Display>, name: Int) -> Result<&'static CStr, Error> {
		unsafe {
			let display_ptr = match display {
				Some(display) => display.as_ptr(),
				None => NO_DISPLAY
			};

			let c_str = self.api.eglQueryString(display_ptr, name);

			if !c_str.is_null() {
				Ok(CStr::from_ptr(c_str))
			} else {
				Err(self.get_error().unwrap())
			}
		}
	}

	/// Return EGL surface information.
	pub fn query_surface(&self, display: Display, surface: Surface, attribute: Int) -> Result<Int, Error> {
		unsafe {
			let mut value = 0;
			if self.api.eglQuerySurface(display.as_ptr(), surface.as_ptr(), attribute, &mut value) == TRUE {
				Ok(value)
			} else {
				Err(self.get_error().unwrap())
			}
		}
	}

	/// Post EGL surface color buffer to a native window.
	pub fn swap_buffers(&self, display: Display, surface: Surface) -> Result<(), Error> {
		unsafe {
			if self.api.eglSwapBuffers(display.as_ptr(), surface.as_ptr()) == TRUE {
				Ok(())
			} else {
				Err(self.get_error().unwrap())
			}
		}
	}

	/// Terminate an EGL display connection.
	pub fn terminate(&self, display: Display) -> Result<(), Error> {
		unsafe {
			if self.api.eglTerminate(display.as_ptr()) == TRUE {
				Ok(())
			} else {
				Err(self.get_error().unwrap())
			}
		}
	}

	/// Complete GL execution prior to subsequent native rendering calls.
	pub fn wait_gl(&self) -> Result<(), Error> {
		unsafe {
			if self.api.eglWaitGL() == TRUE {
				Ok(())
			} else {
				Err(self.get_error().unwrap())
			}
		}
	}

	/// Complete native execution prior to subsequent GL rendering calls.
	pub fn wait_native(&self, engine: Int) -> Result<(), Error> {
		unsafe {
			if self.api.eglWaitNative(engine) == TRUE {
				Ok(())
			} else {
				Err(self.get_error().unwrap())
			}
		}
	}
}

// ------------------------------------------------------------------------------------------------
// EGL 1.1
// ------------------------------------------------------------------------------------------------

pub const BACK_BUFFER: Int = 0x3084;
pub const BIND_TO_TEXTURE_RGB: Int = 0x3039;
pub const BIND_TO_TEXTURE_RGBA: Int = 0x303A;
pub const CONTEXT_LOST: Int = 0x300E;
pub const MIN_SWAP_INTERVAL: Int = 0x303B;
pub const MAX_SWAP_INTERVAL: Int = 0x303C;
pub const MIPMAP_TEXTURE: Int = 0x3082;
pub const MIPMAP_LEVEL: Int = 0x3083;
pub const NO_TEXTURE: Int = 0x305C;
pub const TEXTURE_2D: Int = 0x305F;
pub const TEXTURE_FORMAT: Int = 0x3080;
pub const TEXTURE_RGB: Int = 0x305D;
pub const TEXTURE_RGBA: Int = 0x305E;
pub const TEXTURE_TARGET: Int = 0x3081;

impl<T: Api> Instance<T> {
	/// Defines a two-dimensional texture image.
	pub fn bind_tex_image(&self, display: Display, surface: Surface, buffer: Int) -> Result<(), Error> {
		unsafe {
			if self.api.eglBindTexImage(display.as_ptr(), surface.as_ptr(), buffer) == TRUE {
				Ok(())
			} else {
				Err(self.get_error().unwrap())
			}
		}
	}

	///  Releases a color buffer that is being used as a texture.
	pub fn release_tex_image(&self, display: Display, surface: Surface, buffer: Int) -> Result<(), Error> {
		unsafe {
			if self.api.eglReleaseTexImage(display.as_ptr(), surface.as_ptr(), buffer) == TRUE {
				Ok(())
			} else {
				Err(self.get_error().unwrap())
			}
		}
	}

	/// Set an EGL surface attribute.
	pub fn surface_attrib(&self, display: Display, surface: Surface, attribute: Int, value: Int) -> Result<(), Error> {
		unsafe {
			if self.api.eglSurfaceAttrib(display.as_ptr(), surface.as_ptr(), attribute, value) == TRUE {
				Ok(())
			} else {
				Err(self.get_error().unwrap())
			}
		}
	}

	/// Specifies the minimum number of video frame periods per buffer swap for the
	/// window associated with the current context.
	pub fn swap_interval(&self, display: Display, interval: Int) -> Result<(), Error> {
		unsafe {
			if self.api.eglSwapInterval(display.as_ptr(), interval) == TRUE {
				Ok(())
			} else {
				Err(self.get_error().unwrap())
			}
		}
	}
}

// ------------------------------------------------------------------------------------------------
// EGL 1.2
// ------------------------------------------------------------------------------------------------

pub type Enum = c_uint;
pub type EGLClientBuffer = *mut c_void;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct ClientBuffer(EGLClientBuffer);

impl ClientBuffer {
	#[inline]
	pub unsafe fn from_ptr(ptr: EGLClientBuffer) -> ClientBuffer {
		ClientBuffer(ptr)
	}

	#[inline]
	pub fn as_ptr(&self) -> EGLClientBuffer {
		self.0
	}
}

pub const ALPHA_FORMAT: Int = 0x3088;
pub const ALPHA_FORMAT_NONPRE: Int = 0x308B;
pub const ALPHA_FORMAT_PRE: Int = 0x308C;
pub const ALPHA_MASK_SIZE: Int = 0x303E;
pub const BUFFER_PRESERVED: Int = 0x3094;
pub const BUFFER_DESTROYED: Int = 0x3095;
pub const CLIENT_APIS: Int = 0x308D;
pub const COLORSPACE: Int = 0x3087;
pub const COLORSPACE_sRGB: Int = 0x3089;
pub const COLORSPACE_LINEAR: Int = 0x308A;
pub const COLOR_BUFFER_TYPE: Int = 0x303F;
pub const CONTEXT_CLIENT_TYPE: Int = 0x3097;
pub const DISPLAY_SCALING: Int = 10000;
pub const HORIZONTAL_RESOLUTION: Int = 0x3090;
pub const LUMINANCE_BUFFER: Int = 0x308F;
pub const LUMINANCE_SIZE: Int = 0x303D;
pub const OPENGL_ES_BIT: Int = 0x0001;
pub const OPENVG_BIT: Int = 0x0002;
pub const OPENGL_ES_API: Enum = 0x30A0;
pub const OPENVG_API: Enum = 0x30A1;
pub const OPENVG_IMAGE: Int = 0x3096;
pub const PIXEL_ASPECT_RATIO: Int = 0x3092;
pub const RENDERABLE_TYPE: Int = 0x3040;
pub const RENDER_BUFFER: Int = 0x3086;
pub const RGB_BUFFER: Int = 0x308E;
pub const SINGLE_BUFFER: Int = 0x3085;
pub const SWAP_BEHAVIOR: Int = 0x3093;
pub const UNKNOWN: Int = -1;
pub const VERTICAL_RESOLUTION: Int = 0x3091;

impl<T: Api> Instance<T> {
	/// Set the current rendering API.
	pub fn bind_api(&self, api: Enum) -> Result<(), Error> {
		unsafe {
			if self.api.eglBindAPI(api) == TRUE {
				Ok(())
			} else {
				Err(self.get_error().unwrap())
			}
		}
	}

	/// Query the current rendering API.
	pub fn query_api(&self) -> Enum {
		unsafe { self.api.eglQueryAPI() }
	}

	/// Create a new EGL pixel buffer surface bound to an OpenVG image.
	///
	/// This will return a `BadParameter` error if `attrib_list` is not a valid
	/// attributes list (if it does not terminate with `NONE`).
	pub fn create_pbuffer_from_client_buffer(&self, display: Display, buffer_type: Enum, buffer: ClientBuffer, config: Config, attrib_list: &[Int]) -> Result<Surface, Error> {
		check_int_list(attrib_list)?;
		unsafe {
			let surface = self.api.eglCreatePbufferFromClientBuffer(
				display.as_ptr(),
				buffer_type,
				buffer.as_ptr(),
				config.as_ptr(),
				attrib_list.as_ptr(),
			);

			if surface != NO_SURFACE {
				Ok(Surface(surface))
			} else {
				Err(self.get_error().unwrap())
			}
		}
	}

	/// Release EGL per-thread state.
	pub fn release_thread(&self) -> Result<(), Error> {
		unsafe {
			if self.api.eglReleaseThread() == TRUE {
				Ok(())
			} else {
				Err(self.get_error().unwrap())
			}
		}
	}

	/// Complete client API execution prior to subsequent native rendering calls.
	pub fn wait_client(&self) -> Result<(), Error> {
		unsafe {
			if self.api.eglWaitClient() == TRUE {
				Ok(())
			} else {
				Err(self.get_error().unwrap())
			}
		}
	}
}

// ------------------------------------------------------------------------------------------------
// EGL 1.3
// ------------------------------------------------------------------------------------------------

pub const CONFORMANT: Int = 0x3042;
pub const CONTEXT_CLIENT_VERSION: Int = 0x3098;
pub const MATCH_NATIVE_PIXMAP: Int = 0x3041;
pub const OPENGL_ES2_BIT: Int = 0x0004;
pub const VG_ALPHA_FORMAT: Int = 0x3088;
pub const VG_ALPHA_FORMAT_NONPRE: Int = 0x308B;
pub const VG_ALPHA_FORMAT_PRE: Int = 0x308C;
pub const VG_ALPHA_FORMAT_PRE_BIT: Int = 0x0040;
pub const VG_COLORSPACE: Int = 0x3087;
pub const VG_COLORSPACE_sRGB: Int = 0x3089;
pub const VG_COLORSPACE_LINEAR: Int = 0x308A;
pub const VG_COLORSPACE_LINEAR_BIT: Int = 0x0020;

// ------------------------------------------------------------------------------------------------
// EGL 1.4
// ------------------------------------------------------------------------------------------------

pub const DEFAULT_DISPLAY: NativeDisplayType = 0 as NativeDisplayType;
pub const MULTISAMPLE_RESOLVE_BOX_BIT: Int = 0x0200;
pub const MULTISAMPLE_RESOLVE: Int = 0x3099;
pub const MULTISAMPLE_RESOLVE_DEFAULT: Int = 0x309A;
pub const MULTISAMPLE_RESOLVE_BOX: Int = 0x309B;
pub const OPENGL_API: Enum = 0x30A2;
pub const OPENGL_BIT: Int = 0x0008;
pub const SWAP_BEHAVIOR_PRESERVED_BIT: Int = 0x0400;

impl<T: Api> Instance<T> {
	/// Return the current EGL rendering context.
	pub fn get_current_context(&self) -> Option<Context> {
		unsafe {
			let context = self.api.eglGetCurrentContext();

			if context != NO_CONTEXT {
				Some(Context(context))
			} else {
				None
			}
		}
	}
}

// ------------------------------------------------------------------------------------------------
// EGL 1.5
// ------------------------------------------------------------------------------------------------

pub type Attrib = usize;
pub type Time = u64;
pub type EGLSync = *mut c_void;
pub type EGLImage = *mut c_void;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Sync(EGLSync);

impl Sync {
	#[inline]
	pub unsafe fn from_ptr(ptr: EGLSync) -> Sync {
		Sync(ptr)
	}

	#[inline]
	pub fn as_ptr(&self) -> EGLSync {
		self.0
	}
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Image(EGLImage);

impl Image {
	#[inline]
	pub unsafe fn from_ptr(ptr: EGLImage) -> Image {
		Image(ptr)
	}

	#[inline]
	pub fn as_ptr(&self) -> EGLImage {
		self.0
	}
}

pub const CONTEXT_MAJOR_VERSION: Int = 0x3098;
pub const CONTEXT_MINOR_VERSION: Int = 0x30FB;
pub const CONTEXT_OPENGL_PROFILE_MASK: Int = 0x30FD;
pub const CONTEXT_OPENGL_RESET_NOTIFICATION_STRATEGY: Int = 0x31BD;
pub const NO_RESET_NOTIFICATION: Int = 0x31BE;
pub const LOSE_CONTEXT_ON_RESET: Int = 0x31BF;
pub const CONTEXT_OPENGL_CORE_PROFILE_BIT: Int = 0x00000001;
pub const CONTEXT_OPENGL_COMPATIBILITY_PROFILE_BIT: Int = 0x00000002;
pub const CONTEXT_OPENGL_DEBUG: Int = 0x31B0;
pub const CONTEXT_OPENGL_FORWARD_COMPATIBLE: Int = 0x31B1;
pub const CONTEXT_OPENGL_ROBUST_ACCESS: Int = 0x31B2;
pub const OPENGL_ES3_BIT: Int = 0x00000040;
pub const CL_EVENT_HANDLE: Int = 0x309C;
pub const SYNC_CL_EVENT: Int = 0x30FE;
pub const SYNC_CL_EVENT_COMPLETE: Int = 0x30FF;
pub const SYNC_PRIOR_COMMANDS_COMPLETE: Int = 0x30F0;
pub const SYNC_TYPE: Int = 0x30F7;
pub const SYNC_STATUS: Int = 0x30F1;
pub const SYNC_CONDITION: Int = 0x30F8;
pub const SIGNALED: Int = 0x30F2;
pub const UNSIGNALED: Int = 0x30F3;
pub const SYNC_FLUSH_COMMANDS_BIT: Int = 0x0001;
pub const FOREVER: u64 = 0xFFFFFFFFFFFFFFFFu64;
pub const TIMEOUT_EXPIRED: Int = 0x30F5;
pub const CONDITION_SATISFIED: Int = 0x30F6;
pub const NO_SYNC: EGLSync = 0 as EGLSync;
pub const SYNC_FENCE: Int = 0x30F9;
pub const GL_COLORSPACE: Int = 0x309D;
pub const GL_COLORSPACE_SRGB: Int = 0x3089;
pub const GL_COLORSPACE_LINEAR: Int = 0x308A;
pub const GL_RENDERBUFFER: Int = 0x30B9;
pub const GL_TEXTURE_2D: Int = 0x30B1;
pub const GL_TEXTURE_LEVEL: Int = 0x30BC;
pub const GL_TEXTURE_3D: Int = 0x30B2;
pub const GL_TEXTURE_ZOFFSET: Int = 0x30BD;
pub const GL_TEXTURE_CUBE_MAP_POSITIVE_X: Int = 0x30B3;
pub const GL_TEXTURE_CUBE_MAP_NEGATIVE_X: Int = 0x30B4;
pub const GL_TEXTURE_CUBE_MAP_POSITIVE_Y: Int = 0x30B5;
pub const GL_TEXTURE_CUBE_MAP_NEGATIVE_Y: Int = 0x30B6;
pub const GL_TEXTURE_CUBE_MAP_POSITIVE_Z: Int = 0x30B7;
pub const GL_TEXTURE_CUBE_MAP_NEGATIVE_Z: Int = 0x30B8;
pub const IMAGE_PRESERVED: Int = 0x30D2;
pub const NO_IMAGE: EGLImage = 0 as EGLImage;

impl<T: Api> Instance<T> {
	/// Create a new EGL sync object.
	///
	/// Note that the constant `ATTRIB_NONE` which has the type `Attrib` can be used
	/// instead of `NONE` to terminate the attribute list.
	///
	/// This will return a `BadParameter` error if `attrib_list` is not a valid
	/// attributes list (if it does not terminate with `ATTRIB_NONE`).
	///
	/// This function is unsafe: when creating an OpenCL Event Sync Object, passing an invalid event
	/// handle in `attrib_list` may result in undefined behavior up to and including program
	/// termination.
	pub unsafe fn create_sync(&self, display: Display, ty: Enum, attrib_list: &[Attrib]) -> Result<Sync, Error> {
		check_attrib_list(attrib_list)?;
		let sync = self.api.eglCreateSync(display.as_ptr(), ty, attrib_list.as_ptr());
		if sync != NO_SYNC {
			Ok(Sync(sync))
		} else {
			Err(self.get_error().unwrap())
		}
	}

	/// Destroy a sync object.
	///
	/// This function is unsafe: if display does not match the display passed to eglCreateSync when
	/// sync was created, the behaviour is undefined.
	pub unsafe fn destroy_sync(&self, display: Display, sync: Sync) -> Result<(), Error> {
		if self.api.eglDestroySync(display.as_ptr(), sync.as_ptr()) == TRUE {
			Ok(())
		} else {
			Err(self.get_error().unwrap())
		}
	}

	/// Wait in the client for a sync object to be signalled.
	///
	/// This function is unsafe: if `display` does not match the [`Display`] passed to [`create_sync`](Self::create_sync)
	/// when `sync` was created, the behaviour is undefined.
	pub unsafe fn client_wait_sync(&self, display: Display, sync: Sync, flags: Int, timeout: Time) -> Result<Int, Error> {
		let status = self.api.eglClientWaitSync(display.as_ptr(), sync.as_ptr(), flags, timeout);
		if status != FALSE as Int {
			Ok(status)
		} else {
			Err(self.get_error().unwrap())
		}
	}

	/// Return an attribute of a sync object.
	///
	/// This function is unsafe: If `display` does not match the [`Display`] passed to [`create_sync`](Self::create_sync)
	/// when `sync` was created, behaviour is undefined.
	pub unsafe fn get_sync_attrib(&self, display: Display, sync: Sync, attribute: Int) -> Result<Attrib, Error> {
		let mut value = 0;
		if self.api.eglGetSyncAttrib(
			display.as_ptr(),
			sync.as_ptr(),
			attribute,
			&mut value as *mut Attrib,
		) == TRUE
		{
			Ok(value)
		} else {
			Err(self.get_error().unwrap())
		}
	}

	/// Create a new Image object.
	///
	/// Note that the constant `ATTRIB_NONE` which has the type `Attrib` can be used
	/// instead of `NONE` to terminate the attribute list.
	///
	/// This will return a `BadParameter` error if `attrib_list` is not a valid
	/// attributes list (if it does not terminate with `ATTRIB_NONE`).
	pub fn create_image(&self, display: Display, ctx: Context, target: Enum, buffer: ClientBuffer, attrib_list: &[Attrib]) -> Result<Image, Error> {
		check_attrib_list(attrib_list)?;
		unsafe {
			let image = self.api.eglCreateImage(
				display.as_ptr(),
				ctx.as_ptr(),
				target,
				buffer.as_ptr(),
				attrib_list.as_ptr(),
			);
			if image != NO_IMAGE {
				Ok(Image(image))
			} else {
				Err(self.get_error().unwrap())
			}
		}
	}

	/// Destroy an Image object.
	pub fn destroy_image(&self, display: Display, image: Image) -> Result<(), Error> {
		unsafe {
			if self.api.eglDestroyImage(display.as_ptr(), image.as_ptr()) == TRUE {
				Ok(())
			} else {
				Err(self.get_error().unwrap())
			}
		}
	}

	/// Return an EGL display connection.
	///
	/// Note that the constant `ATTRIB_NONE` which has the type `Attrib` can be used
	/// instead of `NONE` to terminate the attribute list.
	///
	/// This will return a `BadParameter` error if `attrib_list` is not a valid
	/// attributes list (if it does not terminate with `ATTRIB_NONE`).
	pub fn get_platform_display(&self, platform: Enum, native_display: *mut c_void, attrib_list: &[Attrib]) -> Result<Display, Error> {
		check_attrib_list(attrib_list)?;
		unsafe {
			let display = self.api.eglGetPlatformDisplay(platform, native_display, attrib_list.as_ptr());
			if display != NO_DISPLAY {
				Ok(Display(display))
			} else {
				Err(self.get_error().unwrap())
			}
		}
	}

	/// Create a new EGL on-screen rendering surface.
	///
	/// Note that the constant `ATTRIB_NONE` which has the type `Attrib` can be used
	/// instead of `NONE` to terminate the attribute list.
	///
	/// This will return a `BadParameter` error if `attrib_list` is not a valid
	/// attributes list (if it does not terminate with `ATTRIB_NONE`).
	pub fn create_platform_window_surface(&self, display: Display, config: Config, native_window: *mut c_void, attrib_list: &[Attrib]) -> Result<Surface, Error> {
		check_attrib_list(attrib_list)?;
		unsafe {
			let surface = self.api.eglCreatePlatformWindowSurface(
				display.as_ptr(),
				config.as_ptr(),
				native_window,
				attrib_list.as_ptr(),
			);
			if surface != NO_SURFACE {
				Ok(Surface(surface))
			} else {
				Err(self.get_error().unwrap())
			}
		}
	}

	/// Create a new EGL offscreen surface.
	///
	/// Note that the constant `ATTRIB_NONE` which has the type `Attrib` can be used
	/// instead of `NONE` to terminate the attribute list.
	///
	/// This will return a `BadParameter` error if `attrib_list` is not a valid
	/// attributes list (if it does not terminate with `ATTRIB_NONE`).
	pub fn create_platform_pixmap_surface(&self, display: Display, config: Config, native_pixmap: *mut c_void, attrib_list: &[Attrib]) -> Result<Surface, Error> {
		check_attrib_list(attrib_list)?;
		unsafe {
			let surface = self.api.eglCreatePlatformPixmapSurface(
				display.as_ptr(),
				config.as_ptr(),
				native_pixmap,
				attrib_list.as_ptr(),
			);
			if surface != NO_SURFACE {
				Ok(Surface(surface))
			} else {
				Err(self.get_error().unwrap())
			}
		}
	}

	/// Wait in the server for a sync object to be signalled.
	///
	/// This function is unsafe: if `display` does not match the [`Display`] passed to [`create_sync`](Self::create_sync)
	/// when `sync` was created, the behavior is undefined.
	pub fn wait_sync(&self, display: Display, sync: Sync, flags: Int) -> Result<(), Error> {
		unsafe {
			if self.api.eglWaitSync(display.as_ptr(), sync.as_ptr(), flags) == TRUE {
				Ok(())
			} else {
				Err(self.get_error().unwrap())
			}
		}
	}
}

// -------------------------------------------------------------------------------------------------
// FFI
// -------------------------------------------------------------------------------------------------

macro_rules! api {
	($(pub fn $name:ident ($($arg:ident : $atype:ty ),* ) -> $rtype:ty);*) => {
		/// EGL API interface.
		/// 
		/// An implementation of this trait can be used to create an [`Instance`].
		/// 
		/// This crate provides two implemntation of this trait:
		///  - `Static` which is available with the `static` feature enabled,
		///    defined by statically linking to the EGL library at compile time.
		///  - `Dynamic` which is available with the `dynamic` feature enabled,
		///    defined by dynamically linking to the EGL library at runtime.
		pub unsafe trait Api {
			$(
				unsafe fn $name(&self, $($arg : $atype),*) -> $rtype;
			)*
		}

		#[cfg(feature="static")]
		/// Static EGL API interface.
		/// 
		/// Provided by statically linking th EGL library at compile time.
		pub struct Static;

		#[cfg(feature="static")]
		unsafe impl Api for Static {
			$(
				#[inline(always)]
				unsafe fn $name(&self, $($arg : $atype),*) -> $rtype {
					ffi::$name($($arg),*)
				}
			)*
		}

		#[cfg(feature="static")]
		mod ffi {
			use libc::{c_char, c_void};
			
			use super::{
				Attrib, Boolean, EGLClientBuffer, EGLConfig, EGLContext, EGLDisplay, EGLImage, EGLSurface,
				EGLSync, Enum, Int, NativeDisplayType, NativePixmapType, NativeWindowType, Time,
			};

			extern "C" {
				$(
					pub fn $name ($($arg : $atype ),* ) -> $rtype;
				)*
			}
		}

		#[cfg(feature="static")]
		#[cfg(feature="nightly")]
		pub static API: Instance<Static> = Instance::new(Static);

		#[cfg(feature="dynamic")]
		extern crate libloading;

		#[cfg(feature="dynamic")]
		/// Dynamic EGL API interface.
		pub struct Dynamic<L: std::borrow::Borrow<libloading::Library>> {
			lib: L,
			$(
				$name : unsafe extern "C" fn($($atype ),*) -> $rtype,
			)*
		}

		#[cfg(feature="dynamic")]
		unsafe impl<L: std::borrow::Borrow<libloading::Library> + Send> Send for Dynamic<L> {}

		#[cfg(feature="dynamic")]
		unsafe impl<L: std::borrow::Borrow<libloading::Library> + std::marker::Sync> std::marker::Sync for Dynamic<L> {}

		#[cfg(feature="dynamic")]
		impl<L: std::borrow::Borrow<libloading::Library>> Dynamic<L> {
			#[inline]
			pub unsafe fn new(lib: L) -> Result<Dynamic<L>, libloading::Error> {
				$(
					let $name = (&lib.borrow().get::<unsafe extern "C" fn($($atype ),*) -> $rtype>(stringify!($name).as_bytes())?.into_raw().into_raw()) as *const _ as *const unsafe extern "C" fn($($atype ),*) -> $rtype;
					assert!(!$name.is_null());
				)*

				Ok(Dynamic {
					lib,
					$(
						$name: *$name,
					)*
				})
			}

			#[inline(always)]
			pub fn library(&self) -> &L {
				&self.lib
			}
		}

		#[cfg(feature="dynamic")]
		unsafe impl<L: std::borrow::Borrow<libloading::Library>> Api for Dynamic<L> {
			$(
				#[inline(always)]
				unsafe fn $name(&self, $($arg : $atype),*) -> $rtype {
					(self.$name)($($arg),*)
				}
			)*
		}

		#[cfg(feature="dynamic")]
		impl<L: std::borrow::Borrow<libloading::Library>> Instance<Dynamic<L>> {
			#[inline(always)]
			pub unsafe fn from_lib(lib: L) -> Result<Instance<Dynamic<L>>, libloading::Error> {
				Ok(Instance::new(Dynamic::new(lib)?))
			}
		}
	};
}

api! {
	// EGL 1.0
	pub fn eglChooseConfig(
		display: EGLDisplay,
		attrib_list: *const Int,
		configs: *mut EGLConfig,
		config_size: Int,
		num_config: *mut Int
	) -> Boolean;
	pub fn eglCopyBuffers(
		display: EGLDisplay,
		surface: EGLSurface,
		target: NativePixmapType
	) -> Boolean;
	pub fn eglCreateContext(
		display: EGLDisplay,
		config: EGLConfig,
		share_context: EGLContext,
		attrib_list: *const Int
	) -> EGLContext;
	pub fn eglCreatePbufferSurface(
		display: EGLDisplay,
		config: EGLConfig,
		attrib_list: *const Int
	) -> EGLSurface;
	pub fn eglCreatePixmapSurface(
		display: EGLDisplay,
		config: EGLConfig,
		pixmap: NativePixmapType,
		attrib_list: *const Int
	) -> EGLSurface;
	pub fn eglCreateWindowSurface(
		display: EGLDisplay,
		config: EGLConfig,
		win: NativeWindowType,
		attrib_list: *const Int
	) -> EGLSurface;
	pub fn eglDestroyContext(display: EGLDisplay, ctx: EGLContext) -> Boolean;
	pub fn eglDestroySurface(display: EGLDisplay, surface: EGLSurface) -> Boolean;
	pub fn eglGetConfigAttrib(
		display: EGLDisplay,
		config: EGLConfig,
		attribute: Int,
		value: *mut Int
	) -> Boolean;
	pub fn eglGetConfigs(
		display: EGLDisplay,
		configs: *mut EGLConfig,
		config_size: Int,
		num_config: *mut Int
	) -> Boolean;
	pub fn eglGetCurrentDisplay() -> EGLDisplay;
	pub fn eglGetCurrentSurface(readdraw: Int) -> EGLSurface;
	pub fn eglGetDisplay(display_id: NativeDisplayType) -> EGLDisplay;
	pub fn eglGetError() -> Int;
	pub fn eglGetProcAddress(procname: *const c_char) -> extern "C" fn();
	pub fn eglInitialize(display: EGLDisplay, major: *mut Int, minor: *mut Int) -> Boolean;
	pub fn eglMakeCurrent(
		display: EGLDisplay,
		draw: EGLSurface,
		read: EGLSurface,
		ctx: EGLContext
	) -> Boolean;
	pub fn eglQueryContext(
		display: EGLDisplay,
		ctx: EGLContext,
		attribute: Int,
		value: *mut Int
	) -> Boolean;
	pub fn eglQueryString(display: EGLDisplay, name: Int) -> *const c_char;
	pub fn eglQuerySurface(
		display: EGLDisplay,
		surface: EGLSurface,
		attribute: Int,
		value: *mut Int
	) -> Boolean;
	pub fn eglSwapBuffers(display: EGLDisplay, surface: EGLSurface) -> Boolean;
	pub fn eglTerminate(display: EGLDisplay) -> Boolean;
	pub fn eglWaitGL() -> Boolean;
	pub fn eglWaitNative(engine: Int) -> Boolean;

	// EGL 1.1
	pub fn eglBindTexImage(display: EGLDisplay, surface: EGLSurface, buffer: Int) -> Boolean;
	pub fn eglReleaseTexImage(display: EGLDisplay, surface: EGLSurface, buffer: Int) -> Boolean;
	pub fn eglSurfaceAttrib(
		display: EGLDisplay,
		surface: EGLSurface,
		attribute: Int,
		value: Int
	) -> Boolean;
	pub fn eglSwapInterval(display: EGLDisplay, interval: Int) -> Boolean;

	// EGL 1.2
	pub fn eglBindAPI(api: Enum) -> Boolean;
	pub fn eglQueryAPI() -> Enum;
	pub fn eglCreatePbufferFromClientBuffer(
		display: EGLDisplay,
		buftype: Enum,
		buffer: EGLClientBuffer,
		config: EGLConfig,
		attrib_list: *const Int
	) -> EGLSurface;
	pub fn eglReleaseThread() -> Boolean;
	pub fn eglWaitClient() -> Boolean;

	// EGL 1.4
	pub fn eglGetCurrentContext() -> EGLContext;

	// EGL 1.5
	pub fn eglCreateSync(display: EGLDisplay, type_: Enum, attrib_list: *const Attrib) -> EGLSync;
	pub fn eglDestroySync(display: EGLDisplay, sync: EGLSync) -> Boolean;
	pub fn eglClientWaitSync(display: EGLDisplay, sync: EGLSync, flags: Int, timeout: Time) -> Int;
	pub fn eglGetSyncAttrib(
		display: EGLDisplay,
		sync: EGLSync,
		attribute: Int,
		value: *mut Attrib
	) -> Boolean;
	pub fn eglCreateImage(
		display: EGLDisplay,
		ctx: EGLContext,
		target: Enum,
		buffer: EGLClientBuffer,
		attrib_list: *const Attrib
	) -> EGLImage;
	pub fn eglDestroyImage(display: EGLDisplay, image: EGLImage) -> Boolean;
	pub fn eglGetPlatformDisplay(
		platform: Enum,
		native_display: *mut c_void,
		attrib_list: *const Attrib
	) -> EGLDisplay;
	pub fn eglCreatePlatformWindowSurface(
		display: EGLDisplay,
		config: EGLConfig,
		native_window: *mut c_void,
		attrib_list: *const Attrib
	) -> EGLSurface;
	pub fn eglCreatePlatformPixmapSurface(
		display: EGLDisplay,
		config: EGLConfig,
		native_pixmap: *mut c_void,
		attrib_list: *const Attrib
	) -> EGLSurface;
	pub fn eglWaitSync(display: EGLDisplay, sync: EGLSync, flags: Int) -> Boolean
}