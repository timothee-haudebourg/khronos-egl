// +-----------------------------------------------------------------------------------------------+
// | Copyright 2015 Sean Kerr                                                                      |
// |                                                                                               |
// | Licensed under the Apache License, Version 2.0 (the "License");                               |
// | you may not use this file except in compliance with the License.                              |
// | You may obtain a copy of the License Author                                                   |
// |                                                                                               |
// |  http://www.apache.org/licenses/LICENSE-2.0                                                   |
// |                                                                                               |
// | Unless required by applicable law or agreed to in writing, software                           |
// | distributed under the License is distributed on an "AS IS" BASIS,                             |
// | WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.                      |
// | See the License for the specific language governing permissions and                           |
// | limitations under the License.                                                                |
// +-----------------------------------------------------------------------------------------------+
// | Author: Sean Kerr <sean@code-box.org>                                                         |
// +-----------------------------------------------------------------------------------------------+

#![crate_name = "egl"]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]

// -------------------------------------------------------------------------------------------------
// DEPENDENCIES
// -------------------------------------------------------------------------------------------------

extern crate khronos;
extern crate libc;

// rust
use std::ffi::CStr;
use std::ffi::CString;
use std::ptr;

// system
use khronos::khronos_int32_t;

use libc::{ c_void,
            int32_t,
            uint32_t };

// -------------------------------------------------------------------------------------------------
// LINKING
// -------------------------------------------------------------------------------------------------

#[link(name = "EGL")]
extern {}

// -------------------------------------------------------------------------------------------------
// GLOBAL TYPES
// -------------------------------------------------------------------------------------------------

pub type EGLBoolean           = uint32_t;
pub type EGLClientBuffer      = *mut c_void;
pub type EGLConfig            = *mut c_void;
pub type EGLContext           = *mut c_void;
pub type EGLDisplay           = *mut c_void;
pub type EGLenum              = uint32_t;
pub type EGLint               = khronos_int32_t;
pub type EGLNativeDisplayType = *mut c_void;
pub type EGLSurface           = *mut c_void;

// -------------------------------------------------------------------------------------------------
// ANDROID TYPES
// -------------------------------------------------------------------------------------------------

#[repr(C)]
#[cfg(android)]
enum android_native_window_t {}

#[repr(C)]
#[cfg(android)]
enum egl_native_pixmap_t {}

#[cfg(android)]
pub type EGLNativePixmapType = *mut egl_native_pixmap_t;

#[cfg(android)]
pub type EGLNativeWindowType = *mut android_native_window_t;

// -------------------------------------------------------------------------------------------------
// NON-ANDROID TYPES
// -------------------------------------------------------------------------------------------------

#[cfg(not(android))]
pub type EGLNativePixmapType = *mut c_void;

#[cfg(not(android))]
pub type EGLNativeWindowType = *mut c_void;

// -------------------------------------------------------------------------------------------------
// STRUCTS
// -------------------------------------------------------------------------------------------------

pub struct EGLConfigList {
    pub configs: EGLConfig,
    pub count:   int32_t
}

// -------------------------------------------------------------------------------------------------
// CONSTANTS
// -------------------------------------------------------------------------------------------------

// EGL aliases
pub const FALSE: EGLBoolean = 0;
pub const TRUE:  EGLBoolean = 1;

// out-of-band handle values
pub const DEFAULT_DISPLAY: EGLNativeDisplayType = 0 as *mut c_void;
pub const NO_CONTEXT:      EGLContext = 0 as *mut c_void;
pub const NO_DISPLAY:      EGLDisplay = 0 as *mut c_void;
pub const NO_SURFACE:      EGLSurface = 0 as *mut c_void;

// out-of-band attribute value
pub const DONT_CARE: EGLint = -1;

