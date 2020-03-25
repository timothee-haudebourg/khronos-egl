#![allow(non_upper_case_globals)]

// -------------------------------------------------------------------------------------------------
// DEPENDENCIES
// -------------------------------------------------------------------------------------------------

extern crate khronos;
extern crate libc;

// rust
use std::fmt::{self, Display};
use std::convert::{TryFrom, TryInto};
use std::ffi::CStr;
use std::ffi::CString;

// system
use khronos::{ khronos_int32_t,
			   khronos_utime_nanoseconds_t };

use libc::{ c_uint,
			c_void };

// -------------------------------------------------------------------------------------------------
// LINKING
// -------------------------------------------------------------------------------------------------

#[link(name = "EGL")]
extern {}

// ------------------------------------------------------------------------------------------------
// EGL 1.0
// ------------------------------------------------------------------------------------------------

pub type EGLBoolean		   = c_uint;
pub type EGLint			   = khronos_int32_t;
pub type EGLDisplay		   = *mut c_void;
pub type EGLConfig			= *mut c_void;
pub type EGLContext		   = *mut c_void;
pub type EGLSurface		   = *mut c_void;
pub type EGLNativeDisplayType = *mut c_void;

#[cfg(not(android))]
pub type EGLNativePixmapType = *mut c_void;

#[cfg(not(android))]
pub type EGLNativeWindowType = *mut c_void;

#[repr(C)]
#[cfg(android)]
struct android_native_window_t;

#[repr(C)]
#[cfg(android)]
struct egl_native_pixmap_t;

#[cfg(android)]
pub type EGLNativePixmapType = *mut egl_native_pixmap_t;

#[cfg(android)]
pub type EGLNativeWindowType = *mut android_native_window_t;

pub const EGL_ALPHA_SIZE: EGLint = 0x3021;
pub const EGL_BAD_ACCESS: EGLint = 0x3002;
pub const EGL_BAD_ALLOC: EGLint = 0x3003;
pub const EGL_BAD_ATTRIBUTE: EGLint = 0x3004;
pub const EGL_BAD_CONFIG: EGLint = 0x3005;
pub const EGL_BAD_CONTEXT: EGLint = 0x3006;
pub const EGL_BAD_CURRENT_SURFACE: EGLint = 0x3007;
pub const EGL_BAD_DISPLAY: EGLint = 0x3008;
pub const EGL_BAD_MATCH: EGLint = 0x3009;
pub const EGL_BAD_NATIVE_PIXMAP: EGLint = 0x300A;
pub const EGL_BAD_NATIVE_WINDOW: EGLint = 0x300B;
pub const EGL_BAD_PARAMETER: EGLint = 0x300C;
pub const EGL_BAD_SURFACE: EGLint = 0x300D;
pub const EGL_BLUE_SIZE: EGLint = 0x3022;
pub const EGL_BUFFER_SIZE: EGLint = 0x3020;
pub const EGL_CONFIG_CAVEAT: EGLint = 0x3027;
pub const EGL_CONFIG_ID: EGLint = 0x3028;
pub const EGL_CORE_NATIVE_ENGINE: EGLint = 0x305B;
pub const EGL_DEPTH_SIZE: EGLint = 0x3025;
pub const EGL_DONT_CARE: EGLint = -1;
pub const EGL_DRAW: EGLint = 0x3059;
pub const EGL_EXTENSIONS: EGLint = 0x3055;
pub const EGL_FALSE: EGLBoolean = 0;
pub const EGL_GREEN_SIZE: EGLint = 0x3023;
pub const EGL_HEIGHT: EGLint = 0x3056;
pub const EGL_LARGEST_PBUFFER: EGLint = 0x3058;
pub const EGL_LEVEL: EGLint = 0x3029;
pub const EGL_MAX_PBUFFER_HEIGHT: EGLint = 0x302A;
pub const EGL_MAX_PBUFFER_PIXELS: EGLint = 0x302B;
pub const EGL_MAX_PBUFFER_WIDTH: EGLint = 0x302C;
pub const EGL_NATIVE_RENDERABLE: EGLint = 0x302D;
pub const EGL_NATIVE_VISUAL_ID: EGLint = 0x302E;
pub const EGL_NATIVE_VISUAL_TYPE: EGLint = 0x302F;
pub const EGL_NONE: EGLint = 0x3038;
pub const EGL_NON_CONFORMANT_CONFIG: EGLint = 0x3051;
pub const EGL_NOT_INITIALIZED: EGLint = 0x3001;
pub const EGL_NO_CONTEXT: EGLContext = 0 as EGLContext;
pub const EGL_NO_DISPLAY: EGLDisplay = 0 as EGLDisplay;
pub const EGL_NO_SURFACE: EGLSurface = 0 as EGLSurface;
pub const EGL_PBUFFER_BIT: EGLint = 0x0001;
pub const EGL_PIXMAP_BIT: EGLint = 0x0002;
pub const EGL_READ: EGLint = 0x305A;
pub const EGL_RED_SIZE: EGLint = 0x3024;
pub const EGL_SAMPLES: EGLint = 0x3031;
pub const EGL_SAMPLE_BUFFERS: EGLint = 0x3032;
pub const EGL_SLOW_CONFIG: EGLint = 0x3050;
pub const EGL_STENCIL_SIZE: EGLint = 0x3026;
pub const EGL_SUCCESS: EGLint = 0x3000;
pub const EGL_SURFACE_TYPE: EGLint = 0x3033;
pub const EGL_TRANSPARENT_BLUE_VALUE: EGLint = 0x3035;
pub const EGL_TRANSPARENT_GREEN_VALUE: EGLint = 0x3036;
pub const EGL_TRANSPARENT_RED_VALUE: EGLint = 0x3037;
pub const EGL_TRANSPARENT_RGB: EGLint = 0x3052;
pub const EGL_TRANSPARENT_TYPE: EGLint = 0x3034;
pub const EGL_TRUE: EGLBoolean = 1;
pub const EGL_VENDOR: EGLint = 0x3053;
pub const EGL_VERSION: EGLint = 0x3054;
pub const EGL_WIDTH: EGLint = 0x3057;
pub const EGL_WINDOW_BIT: EGLint = 0x0004;