// errors / GetError return values
pub const SUCCESS:             EGLint = 0x3000;
pub const NOT_INITIALIZED:     EGLint = 0x3001;
pub const BAD_ACCESS:          EGLint = 0x3002;
pub const BAD_ALLOC:           EGLint = 0x3003;
pub const BAD_ATTRIBUTE:       EGLint = 0x3004;
pub const BAD_CONFIG:          EGLint = 0x3005;
pub const BAD_CONTEXT:         EGLint = 0x3006;
pub const BAD_CURRENT_SURFACE: EGLint = 0x3007;
pub const BAD_DISPLAY:         EGLint = 0x3008;
pub const BAD_MATCH:           EGLint = 0x3009;
pub const BAD_NATIVE_PIXMAP:   EGLint = 0x300A;
pub const BAD_NATIVE_WINDOW:   EGLint = 0x300B;
pub const BAD_PARAMETER:       EGLint = 0x300C;
pub const BAD_SURFACE:         EGLint = 0x300D;
pub const CONTEXT_LOST:        EGLint = 0x300E;  // EGL 1.1 - IMG_power_management

// config attributes
pub const BUFFER_SIZE:             EGLint = 0x3020;
pub const ALPHA_SIZE:              EGLint = 0x3021;
pub const BLUE_SIZE:               EGLint = 0x3022;
pub const GREEN_SIZE:              EGLint = 0x3023;
pub const RED_SIZE:                EGLint = 0x3024;
pub const DEPTH_SIZE:              EGLint = 0x3025;
pub const STENCIL_SIZE:            EGLint = 0x3026;
pub const CONFIG_CAVEAT:           EGLint = 0x3027;
pub const CONFIG_ID:               EGLint = 0x3028;
pub const LEVEL:                   EGLint = 0x3029;
pub const MAX_PBUFFER_HEIGHT:      EGLint = 0x302A;
pub const MAX_PBUFFER_PIXELS:      EGLint = 0x302B;
pub const MAX_PBUFFER_WIDTH:       EGLint = 0x302C;
pub const NATIVE_RENDERABLE:       EGLint = 0x302D;
pub const NATIVE_VISUAL_ID:        EGLint = 0x302E;
pub const NATIVE_VISUAL_TYPE:      EGLint = 0x302F;
pub const SAMPLES:                 EGLint = 0x3031;
pub const SAMPLE_BUFFERS:          EGLint = 0x3032;
pub const SURFACE_TYPE:            EGLint = 0x3033;
pub const TRANSPARENT_TYPE:        EGLint = 0x3034;
pub const TRANSPARENT_BLUE_VALUE:  EGLint = 0x3035;
pub const TRANSPARENT_GREEN_VALUE: EGLint = 0x3036;
pub const TRANSPARENT_RED_VALUE:   EGLint = 0x3037;
pub const NONE:                    EGLint = 0x3038; // attrib list terminator
pub const BIND_TO_TEXTURE_RGB:     EGLint = 0x3039;
pub const BIND_TO_TEXTURE_RGBA:    EGLint = 0x303A;
pub const MIN_SWAP_INTERVAL:       EGLint = 0x303B;
pub const MAX_SWAP_INTERVAL:       EGLint = 0x303C;
pub const LUMINANCE_SIZE:          EGLint = 0x303D;
pub const ALPHA_MASK_SIZE:         EGLint = 0x303E;
pub const COLOR_BUFFER_TYPE:       EGLint = 0x303F;
pub const RENDERABLE_TYPE:         EGLint = 0x3040;
pub const MATCH_NATIVE_PIXMAP:     EGLint = 0x3041;  // psseudo-attribute (not queryable)
pub const CONFORMANT:              EGLint = 0x3042;

// config attribute values
pub const SLOW_CONFIG:           EGLint = 0x3050;  // CONFIG_CAVEAT value
pub const NON_CONFORMANT_CONFIG: EGLint = 0x3051;  // CONFIG_CAVEAT value
pub const TRANSPARENT_RGB:       EGLint = 0x3052;  // TRANSPARENT_TYPE value
pub const RGB_BUFFER:            EGLint = 0x308E;  // COLOR_BUFFER_TYPE value
pub const LUMINANCE_BUFFER:      EGLint = 0x308F;  // COLOR_BUFFER_TYPE value

// more config attribute values, for TEXTURE_FORMAT
pub const NO_TEXTURE:   EGLint = 0x305C;
pub const TEXTURE_RGB:  EGLint = 0x305D;
pub const TEXTURE_RGBA: EGLint = 0x305E;
pub const TEXTURE_2D:   EGLint = 0x305F;

// config attribute mask bits
pub const PBUFFER_BIT:                 EGLint = 0x0001;  // SURFACE_TYPE mask bits
pub const PIXMAP_BIT:                  EGLint = 0x0002;  // SURFACE_TYPE mask bits
pub const WINDOW_BIT:                  EGLint = 0x0004;  // SURFACE_TYPE mask bits
pub const VG_COLORSPACE_LINEAR_BIT:    EGLint = 0x0020;  // SURFACE_TYPE mask bits
pub const VG_ALPHA_FORMAT_PRE_BIT:     EGLint = 0x0040;  // SURFACE_TYPE mask bits
pub const MULTISAMPLE_RESOLVE_BOX_BIT: EGLint = 0x0200;  // SURFACE_TYPE mask bits
pub const SWAP_BEHAVIOR_PRESERVED_BIT: EGLint = 0x0400;  // SURFACE_TYPE mask bits

pub const OPENGL_ES_BIT:  EGLint = 0x0001; // RENDERABLE_TYPE mask bits
pub const OPENVG_BIT:     EGLint = 0x0002; // RENDERABLE_TYPE mask bits
pub const OPENGL_ES2_BIT: EGLint = 0x0004; // RENDERABLE_TYPE mask bits
pub const OPENGL_BIT:     EGLint = 0x0008; // RENDERABLE_TYPE mask bits

// QueryString targets
pub const VENDOR:      EGLint = 0x3053;
pub const VERSION:     EGLint = 0x3054;
pub const EXTENSIONS:  EGLint = 0x3055;
pub const CLIENT_APIS: EGLint = 0x308D;

// QuerySurface / SurfaceAttrib / CreatePbufferSurface targets
pub const HEIGHT:                EGLint = 0x3056;
pub const WIDTH:                 EGLint = 0x3057;
pub const LARGEST_PBUFFER:       EGLint = 0x3058;
pub const TEXTURE_FORMAT:        EGLint = 0x3080;
pub const TEXTURE_TARGET:        EGLint = 0x3081;
pub const MIPMAP_TEXTURE:        EGLint = 0x3082;
pub const MIPMAP_LEVEL:          EGLint = 0x3083;
pub const RENDER_BUFFER:         EGLint = 0x3086;
pub const VG_COLORSPACE:         EGLint = 0x3087;
pub const VG_ALPHA_FORMAT:       EGLint = 0x3088;
pub const HORIZONTAL_RESOLUTION: EGLint = 0x3090;
pub const VERTICAL_RESOLUTION:   EGLint = 0x3091;
pub const PIXEL_ASPECT_RATIO:    EGLint = 0x3092;
pub const SWAP_BEHAVIOR:         EGLint = 0x3093;
pub const MULTISAMPLE_RESOLVE:   EGLint = 0x3099;

// RENDER_BUFFER values / BindTexImage / ReleaseTexImage buffer targets
pub const BACK_BUFFER:   EGLint = 0x3084;
pub const SINGLE_BUFFER: EGLint = 0x3085;

// OpenVG color spaces */
pub const VG_COLORSPACE_sRGB:   EGLint = 0x3089;  // VG_COLORSPACE value
pub const VG_COLORSPACE_LINEAR: EGLint = 0x308A;  // VG_COLORSPACE value