/// EGL errors.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Error {
	/// EGL is not initialized, or could not be initialized, for the specified EGL display connection.
	NotInitialized,

	/// EGL cannot access a requested resource (for example a context is bound in another thread).
	BadAccess,

	/// EGL failed to allocate resources for the requested operation.
	BadAlloc,

	/// An unrecognized attribute or attribute value was passed in the attribute list.
	BadAttribute,

	/// An EGLContext argument does not name a valid EGL rendering context.
	BadContext,

	/// An EGLConfig argument does not name a valid EGL frame buffer configuration.
	BadConfig,

	/// The current surface of the calling thread is a window, pixel buffer or pixmap that is no longer valid.
	BadCurrentSurface,

	/// An EGLDisplay argument does not name a valid EGL display connection.
	BadDisplay,

	/// An EGLSurface argument does not name a valid surface (window, pixel buffer or pixmap) configured for GL rendering.
	BadSurface,

	/// Arguments are inconsistent (for example, a valid context requires buffers not supplied by a valid surface).
	BadMatch,

	/// One or more argument values are invalid.
	BadParameter,

	/// A NativePixmapType argument does not refer to a valid native pixmap.
	BadNativePixmap,

	/// A NativeWindowType argument does not refer to a valid native window.
	BadNativeWindow,

	/// A power management event has occurred. The application must destroy all contexts and reinitialise OpenGL ES state and objects to continue rendering.
	ContextLost
}

impl Error {
	pub fn native(&self) -> EGLint {
		use Error::*;
		match self {
			NotInitialized => EGL_NOT_INITIALIZED,
			BadAccess => EGL_BAD_ACCESS,
			BadAlloc => EGL_BAD_ALLOC,
			BadAttribute => EGL_BAD_ATTRIBUTE,
			BadContext => EGL_BAD_CONTEXT,
			BadConfig => EGL_BAD_CONFIG,
			BadCurrentSurface => EGL_BAD_CURRENT_SURFACE,
			BadDisplay => EGL_BAD_DISPLAY,
			BadSurface => EGL_BAD_SURFACE,
			BadMatch => EGL_BAD_MATCH,
			BadParameter => EGL_BAD_PARAMETER,
			BadNativePixmap => EGL_BAD_NATIVE_PIXMAP,
			BadNativeWindow => EGL_BAD_NATIVE_WINDOW,
			ContextLost => EGL_CONTEXT_LOST
		}
	}

	fn message(&self) -> &'static str {
		use Error::*;
		match self {
			NotInitialized => "EGL is not initialized, or could not be initialized, for the specified EGL display connection.",
			BadAccess => "EGL cannot access a requested resource (for example a context is bound in another thread.",
			BadAlloc => "EGL failed to allocate resources for the requested operation.",
			BadAttribute => "An unrecognized attribute or attribute value was passed in the attribute list.",
			BadContext => "An EGLContext argument does not name a valid EGL rendering context.",
			BadConfig => "An EGLConfig argument does not name a valid EGL frame buffer configuration.",
			BadCurrentSurface => "The current surface of the calling thread is a window, pixel buffer or pixmap that is no longer valid.",
			BadDisplay => "An EGLDisplay argument does not name a valid EGL display connection.",
			BadSurface => "An EGLSurface argument does not name a valid surface (window, pixel buffer or pixmap) configured for GL rendering.",
			BadMatch => "Arguments are inconsistent (for example, a valid context requires buffers not supplied by a valid surface.",
			BadParameter => "One or more argument values are invalid.",
			BadNativePixmap => "A NativePixmapType argument does not refer to a valid native pixmap.",
			BadNativeWindow => "A NativeWindowType argument does not refer to a valid native window.",
			ContextLost => "A power management event has occurred. The application must destroy all contexts and reinitialise OpenGL ES state and objects to continue rendering."
		}
	}
}

impl From<Error> for EGLint {
	fn from(e: Error) -> EGLint {
		e.native()
	}
}

impl TryFrom<EGLint> for Error {
	type Error = EGLint;

	fn try_from(e: EGLint) -> Result<Error, EGLint> {
		use Error::*;
		match e {
			EGL_NOT_INITIALIZED => Ok(NotInitialized),
			EGL_BAD_ACCESS => Ok(BadAccess),
			EGL_BAD_ALLOC => Ok(BadAlloc),
			EGL_BAD_ATTRIBUTE => Ok(BadAttribute),
			EGL_BAD_CONTEXT => Ok(BadContext),
			EGL_BAD_CONFIG => Ok(BadConfig),
			EGL_BAD_CURRENT_SURFACE => Ok(BadCurrentSurface),
			EGL_BAD_DISPLAY => Ok(BadDisplay),
			EGL_BAD_SURFACE => Ok(BadSurface),
			EGL_BAD_MATCH => Ok(BadMatch),
			EGL_BAD_PARAMETER => Ok(BadParameter),
			EGL_BAD_NATIVE_PIXMAP => Ok(BadNativePixmap),
			EGL_BAD_NATIVE_WINDOW => Ok(BadNativeWindow),
			EGL_CONTEXT_LOST => Ok(ContextLost),
			_ => Err(e)
		}
	}
}

impl Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		self.message().fmt(f)
	}
}