// OpenVG alpha formats
pub const VG_ALPHA_FORMAT_NONPRE: EGLint = 0x308B; // ALPHA_FORMAT value
pub const VG_ALPHA_FORMAT_PRE:    EGLint = 0x308C; // ALPHA_FORMAT value

// constant scale factor by which fractional display resolutions & aspect ratio are scaled when
// queried as integer values
pub const DISPLAY_SCALING: EGLint = 10000;

// unknown display resolution/aspect ratio
pub const UNKNOWN: EGLint = -1;

// back buffer swap behaviors
pub const BUFFER_PRESERVED: EGLint = 0x3094; // SWAP_BEHAVIOR value
pub const BUFFER_DESTROYED: EGLint = 0x3095; // SWAP_BEHAVIOR value

// CreatePbufferFromClientBuffer buffer types
pub const OPENVG_IMAGE: EGLint = 0x3096;

// QueryContext targets
pub const CONTEXT_CLIENT_TYPE: EGLint = 0x3097;

// CreateContext attributes
pub const CONTEXT_CLIENT_VERSION: EGLint = 0x3098;

// multisample resolution behaviors
pub const MULTISAMPLE_RESOLVE_DEFAULT: EGLint = 0x309A; // MULTISAMPLE_RESOLVE value
pub const MULTISAMPLE_RESOLVE_BOX:     EGLint = 0x309B; // MULTISAMPLE_RESOLVE value

// BindAPI/QueryAPI targets
pub const OPENGL_ES_API: EGLint = 0x30A0;
pub const OPENVG_API:    EGLint = 0x30A1;
pub const OPENGL_API:    EGLint = 0x30A2;

// GetCurrentSurface targets
pub const DRAW: EGLint = 0x3059;
pub const READ: EGLint = 0x305A;

// WaitNative engines
pub const CORE_NATIVE_ENGINE: EGLint = 0x305B;

// EGL 1.2 tokens renamed for consistency in EGL 1.3
pub const COLORSPACE:          EGLint = VG_COLORSPACE;
pub const ALPHA_FORMAT:        EGLint = VG_ALPHA_FORMAT;
pub const COLORSPACE_sRGB:     EGLint = VG_COLORSPACE_sRGB;
pub const COLORSPACE_LINEAR:   EGLint = VG_COLORSPACE_LINEAR;
pub const ALPHA_FORMAT_NONPRE: EGLint = VG_ALPHA_FORMAT_NONPRE;
pub const ALPHA_FORMAT_PRE:    EGLint = VG_ALPHA_FORMAT_PRE;

// -------------------------------------------------------------------------------------------------
// FUNCTIONS
// -------------------------------------------------------------------------------------------------

pub fn bind_api(api: EGLenum) -> bool {
    unsafe {
        ffi::eglBindAPI(api) == TRUE
    }
}

pub fn bind_tex_image(display: EGLDisplay, surface: EGLSurface, buffer: EGLint) -> bool {
    unsafe {
        ffi::eglBindTexImage(display, surface, buffer) == TRUE
    }
}

pub fn choose_config(display: EGLDisplay, attrib_list: &[EGLint],
                     config_size: EGLint) -> Option<EGLConfig> {
    unsafe {
        let     config: EGLConfig = ptr::null_mut();
        let mut count:  EGLint = 0;

        if ffi::eglChooseConfig(display, attrib_list.as_ptr(), config, config_size,
                                &mut count) == TRUE {
            Some(config)
        } else {
            None
        }
    }
}

pub fn copy_buffers(display: EGLDisplay, surface: EGLSurface,
                    target: EGLNativePixmapType) -> bool {
    unsafe {
        ffi::eglCopyBuffers(display, surface, target) == TRUE
    }
}

pub fn create_context(display: EGLDisplay, config: EGLConfig, share_context: EGLContext,
                      attrib_list: &[EGLint]) -> Option<EGLContext> {
    unsafe {
        let context = ffi::eglCreateContext(display, config, share_context, attrib_list.as_ptr());

        if !context.is_null() {
            Some(context)
        } else {
            None
        }
    }
}

pub fn create_pbuffer_from_client_buffer(display: EGLDisplay, buffer_type: EGLenum,
                                         buffer: EGLClientBuffer, config: EGLConfig,
                                         attrib_list: &[EGLint]) -> Option<EGLSurface> {
    unsafe {
        let surface = ffi::eglCreatePbufferFromClientBuffer(display, buffer_type, buffer, config,
                                                            attrib_list.as_ptr());

        if !surface.is_null() {
            Some(surface)
        } else {
            None
        }
    }
}

pub fn create_pbuffer_surface(display: EGLDisplay, config: EGLConfig,
                              attrib_list: &[EGLint]) -> Option<EGLSurface> {
    unsafe {
        let surface = ffi::eglCreatePbufferSurface(display, config, attrib_list.as_ptr());

        if !surface.is_null() {
            Some(surface)
        } else {
            None
        }
    }
}

pub fn create_pixmap_surface(display: EGLDisplay, config: EGLConfig,
                             pixmap: EGLNativePixmapType,
                             attrib_list: &[EGLint]) -> Option<EGLSurface> {
    unsafe {
        let surface = ffi::eglCreatePixmapSurface(display, config, pixmap, attrib_list.as_ptr());

        if !surface.is_null() {
            Some(surface)
        } else {
            None
        }
    }
}

pub fn create_window_surface(display: EGLDisplay, config: EGLConfig,
                             win: EGLNativeWindowType,
                             attrib_list: &[EGLint]) -> Option<EGLSurface> {
    unsafe {
        let surface = ffi::eglCreateWindowSurface(display, config, win, attrib_list.as_ptr());

        if !surface.is_null() {
            Some(surface)
        } else {
            None
        }
    }
}

pub fn destroy_context(display: EGLDisplay, ctx: EGLContext) -> bool {
    unsafe {
        ffi::eglDestroyContext(display, ctx) == TRUE
    }
}

pub fn destroy_surface(display: EGLDisplay, surface: EGLSurface) -> bool {
    unsafe {
        ffi::eglDestroySurface(display, surface) == TRUE
    }
}

pub fn get_config_attrib(display: EGLDisplay, config: EGLConfig,
                         attribute: EGLint, value: &mut EGLint) -> bool {
    unsafe {
        ffi::eglGetConfigAttrib(display, config, attribute, value) == TRUE
    }
}

pub fn get_configs(display: EGLDisplay, config_size: EGLint) -> EGLConfigList {
    unsafe {
        let list = EGLConfigList{ configs: ptr::null_mut(),
                                  count:   0 };

        ffi::eglGetConfigs(display, list.configs, config_size, list.count as *mut int32_t);

        list
    }
}

pub fn get_current_context() -> Option<EGLContext> {
    unsafe {
        let context = ffi::eglGetCurrentContext();

        if !context.is_null() {
            Some(context)
        } else {
            None
        }
    }
}

pub fn get_current_display() -> Option<EGLDisplay> {
    unsafe {
        let display = ffi::eglGetCurrentDisplay();

        if !display.is_null() {
            Some(display)
        } else {
            None
        }
    }
}

pub fn get_current_surface(readdraw: EGLint) -> Option<EGLSurface> {
    unsafe {
        let surface = ffi::eglGetCurrentSurface(readdraw);

        if !surface.is_null() {
            Some(surface)
        } else {
            None
        }
    }
}

pub fn get_display(display_id: EGLNativeDisplayType) -> Option<EGLDisplay> {
    unsafe {
        let display = ffi::eglGetDisplay(display_id);

        if !display.is_null() {
            Some(display)
        } else {
            None
        }
    }
}