/// Return a list of EGL frame buffer configurations that match specified attributes.
pub fn choose_config(display: EGLDisplay, attrib_list: &[EGLint], configs: &mut Vec<EGLConfig>) -> Result<(), Error> {
	unsafe {
		let capacity = configs.capacity();
		let mut count = 0;

		if ffi::eglChooseConfig(display, attrib_list.as_ptr(), configs.as_mut_ptr(), capacity.try_into().unwrap(), &mut count) == EGL_TRUE {
			configs.set_len(count as usize);
			Ok(())
		} else {
			Err(get_error().unwrap())
		}
	}
}

/// Copy EGL surface color buffer to a native pixmap.
pub fn copy_buffers(display: EGLDisplay, surface: EGLSurface, target: EGLNativePixmapType) -> Result<(), Error> {
	unsafe {
		if ffi::eglCopyBuffers(display, surface, target) == EGL_TRUE {
			Ok(())
		} else {
			Err(get_error().unwrap())
		}
	}
}

/// Create a new EGL rendering context.
pub fn create_context(display: EGLDisplay, config: EGLConfig, share_context: EGLContext, attrib_list: &[EGLint]) -> Result<EGLContext, Error> {
	unsafe {
		let context = ffi::eglCreateContext(display, config, share_context, attrib_list.as_ptr());

		if context != EGL_NO_CONTEXT {
			Ok(context)
		} else {
			Err(get_error().unwrap())
		}
	}
}

/// Create a new EGL pixel buffer surface.
pub fn create_pbuffer_surface(display: EGLDisplay, config: EGLConfig, attrib_list: &[EGLint]) -> Result<EGLSurface, Error> {
	unsafe {
		let surface = ffi::eglCreatePbufferSurface(display, config, attrib_list.as_ptr());

		if surface != EGL_NO_SURFACE {
			Ok(surface)
		} else {
			Err(get_error().unwrap())
		}
	}
}

/// Create a new EGL offscreen surface.
pub fn create_pixmap_surface(display: EGLDisplay, config: EGLConfig, pixmap: EGLNativePixmapType, attrib_list: &[EGLint]) -> Result<EGLSurface, Error> {
	unsafe {
		let surface = ffi::eglCreatePixmapSurface(display, config, pixmap, attrib_list.as_ptr());

		if surface != EGL_NO_SURFACE {
			Ok(surface)
		} else {
			Err(get_error().unwrap())
		}
	}
}

/// Create a new EGL window surface.
pub fn create_window_surface(display: EGLDisplay, config: EGLConfig, window: EGLNativeWindowType, attrib_list: &[EGLint]) -> Result<EGLSurface, Error> {
	unsafe {
		let surface = ffi::eglCreateWindowSurface(display, config, window, attrib_list.as_ptr());

		if surface != EGL_NO_SURFACE {
			Ok(surface)
		} else {
			Err(get_error().unwrap())
		}
	}
}

/// Destroy an EGL rendering context.
pub fn destroy_context(display: EGLDisplay, ctx: EGLContext) -> Result<(), Error> {
	unsafe {
		if ffi::eglDestroyContext(display, ctx) == EGL_TRUE {
			Ok(())
		} else {
			Err(get_error().unwrap())
		}
	}
}

/// Destroy an EGL surface.
pub fn destroy_surface(display: EGLDisplay, surface: EGLSurface) -> Result<(), Error> {
	unsafe {
		if ffi::eglDestroySurface(display, surface) == EGL_TRUE {
			Ok(())
		} else {
			Err(get_error().unwrap())
		}
	}
}

/// Return information about an EGL frame buffer configuration.
pub fn get_config_attrib(display: EGLDisplay, config: EGLConfig, attribute: EGLint) -> Result<EGLint, Error> {
	unsafe {
		let mut value: EGLint = 0;
		if ffi::eglGetConfigAttrib(display, config, attribute, &mut value) == EGL_TRUE {
			Ok(value)
		} else {
			Err(get_error().unwrap())
		}
	}
}

/// Return a list of all EGL frame buffer configurations for a display.
pub fn get_configs(display: EGLDisplay, configs: &mut Vec<EGLConfig>) -> Result<(), Error> {
	unsafe {
		let capacity = configs.capacity();
		let mut count = 0;

		if ffi::eglGetConfigs(display, configs.as_mut_ptr(), capacity.try_into().unwrap(), &mut count) == EGL_TRUE {
			configs.set_len(count as usize);
			Ok(())
		} else {
			Err(get_error().unwrap())
		}
	}
}

/// Return the display for the current EGL rendering context.
pub fn get_current_display() -> Option<EGLDisplay> {
	unsafe {
		let display = ffi::eglGetCurrentDisplay();

		if display != EGL_NO_DISPLAY {
			Some(display)
		} else {
			None
		}
	}
}

/// Return the read or draw surface for the current EGL rendering context.
pub fn get_current_surface(readdraw: EGLint) -> Option<EGLSurface> {
	unsafe {
		let surface = ffi::eglGetCurrentSurface(readdraw);

		if surface != EGL_NO_SURFACE {
			Some(surface)
		} else {
			None
		}
	}
}

/// Return an EGL display connection.
pub fn get_display(display_id: EGLNativeDisplayType) -> Option<EGLDisplay> {
	unsafe {
		let display = ffi::eglGetDisplay(display_id);

		if display != EGL_NO_DISPLAY {
			Some(display)
		} else {
			None
		}
	}
}

/// Return error information.
///
/// Return the error of the last called EGL function in the current thread, or `None` if the
/// error is set to `EGL_SUCCESS`.
///
/// Note that since a call to `eglGetError` sets the error to `EGL_SUCCESS`, and since this
/// function is automatically called by any wrapper function returning a `Result` when necessary,
/// this function may only return `None` from the point of view of a user.
pub fn get_error() -> Option<Error> {
	unsafe {
		let e = ffi::eglGetError();
		if e == EGL_SUCCESS {
			None
		} else {
			Some(e.try_into().unwrap())
		}
	}
}

/// Return a GL or an EGL extension function.
pub fn get_proc_address(procname: &str) -> Option<extern "C" fn()> {
	unsafe {
		let string = CString::new(procname).unwrap();

		let addr = ffi::eglGetProcAddress(string.as_ptr());
		if !(addr as *const ()).is_null() {
			Some(addr)
		} else {
			None
		}
	}
}

/// Initialize an EGL display connection.
pub fn initialize(display: EGLDisplay) -> Result<(EGLint, EGLint), Error> {
	unsafe {
		let mut major = 0;
		let mut minor = 0;

		if ffi::eglInitialize(display, &mut major, &mut minor) == EGL_TRUE {
			Ok((major, minor))
		} else {
			Err(get_error().unwrap())
		}
	}
}

/// Attach an EGL rendering context to EGL surfaces.
pub fn make_current(display: EGLDisplay, draw: Option<EGLSurface>, read: Option<EGLSurface>, ctx: Option<EGLContext>) -> Result<(), Error> {
	unsafe {
		let draw = match draw {
			Some(draw) => draw,
			None => EGL_NO_SURFACE
		};
		let read = match read {
			Some(read) => read,
			None => EGL_NO_SURFACE
		};
		let ctx = match ctx {
			Some(ctx) => ctx,
			None => EGL_NO_CONTEXT
		};

		if ffi::eglMakeCurrent(display, draw, read, ctx) == EGL_TRUE {
			Ok(())
		} else {
			Err(get_error().unwrap())
		}
	}
}

/// Return EGL rendering context information.
pub fn query_context(display: EGLDisplay, ctx: EGLContext, attribute: EGLint) -> Result<EGLint, Error> {
	unsafe {
		let mut value = 0;
		if ffi::eglQueryContext(display, ctx, attribute, &mut value) == EGL_TRUE {
			Ok(value)
		} else {
			Err(get_error().unwrap())
		}
	}
}

/// Return a string describing properties of the EGL client or of an EGL display connection.
pub fn query_string(display: EGLDisplay, name: EGLint) -> Result<&'static CStr, Error> {
	unsafe {
		let c_str = ffi::eglQueryString(display, name);

		if !c_str.is_null() {
			Ok(CStr::from_ptr(c_str))
		} else {
			Err(get_error().unwrap())
		}
	}
}

/// Return EGL surface information.
pub fn query_surface(display: EGLDisplay, surface: EGLSurface, attribute: EGLint) -> Result<EGLint, Error> {
	unsafe {
		let mut value = 0;
		if ffi::eglQuerySurface(display, surface, attribute, &mut value) == EGL_TRUE {
			Ok(value)
		} else {
			Err(get_error().unwrap())
		}
	}
}

/// Post EGL surface color buffer to a native window.
pub fn swap_buffers(display: EGLDisplay, surface: EGLSurface) -> Result<(), Error> {
	unsafe {
		if ffi::eglSwapBuffers(display, surface) == EGL_TRUE {
			Ok(())
		} else {
			Err(get_error().unwrap())
		}
	}
}

/// Terminate an EGL display connection.
pub fn terminate(display: EGLDisplay) -> Result<(), Error> {
	unsafe {
		if ffi::eglTerminate(display) == EGL_TRUE {
			Ok(())
		} else {
			Err(get_error().unwrap())
		}
	}
}

/// Complete GL execution prior to subsequent native rendering calls.
pub fn wait_gl() -> Result<(), Error> {
	unsafe {
		if ffi::eglWaitGL() == EGL_TRUE {
			Ok(())
		} else {
			Err(get_error().unwrap())
		}
	}
}

/// Complete native execution prior to subsequent GL rendering calls.
pub fn wait_native(engine: EGLint) -> Result<(), Error> {
	unsafe {
		if ffi::eglWaitNative(engine) == EGL_TRUE {
			Ok(())
		} else {
			Err(get_error().unwrap())
		}
	}
}

// ------------------------------------------------------------------------------------------------
// EGL 1.1
// ------------------------------------------------------------------------------------------------

pub const EGL_BACK_BUFFER: EGLint = 0x3084;
pub const EGL_BIND_TO_TEXTURE_RGB: EGLint = 0x3039;
pub const EGL_BIND_TO_TEXTURE_RGBA: EGLint = 0x303A;
pub const EGL_CONTEXT_LOST: EGLint = 0x300E;
pub const EGL_MIN_SWAP_INTERVAL: EGLint = 0x303B;
pub const EGL_MAX_SWAP_INTERVAL: EGLint = 0x303C;
pub const EGL_MIPMAP_TEXTURE: EGLint = 0x3082;
pub const EGL_MIPMAP_LEVEL: EGLint = 0x3083;
pub const EGL_NO_TEXTURE: EGLint = 0x305C;
pub const EGL_TEXTURE_2D: EGLint = 0x305F;
pub const EGL_TEXTURE_FORMAT: EGLint = 0x3080;
pub const EGL_TEXTURE_RGB: EGLint = 0x305D;
pub const EGL_TEXTURE_RGBA: EGLint = 0x305E;
pub const EGL_TEXTURE_TARGET: EGLint = 0x3081;

/// Defines a two-dimensional texture image.
pub fn bind_tex_image(display: EGLDisplay, surface: EGLSurface, buffer: EGLint) -> Result<(), Error> {
	unsafe {
		if ffi::eglBindTexImage(display, surface, buffer) == EGL_TRUE {
			Ok(())
		} else {
			Err(get_error().unwrap())
		}
	}
}