pub fn get_error() -> EGLint {
    unsafe {
        ffi::eglGetError()
    }
}

pub fn get_proc_address(procname: &str) -> extern "C" fn() {
    unsafe {
        let string = CString::new(procname).unwrap();

        ffi::eglGetProcAddress(string.as_ptr())
    }
}

pub fn initialize(display: EGLDisplay, major: &mut EGLint, minor: &mut EGLint) -> bool {
    unsafe {
        ffi::eglInitialize(display, major, minor) == TRUE
    }
}

pub fn make_current(display: EGLDisplay, draw: EGLSurface,
                    read: EGLSurface, ctx: EGLContext) -> bool {
    unsafe {
        ffi::eglMakeCurrent(display, draw, read, ctx) == TRUE
    }
}

pub fn query_api() -> EGLenum {
    unsafe {
        ffi::eglQueryAPI()
    }
}

pub fn query_context(display: EGLDisplay, ctx: EGLContext,
                     attribute: EGLint, value: &mut EGLint) -> bool {
    unsafe {
        ffi::eglQueryContext(display, ctx, attribute, value) == TRUE
    }
}

pub fn query_string(display: EGLDisplay, name: EGLint) -> Option<&'static CStr> {
    unsafe {
        let c_str = ffi::eglQueryString(display, name);

        if !c_str.is_null() {
            Some(CStr::from_ptr(c_str))
        } else {
            None
        }
    }
}

pub fn query_surface(display: EGLDisplay, surface: EGLSurface,
                     attribute: EGLint, value: *mut EGLint) -> bool {
    unsafe {
        ffi::eglQuerySurface(display, surface, attribute, value) == TRUE
    }
}

pub fn release_tex_image(display: EGLDisplay, surface: EGLSurface, buffer: EGLint) -> bool {
    unsafe {
        ffi::eglReleaseTexImage(display, surface, buffer) == TRUE
    }
}

pub fn release_thread() -> bool {
    unsafe {
        ffi::eglReleaseThread() == TRUE
    }
}

pub fn surface_attrib(display: EGLDisplay, surface: EGLSurface,
                      attribute: EGLint, value: EGLint) -> bool {
    unsafe {
        ffi::eglSurfaceAttrib(display, surface, attribute, value) == TRUE
    }
}

pub fn swap_buffers(display: EGLDisplay, surface: EGLSurface) -> bool {
    unsafe {
        ffi::eglSwapBuffers(display, surface) == TRUE
    }
}

pub fn swap_interface(display: EGLDisplay, interval: EGLint) -> bool {
    unsafe {
        ffi::eglSwapInterface(display, interval) == TRUE
    }
}

pub fn terminate(display: EGLDisplay) -> bool {
    unsafe {
        ffi::eglTerminate(display) == TRUE
    }
}

pub fn wait_client() -> bool {
    unsafe {
        ffi::eglWaitClient() == TRUE
    }
}

pub fn wait_gl() -> bool {
    unsafe {
        ffi::eglWaitGL() == TRUE
    }
}

pub fn wait_native(engine: EGLint) -> bool {
    unsafe {
        ffi::eglWaitNative(engine) == TRUE
    }
}

// -------------------------------------------------------------------------------------------------
// FFI
// -------------------------------------------------------------------------------------------------