///  Releases a color buffer that is being used as a texture.
pub fn release_tex_image(display: EGLDisplay, surface: EGLSurface, buffer: EGLint) -> Result<(), Error> {
	unsafe {
		if ffi::eglReleaseTexImage(display, surface, buffer) == EGL_TRUE {
			Ok(())
		} else {
			Err(get_error().unwrap())
		}
	}
}

/// Set an EGL surface attribute.
pub fn surface_attrib(display: EGLDisplay, surface: EGLSurface, attribute: EGLint, value: EGLint) -> Result<(), Error> {
	unsafe {
		if ffi::eglSurfaceAttrib(display, surface, attribute, value) == EGL_TRUE {
			Ok(())
		} else {
			Err(get_error().unwrap())
		}
	}
}

/// Specifies the minimum number of video frame periods per buffer swap for the window associated with the current context.
pub fn swap_interval(display: EGLDisplay, interval: EGLint) -> Result<(), Error> {
	unsafe {
		if ffi::eglSwapInterval(display, interval) == EGL_TRUE {
			Ok(())
		} else {
			Err(get_error().unwrap())
		}
	}
}

// ------------------------------------------------------------------------------------------------
// EGL 1.2
// ------------------------------------------------------------------------------------------------

pub type EGLenum			  = c_uint;
pub type EGLClientBuffer	  = *mut c_void;

pub const EGL_ALPHA_FORMAT: EGLint = 0x3088;
pub const EGL_ALPHA_FORMAT_NONPRE: EGLint = 0x308B;
pub const EGL_ALPHA_FORMAT_PRE: EGLint = 0x308C;
pub const EGL_ALPHA_MASK_SIZE: EGLint = 0x303E;
pub const EGL_BUFFER_PRESERVED: EGLint = 0x3094;
pub const EGL_BUFFER_DESTROYED: EGLint = 0x3095;
pub const EGL_CLIENT_APIS: EGLint = 0x308D;
pub const EGL_COLORSPACE: EGLint = 0x3087;
pub const EGL_COLORSPACE_sRGB: EGLint = 0x3089;
pub const EGL_COLORSPACE_LINEAR: EGLint = 0x308A;
pub const EGL_COLOR_BUFFER_TYPE: EGLint = 0x303F;
pub const EGL_CONTEXT_CLIENT_TYPE: EGLint = 0x3097;
pub const EGL_DISPLAY_SCALING: EGLint = 10000;
pub const EGL_HORIZONTAL_RESOLUTION: EGLint = 0x3090;
pub const EGL_LUMINANCE_BUFFER: EGLint = 0x308F;
pub const EGL_LUMINANCE_SIZE: EGLint = 0x303D;
pub const EGL_OPENGL_ES_BIT: EGLint = 0x0001;
pub const EGL_OPENVG_BIT: EGLint = 0x0002;
pub const EGL_OPENGL_ES_API: EGLenum = 0x30A0;
pub const EGL_OPENVG_API: EGLenum = 0x30A1;
pub const EGL_OPENVG_IMAGE: EGLint = 0x3096;
pub const EGL_PIXEL_ASPECT_RATIO: EGLint = 0x3092;
pub const EGL_RENDERABLE_TYPE: EGLint = 0x3040;
pub const EGL_RENDER_BUFFER: EGLint = 0x3086;
pub const EGL_RGB_BUFFER: EGLint = 0x308E;
pub const EGL_SINGLE_BUFFER: EGLint = 0x3085;
pub const EGL_SWAP_BEHAVIOR: EGLint = 0x3093;
pub const EGL_UNKNOWN: EGLint = -1;
pub const EGL_VERTICAL_RESOLUTION: EGLint = 0x3091;

/// Set the current rendering API.
pub fn bind_api(api: EGLenum) -> Result<(), Error> {
	unsafe {
		if ffi::eglBindAPI(api) == EGL_TRUE {
			Ok(())
		} else {
			Err(get_error().unwrap())
		}
	}
}

/// Query the current rendering API.
pub fn query_api() -> EGLenum {
	unsafe {
		ffi::eglQueryAPI()
	}
}

/// Create a new EGL pixel buffer surface bound to an OpenVG image.
pub fn create_pbuffer_from_client_buffer(display: EGLDisplay, buffer_type: EGLenum, buffer: EGLClientBuffer, config: EGLConfig, attrib_list: &[EGLint]) -> Result<EGLSurface, Error> {
	unsafe {
		let surface = ffi::eglCreatePbufferFromClientBuffer(display, buffer_type, buffer, config, attrib_list.as_ptr());

		if surface != EGL_NO_SURFACE {
			Ok(surface)
		} else {
			Err(get_error().unwrap())
		}
	}
}

/// Release EGL per-thread state.
pub fn release_thread() -> Result<(), Error> {
	unsafe {
		if ffi::eglReleaseThread() == EGL_TRUE {
			Ok(())
		} else {
			Err(get_error().unwrap())
		}
	}
}

/// Complete client API execution prior to subsequent native rendering calls.
pub fn wait_client() -> Result<(), Error> {
	unsafe {
		if ffi::eglWaitClient() == EGL_TRUE {
			Ok(())
		} else {
			Err(get_error().unwrap())
		}
	}
}

// ------------------------------------------------------------------------------------------------
// EGL 1.3
// ------------------------------------------------------------------------------------------------

pub const EGL_CONFORMANT: EGLint = 0x3042;
pub const EGL_CONTEXT_CLIENT_VERSION: EGLint = 0x3098;
pub const EGL_MATCH_NATIVE_PIXMAP: EGLint = 0x3041;
pub const EGL_OPENGL_ES2_BIT: EGLint = 0x0004;
pub const EGL_VG_ALPHA_FORMAT: EGLint = 0x3088;
pub const EGL_VG_ALPHA_FORMAT_NONPRE: EGLint = 0x308B;
pub const EGL_VG_ALPHA_FORMAT_PRE: EGLint = 0x308C;
pub const EGL_VG_ALPHA_FORMAT_PRE_BIT: EGLint = 0x0040;
pub const EGL_VG_COLORSPACE: EGLint = 0x3087;
pub const EGL_VG_COLORSPACE_sRGB: EGLint = 0x3089;
pub const EGL_VG_COLORSPACE_LINEAR: EGLint = 0x308A;
pub const EGL_VG_COLORSPACE_LINEAR_BIT: EGLint = 0x0020;

// ------------------------------------------------------------------------------------------------
// EGL 1.4
// ------------------------------------------------------------------------------------------------

pub const EGL_DEFAULT_DISPLAY: EGLNativeDisplayType = 0 as EGLNativeDisplayType;
pub const EGL_MULTISAMPLE_RESOLVE_BOX_BIT: EGLint = 0x0200;
pub const EGL_MULTISAMPLE_RESOLVE: EGLint = 0x3099;
pub const EGL_MULTISAMPLE_RESOLVE_DEFAULT: EGLint = 0x309A;
pub const EGL_MULTISAMPLE_RESOLVE_BOX: EGLint = 0x309B;
pub const EGL_OPENGL_API: EGLenum = 0x30A2;
pub const EGL_OPENGL_BIT: EGLint = 0x0008;
pub const EGL_SWAP_BEHAVIOR_PRESERVED_BIT: EGLint = 0x0400;

/// Return the current EGL rendering context.
pub fn get_current_context() -> Option<EGLContext> {
	unsafe {
		let context = ffi::eglGetCurrentContext();

		if context != EGL_NO_CONTEXT {
			Some(context)
		} else {
			None
		}
	}
}

// ------------------------------------------------------------------------------------------------
// EGL 1.5
// ------------------------------------------------------------------------------------------------

pub type EGLSync			  = *mut c_void;
pub type EGLAttrib			= usize;
pub type EGLTime			  = khronos_utime_nanoseconds_t;
pub type EGLImage			 = *mut c_void;

pub const EGL_CONTEXT_MAJOR_VERSION: EGLint = 0x3098;
pub const EGL_CONTEXT_MINOR_VERSION: EGLint = 0x30FB;
pub const EGL_CONTEXT_OPENGL_PROFILE_MASK: EGLint = 0x30FD;
pub const EGL_CONTEXT_OPENGL_RESET_NOTIFICATION_STRATEGY: EGLint = 0x31BD;
pub const EGL_NO_RESET_NOTIFICATION: EGLint = 0x31BE;
pub const EGL_LOSE_CONTEXT_ON_RESET: EGLint = 0x31BF;
pub const EGL_CONTEXT_OPENGL_CORE_PROFILE_BIT: EGLint = 0x00000001;
pub const EGL_CONTEXT_OPENGL_COMPATIBILITY_PROFILE_BIT: EGLint = 0x00000002;
pub const EGL_CONTEXT_OPENGL_DEBUG: EGLint = 0x31B0;
pub const EGL_CONTEXT_OPENGL_FORWARD_COMPATIBLE: EGLint = 0x31B1;
pub const EGL_CONTEXT_OPENGL_ROBUST_ACCESS: EGLint = 0x31B2;
pub const EGL_OPENGL_ES3_BIT: EGLint = 0x00000040;
pub const EGL_CL_EVENT_HANDLE: EGLint = 0x309C;
pub const EGL_SYNC_CL_EVENT: EGLint = 0x30FE;
pub const EGL_SYNC_CL_EVENT_COMPLETE: EGLint = 0x30FF;
pub const EGL_SYNC_PRIOR_COMMANDS_COMPLETE: EGLint = 0x30F0;
pub const EGL_SYNC_TYPE: EGLint = 0x30F7;
pub const EGL_SYNC_STATUS: EGLint = 0x30F1;
pub const EGL_SYNC_CONDITION: EGLint = 0x30F8;
pub const EGL_SIGNALED: EGLint = 0x30F2;
pub const EGL_UNSIGNALED: EGLint = 0x30F3;
pub const EGL_SYNC_FLUSH_COMMANDS_BIT: EGLint = 0x0001;
pub const EGL_FOREVER: u64 = 0xFFFFFFFFFFFFFFFFu64;
pub const EGL_TIMEOUT_EXPIRED: EGLint = 0x30F5;
pub const EGL_CONDITION_SATISFIED: EGLint = 0x30F6;
pub const EGL_NO_SYNC: EGLSync = 0 as EGLSync;
pub const EGL_SYNC_FENCE: EGLint = 0x30F9;
pub const EGL_GL_COLORSPACE: EGLint = 0x309D;
pub const EGL_GL_COLORSPACE_SRGB: EGLint = 0x3089;
pub const EGL_GL_COLORSPACE_LINEAR: EGLint = 0x308A;
pub const EGL_GL_RENDERBUFFER: EGLint = 0x30B9;
pub const EGL_GL_TEXTURE_2D: EGLint = 0x30B1;
pub const EGL_GL_TEXTURE_LEVEL: EGLint = 0x30BC;
pub const EGL_GL_TEXTURE_3D: EGLint = 0x30B2;
pub const EGL_GL_TEXTURE_ZOFFSET: EGLint = 0x30BD;
pub const EGL_GL_TEXTURE_CUBE_MAP_POSITIVE_X: EGLint = 0x30B3;
pub const EGL_GL_TEXTURE_CUBE_MAP_NEGATIVE_X: EGLint = 0x30B4;
pub const EGL_GL_TEXTURE_CUBE_MAP_POSITIVE_Y: EGLint = 0x30B5;
pub const EGL_GL_TEXTURE_CUBE_MAP_NEGATIVE_Y: EGLint = 0x30B6;
pub const EGL_GL_TEXTURE_CUBE_MAP_POSITIVE_Z: EGLint = 0x30B7;
pub const EGL_GL_TEXTURE_CUBE_MAP_NEGATIVE_Z: EGLint = 0x30B8;
pub const EGL_IMAGE_PRESERVED: EGLint = 0x30D2;
pub const EGL_NO_IMAGE: EGLImage = 0 as EGLImage;