mod ffi {
    use libc::c_char;

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
                 EGLSurface };

    extern {
        pub fn eglBindAPI(api: EGLenum) -> EGLBoolean;

        pub fn eglBindTexImage(dpy: EGLDisplay, surface: EGLSurface, buffer: EGLint) -> EGLBoolean;

        pub fn eglChooseConfig(dpy: EGLDisplay, attrib_list: *const EGLint,
                               configs: EGLConfig, config_size: EGLint,
                               num_config: *mut EGLint) -> EGLBoolean;

        pub fn eglCopyBuffers(dpy: EGLDisplay, surface: EGLSurface,
                              target: EGLNativePixmapType) -> EGLBoolean;

        pub fn eglCreateContext(dpy: EGLDisplay, config: EGLConfig,
                                share_context: EGLContext,
                                attrib_list: *const EGLint) -> EGLContext;

        pub fn eglCreatePbufferFromClientBuffer(dpy: EGLDisplay, buftype: EGLenum,
                                                buffer: EGLClientBuffer, config: EGLConfig,
                                                attrib_list: *const EGLint) -> EGLSurface;

        pub fn eglCreatePbufferSurface(dpy: EGLDisplay, config: EGLConfig,
                                       attrib_list: *const EGLint) -> EGLSurface;

        pub fn eglCreatePixmapSurface(dpy: EGLDisplay, config: EGLConfig,
                                      pixmap: EGLNativePixmapType,
                                      attrib_list: *const EGLint) -> EGLSurface;

        pub fn eglCreateWindowSurface(dpy: EGLDisplay, config: EGLConfig,
                                      win: EGLNativeWindowType,
                                      attrib_list: *const EGLint) -> EGLSurface;

        pub fn eglDestroyContext(dpy: EGLDisplay, ctx: EGLContext) -> EGLBoolean;

        pub fn eglDestroySurface(dpy: EGLDisplay, surface: EGLSurface) -> EGLBoolean;

        pub fn eglGetConfigAttrib(dpy: EGLDisplay, config: EGLConfig,
                                  attribute: EGLint, value: *mut EGLint) -> EGLBoolean;

        pub fn eglGetConfigs(dpy: EGLDisplay, configs: EGLConfig,
                             config_size: EGLint, num_config: *mut EGLint) -> EGLBoolean;

        pub fn eglGetCurrentContext() -> EGLContext;

        pub fn eglGetCurrentDisplay() -> EGLDisplay;

        pub fn eglGetCurrentSurface(readdraw: EGLint) -> EGLSurface;

        pub fn eglGetDisplay(display_id: EGLNativeDisplayType) -> EGLDisplay;

        pub fn eglGetError() -> EGLint;

        pub fn eglGetProcAddress(procname: *const c_char) -> extern "C" fn();

        pub fn eglInitialize(dpy: EGLDisplay, major: *mut EGLint, minor: *mut EGLint) -> EGLBoolean;

        pub fn eglMakeCurrent(dpy: EGLDisplay, draw: EGLSurface,
                              read: EGLSurface, ctx: EGLContext) -> EGLBoolean;

        pub fn eglQueryAPI() -> EGLenum;

        pub fn eglQueryContext(dpy: EGLDisplay, ctx: EGLContext,
                               attribute: EGLint, value: *mut EGLint) -> EGLBoolean;

        pub fn eglQueryString(dpy: EGLDisplay, name: EGLint) -> *const c_char;

        pub fn eglQuerySurface(dpy: EGLDisplay, surface: EGLSurface,
                               attribute: EGLint, value: *mut EGLint) -> EGLBoolean;

        pub fn eglReleaseTexImage(dpy: EGLDisplay, surface: EGLSurface,
                                  buffer: EGLint) -> EGLBoolean;

        pub fn eglReleaseThread() -> EGLBoolean;

        pub fn eglSurfaceAttrib(dpy: EGLDisplay, surface: EGLSurface,
                                attribute: EGLint, value: EGLint) -> EGLBoolean;

        pub fn eglSwapBuffers(dpy: EGLDisplay, surface: EGLSurface) -> EGLBoolean;

        pub fn eglSwapInterface(dpy: EGLDisplay, interval: EGLint) -> EGLBoolean;

        pub fn eglTerminate(dpy: EGLDisplay) -> EGLBoolean;

        pub fn eglWaitClient() -> EGLBoolean;

        pub fn eglWaitGL() -> EGLBoolean;

        pub fn eglWaitNative(engine: EGLint) -> EGLBoolean;
    }
}