/// Create a new EGL sync object.
pub fn create_sync(dpy: EGLDisplay, ty: EGLenum, attrib_list: &[EGLAttrib]) -> Result<EGLSync, Error> {
	unsafe {
		let sync = ffi::eglCreateSync(dpy, ty, attrib_list.as_ptr());
		if sync != EGL_NO_SYNC {
			Ok(sync)
		} else {
			Err(get_error().unwrap())
		}
	}
}

/// Destroy a sync object.
pub fn destroy_sync(dpy: EGLDisplay, sync: EGLSync) -> Result<(), Error> {
	unsafe {
		if ffi::eglDestroySync(dpy, sync) == EGL_TRUE {
			Ok(())
		} else {
			Err(get_error().unwrap())
		}
	}
}

/// Wait in the client for a sync object to be signalled.
pub fn client_wait_sync(dpy: EGLDisplay, sync: EGLSync, flags: EGLint, timeout: EGLTime) -> Result<EGLint, Error> {
	unsafe {
		let status = ffi::eglClientWaitSync(dpy, sync, flags, timeout);
		if status != EGL_FALSE as EGLint {
			Ok(status)
		} else {
			Err(get_error().unwrap())
		}
	}
}

/// Return an attribute of a sync object.
pub fn get_sync_attrib(dpy: EGLDisplay, sync: EGLSync, attribute: EGLint) -> Result<EGLAttrib, Error> {
	unsafe {
		let mut value = 0;
		if ffi::eglGetSyncAttrib(dpy, sync, attribute, &mut value as *mut EGLAttrib) == EGL_TRUE {
			Ok(value)
		} else {
			Err(get_error().unwrap())
		}
	}
}

/// Create a new EGLImage object.
pub fn create_image(dpy: EGLDisplay, ctx: EGLContext, target: EGLenum, buffer: EGLClientBuffer, attrib_list: &[EGLAttrib]) -> Result<EGLImage, Error> {
	unsafe {
		let image = ffi::eglCreateImage(dpy, ctx, target, buffer, attrib_list.as_ptr());
		if image != EGL_NO_IMAGE {
			Ok(image)
		} else {
			Err(get_error().unwrap())
		}
	}
}

/// Destroy an EGLImage object.
pub fn destroy_image(dpy: EGLDisplay, image: EGLImage) -> Result<(), Error> {
	unsafe {
		if ffi::eglDestroyImage(dpy, image) == EGL_TRUE {
			Ok(())
		} else {
			Err(get_error().unwrap())
		}
	}
}

/// Return an EGL display connection.
pub fn get_platform_display(platform: EGLenum, native_display: *mut c_void, attrib_list: &[EGLAttrib]) -> Result<EGLDisplay, Error> {
	unsafe {
		let display = ffi::eglGetPlatformDisplay(platform, native_display, attrib_list.as_ptr());
		if display != EGL_NO_DISPLAY {
			Ok(display)
		} else {
			Err(get_error().unwrap())
		}
	}
}

/// Create a new EGL on-screen rendering surface.
pub fn create_platform_window_surface(dpy: EGLDisplay, config: EGLConfig, native_window: *mut c_void, attrib_list: &[EGLAttrib]) -> Result<EGLSurface, Error> {
	unsafe {
		let surface = ffi::eglCreatePlatformWindowSurface(dpy, config, native_window, attrib_list.as_ptr());
		if surface != EGL_NO_SURFACE {
			Ok(surface)
		} else {
			Err(get_error().unwrap())
		}
	}
}

/// Create a new EGL offscreen surface.
pub fn create_platform_pixmap_surface(dpy: EGLDisplay, config: EGLConfig, native_pixmap: *mut c_void, attrib_list: &[EGLAttrib]) -> Result<EGLSurface, Error> {
	unsafe {
		let surface = ffi::eglCreatePlatformPixmapSurface(dpy, config, native_pixmap, attrib_list.as_ptr());
		if surface != EGL_NO_SURFACE {
			Ok(surface)
		} else {
			Err(get_error().unwrap())
		}
	}
}

/// Wait in the server for a sync object to be signalled.
pub fn wait_sync(dpy: EGLDisplay, sync: EGLSync, flags: EGLint) -> Result<(), Error> {
	unsafe {
		if ffi::eglWaitSync(dpy, sync, flags) == EGL_TRUE {
			Ok(())
		} else {
			Err(get_error().unwrap())
		}
	}
}

// -------------------------------------------------------------------------------------------------
// FFI
// -------------------------------------------------------------------------------------------------

mod ffi {
	use libc::{ c_char,
				c_void };

	use super::{ EGLBoolean,
				 EGLClientBuffer,
				 EGLConfig,
				 EGLContext,
				 EGLDisplay,
				 EGLenum,
				 EGLint,
				 EGLNativeDisplayType,
				 EGLNativePixmapType,
				 EGLNativeWindowType,
				 EGLSurface,
				 EGLAttrib,
				 EGLSync,
				 EGLTime,
				 EGLImage };

	extern {
		// EGL 1.0
		pub fn eglChooseConfig(dpy: EGLDisplay, attrib_list: *const EGLint, configs: *mut EGLConfig, config_size: EGLint, num_config: *mut EGLint) -> EGLBoolean;
		pub fn eglCopyBuffers(dpy: EGLDisplay, surface: EGLSurface, target: EGLNativePixmapType) -> EGLBoolean;
		pub fn eglCreateContext(dpy: EGLDisplay, config: EGLConfig, share_context: EGLContext, attrib_list: *const EGLint) -> EGLContext;
		pub fn eglCreatePbufferSurface(dpy: EGLDisplay, config: EGLConfig, attrib_list: *const EGLint) -> EGLSurface;
		pub fn eglCreatePixmapSurface(dpy: EGLDisplay, config: EGLConfig, pixmap: EGLNativePixmapType, attrib_list: *const EGLint) -> EGLSurface;
		pub fn eglCreateWindowSurface(dpy: EGLDisplay, config: EGLConfig, win: EGLNativeWindowType, attrib_list: *const EGLint) -> EGLSurface;
		pub fn eglDestroyContext(dpy: EGLDisplay, ctx: EGLContext) -> EGLBoolean;
		pub fn eglDestroySurface(dpy: EGLDisplay, surface: EGLSurface) -> EGLBoolean;
		pub fn eglGetConfigAttrib(dpy: EGLDisplay, config: EGLConfig, attribute: EGLint, value: *mut EGLint) -> EGLBoolean;
		pub fn eglGetConfigs(dpy: EGLDisplay, configs: *mut EGLConfig, config_size: EGLint, num_config: *mut EGLint) -> EGLBoolean;
		pub fn eglGetCurrentDisplay() -> EGLDisplay;
		pub fn eglGetCurrentSurface(readdraw: EGLint) -> EGLSurface;
		pub fn eglGetDisplay(display_id: EGLNativeDisplayType) -> EGLDisplay;
		pub fn eglGetError() -> EGLint;
		pub fn eglGetProcAddress(procname: *const c_char) -> extern "C" fn();
		pub fn eglInitialize(dpy: EGLDisplay, major: *mut EGLint, minor: *mut EGLint) -> EGLBoolean;
		pub fn eglMakeCurrent(dpy: EGLDisplay, draw: EGLSurface, read: EGLSurface, ctx: EGLContext) -> EGLBoolean;
		pub fn eglQueryContext(dpy: EGLDisplay, ctx: EGLContext, attribute: EGLint, value: *mut EGLint) -> EGLBoolean;
		pub fn eglQueryString(dpy: EGLDisplay, name: EGLint) -> *const c_char;
		pub fn eglQuerySurface(dpy: EGLDisplay, surface: EGLSurface, attribute: EGLint, value: *mut EGLint) -> EGLBoolean;
		pub fn eglSwapBuffers(dpy: EGLDisplay, surface: EGLSurface) -> EGLBoolean;
		pub fn eglTerminate(dpy: EGLDisplay) -> EGLBoolean;
		pub fn eglWaitGL() -> EGLBoolean;
		pub fn eglWaitNative(engine: EGLint) -> EGLBoolean;

		// EGL 1.1
		pub fn eglBindTexImage(dpy: EGLDisplay, surface: EGLSurface, buffer: EGLint) -> EGLBoolean;
		pub fn eglReleaseTexImage(dpy: EGLDisplay, surface: EGLSurface, buffer: EGLint) -> EGLBoolean;
		pub fn eglSurfaceAttrib(dpy: EGLDisplay, surface: EGLSurface, attribute: EGLint, value: EGLint) -> EGLBoolean;
		pub fn eglSwapInterval(dpy: EGLDisplay, interval: EGLint) -> EGLBoolean;

		// EGL 1.2
		pub fn eglBindAPI(api: EGLenum) -> EGLBoolean;
		pub fn eglQueryAPI() -> EGLenum;
		pub fn eglCreatePbufferFromClientBuffer(dpy: EGLDisplay, buftype: EGLenum, buffer: EGLClientBuffer, config: EGLConfig, attrib_list: *const EGLint) -> EGLSurface;
		pub fn eglReleaseThread() -> EGLBoolean;
		pub fn eglWaitClient() -> EGLBoolean;

		// EGL 1.4
		pub fn eglGetCurrentContext() -> EGLContext;

		// EGL 1.5
		pub fn eglCreateSync(dpy: EGLDisplay, type_: EGLenum, attrib_list: *const EGLAttrib) -> EGLSync;
		pub fn eglDestroySync(dpy: EGLDisplay, sync: EGLSync) -> EGLBoolean;
		pub fn eglClientWaitSync(dpy: EGLDisplay, sync: EGLSync, flags: EGLint, timeout: EGLTime) -> EGLint;
		pub fn eglGetSyncAttrib(dpy: EGLDisplay, sync: EGLSync, attribute: EGLint, value: *mut EGLAttrib) -> EGLBoolean;
		pub fn eglCreateImage(dpy: EGLDisplay, ctx: EGLContext, target: EGLenum, buffer: EGLClientBuffer, attrib_list: *const EGLAttrib) -> EGLImage;
		pub fn eglDestroyImage(dpy: EGLDisplay, image: EGLImage) -> EGLBoolean;
		pub fn eglGetPlatformDisplay(platform: EGLenum, native_display: *mut c_void, attrib_list: *const EGLAttrib) -> EGLDisplay;
		pub fn eglCreatePlatformWindowSurface(dpy: EGLDisplay, config: EGLConfig, native_window: *mut c_void, attrib_list: *const EGLAttrib) -> EGLSurface;
		pub fn eglCreatePlatformPixmapSurface(dpy: EGLDisplay, config: EGLConfig, native_pixmap: *mut c_void, attrib_list: *const EGLAttrib) -> EGLSurface;
		pub fn eglWaitSync(dpy: EGLDisplay, sync: EGLSync, flags: EGLint) -> EGLBoolean;
	}
}
