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
// | Authors: Sean Kerr <sean@metatomic.io>                                                        |
// |          Timothée Haudebourg <author@haudebourg.net>                                          |
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
use khronos::{ khronos_int32_t,
               khronos_utime_nanoseconds_t };

use libc::{ c_uint,
            c_void,
            int32_t };

// -------------------------------------------------------------------------------------------------
// LINKING
// -------------------------------------------------------------------------------------------------

#[link(name = "EGL")]
extern {}

// ------------------------------------------------------------------------------------------------
// EGL 1.0
// ------------------------------------------------------------------------------------------------

pub type EGLBoolean           = c_uint;
pub type EGLint               = khronos_int32_t;
pub type EGLDisplay           = *mut c_void;
pub type EGLConfig            = *mut c_void;
pub type EGLContext           = *mut c_void;
pub type EGLSurface           = *mut c_void;
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

pub fn choose_config(display: EGLDisplay, attrib_list: &[EGLint],
                     config_size: EGLint) -> Option<EGLConfig> {
    unsafe {
        let mut config: EGLConfig = ptr::null_mut();
        let mut count:  EGLint = 0;;

        let attribs = if attrib_list.len() > 0 {
            attrib_list.as_ptr()
        } else {
            ptr::null()
        };

        if ffi::eglChooseConfig(display, attribs, &mut config, config_size,
                                &mut count) == EGL_TRUE {
            if count > 0 {
                Some(config)
            } else {
                None
            }
        } else {
            None
        }
    }
}

pub fn copy_buffers(display: EGLDisplay, surface: EGLSurface,
                    target: EGLNativePixmapType) -> bool {
    unsafe {
        ffi::eglCopyBuffers(display, surface, target) == EGL_TRUE
    }
}

pub fn create_context(display: EGLDisplay, config: EGLConfig, share_context: EGLContext,
                      attrib_list: &[EGLint]) -> Option<EGLContext> {
    unsafe {
        let attribs = if attrib_list.len() > 0 {
            attrib_list.as_ptr()
        } else {
            ptr::null()
        };

        let context = ffi::eglCreateContext(display, config, share_context, attribs);

        if !context.is_null() {
            Some(context)
        } else {
            None
        }
    }
}

pub fn create_pbuffer_surface(display: EGLDisplay, config: EGLConfig,
                              attrib_list: &[EGLint]) -> Option<EGLSurface> {
    unsafe {
        let attribs = if attrib_list.len() > 0 {
            attrib_list.as_ptr()
        } else {
            ptr::null()
        };

        let surface = ffi::eglCreatePbufferSurface(display, config, attribs);

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
        let attribs = if attrib_list.len() > 0 {
            attrib_list.as_ptr()
        } else {
            ptr::null()
        };

        let surface = ffi::eglCreatePixmapSurface(display, config, pixmap, attribs);

        if !surface.is_null() {
            Some(surface)
        } else {
            None
        }
    }
}

pub fn create_window_surface(display: EGLDisplay, config: EGLConfig,
                             window: EGLNativeWindowType,
                             attrib_list: &[EGLint]) -> Option<EGLSurface> {
    unsafe {
        let attribs = if attrib_list.len() > 0 {
            attrib_list.as_ptr()
        } else {
            ptr::null()
        };

        let surface = ffi::eglCreateWindowSurface(display, config, window, attribs);

        if !surface.is_null() {
            Some(surface)
        } else {
            None
        }
    }
}

pub fn destroy_context(display: EGLDisplay, ctx: EGLContext) -> bool {
    unsafe {
        ffi::eglDestroyContext(display, ctx) == EGL_TRUE
    }
}

pub fn destroy_surface(display: EGLDisplay, surface: EGLSurface) -> bool {
    unsafe {
        ffi::eglDestroySurface(display, surface) == EGL_TRUE
    }
}

pub fn get_config_attrib(display: EGLDisplay, config: EGLConfig,
                         attribute: EGLint, value: &mut EGLint) -> bool {
    unsafe {
        ffi::eglGetConfigAttrib(display, config, attribute, value) == EGL_TRUE
    }
}

pub fn get_configs(display: EGLDisplay, config_size: EGLint) -> Vec<EGLConfig> {
    if config_size > 0 {
        let capacity = config_size as usize;

        unsafe {
            let mut vec: Vec<EGLConfig> = Vec::with_capacity(capacity);
            let config_list = vec.as_mut_ptr();
            let mut num_config: int32_t = 0;
            std::mem::forget(vec);

            ffi::eglGetConfigs(display, config_list, config_size, &mut num_config as *mut int32_t);

            Vec::from_raw_parts(config_list, num_config as usize, capacity)
        }
    } else {
        Vec::new()
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
        ffi::eglInitialize(display, major, minor) == EGL_TRUE
    }
}

pub fn make_current(display: EGLDisplay, draw: EGLSurface,
                    read: EGLSurface, ctx: EGLContext) -> bool {
    unsafe {
        ffi::eglMakeCurrent(display, draw, read, ctx) == EGL_TRUE
    }
}

pub fn query_context(display: EGLDisplay, ctx: EGLContext,
                     attribute: EGLint, value: &mut EGLint) -> bool {
    unsafe {
        ffi::eglQueryContext(display, ctx, attribute, value) == EGL_TRUE
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
                     attribute: EGLint, value: &mut EGLint) -> bool {
    unsafe {
        ffi::eglQuerySurface(display, surface, attribute, value) == EGL_TRUE
    }
}

pub fn swap_buffers(display: EGLDisplay, surface: EGLSurface) -> bool {
    unsafe {
        ffi::eglSwapBuffers(display, surface) == EGL_TRUE
    }
}

pub fn terminate(display: EGLDisplay) -> bool {
    unsafe {
        ffi::eglTerminate(display) == EGL_TRUE
    }
}

pub fn wait_gl() -> bool {
    unsafe {
        ffi::eglWaitGL() == EGL_TRUE
    }
}

pub fn wait_native(engine: EGLint) -> bool {
    unsafe {
        ffi::eglWaitNative(engine) == EGL_TRUE
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

pub fn bind_tex_image(display: EGLDisplay, surface: EGLSurface, buffer: EGLint) -> bool {
    unsafe {
        ffi::eglBindTexImage(display, surface, buffer) == EGL_TRUE
    }
}

pub fn release_tex_image(display: EGLDisplay, surface: EGLSurface, buffer: EGLint) -> bool {
    unsafe {
        ffi::eglReleaseTexImage(display, surface, buffer) == EGL_TRUE
    }
}

pub fn surface_attrib(display: EGLDisplay, surface: EGLSurface,
                      attribute: EGLint, value: EGLint) -> bool {
    unsafe {
        ffi::eglSurfaceAttrib(display, surface, attribute, value) == EGL_TRUE
    }
}

pub fn swap_interval(display: EGLDisplay, interval: EGLint) -> bool {
    unsafe {
        ffi::eglSwapInterval(display, interval) == EGL_TRUE
    }
}

// ------------------------------------------------------------------------------------------------
// EGL 1.2
// ------------------------------------------------------------------------------------------------

pub type EGLenum              = c_uint;
pub type EGLClientBuffer      = *mut c_void;

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

pub fn bind_api(api: EGLenum) -> bool {
    unsafe {
        ffi::eglBindAPI(api) == EGL_TRUE
    }
}

pub fn query_api() -> EGLenum {
    unsafe {
        ffi::eglQueryAPI()
    }
}

pub fn create_pbuffer_from_client_buffer(display: EGLDisplay, buffer_type: EGLenum,
                                         buffer: EGLClientBuffer, config: EGLConfig,
                                         attrib_list: &[EGLint]) -> Option<EGLSurface> {
    unsafe {
        let attribs = if attrib_list.len() > 0 {
            attrib_list.as_ptr()
        } else {
            ptr::null()
        };

        let surface = ffi::eglCreatePbufferFromClientBuffer(display, buffer_type, buffer, config,
                                                            attribs);

        if !surface.is_null() {
            Some(surface)
        } else {
            None
        }
    }
}

pub fn release_thread() -> bool {
    unsafe {
        ffi::eglReleaseThread() == EGL_TRUE
    }
}

pub fn wait_client() -> bool {
    unsafe {
        ffi::eglWaitClient() == EGL_TRUE
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

// ------------------------------------------------------------------------------------------------
// EGL 1.5
// ------------------------------------------------------------------------------------------------

pub type EGLSync              = *mut c_void;
pub type EGLAttrib            = usize;
pub type EGLTime              = khronos_utime_nanoseconds_t;
pub type EGLImage             = *mut c_void;

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

pub fn create_sync(dpy: EGLDisplay, ty: EGLenum, attrib_list: &[EGLAttrib]) -> Option<EGLSync> {
    let attribs = if attrib_list.len() > 0 {
        attrib_list.as_ptr()
    } else {
        ptr::null()
    };

    unsafe {
        let sync = ffi::eglCreateSync(dpy, ty, attribs);
        if sync.is_null() {
            None
        } else {
            Some(sync)
        }
    }
}

pub fn destroy_sync(dpy: EGLDisplay, sync: EGLSync) -> bool {
    unsafe {
        ffi::eglDestroySync(dpy, sync) == EGL_TRUE
    }
}

pub fn client_wait_sync(dpy: EGLDisplay, sync: EGLSync, flags: EGLint, timeout: EGLTime) -> EGLint {
    unsafe {
        ffi::eglClientWaitSync(dpy, sync, flags, timeout)
    }
}

pub fn get_sync_attrib(dpy: EGLDisplay, sync: EGLSync, attribute: EGLint) -> Option<EGLAttrib> {
    unsafe {
        let mut value = 0;
        if ffi::eglGetSyncAttrib(dpy, sync, attribute, &mut value as *mut EGLAttrib) == EGL_TRUE {
            None
        } else {
            Some(value)
        }
    }
}

pub fn create_image(dpy: EGLDisplay, ctx: EGLContext, target: EGLenum, buffer: EGLClientBuffer, attrib_list: &[EGLAttrib]) -> Option<EGLImage> {
    let attribs = if attrib_list.len() > 0 {
        attrib_list.as_ptr()
    } else {
        ptr::null()
    };

    unsafe {
        let image = ffi::eglCreateImage(dpy, ctx, target, buffer, attribs);
        if image.is_null() {
            None
        } else {
            Some(image)
        }
    }
}

pub fn destroy_image(dpy: EGLDisplay, image: EGLImage) -> bool {
    unsafe {
        ffi::eglDestroyImage(dpy, image) == EGL_TRUE
    }
}

pub fn get_platform_display(platform: EGLenum, native_display: *mut c_void, attrib_list: &[EGLAttrib]) -> Option<EGLDisplay> {
    let attribs = if attrib_list.len() > 0 {
        attrib_list.as_ptr()
    } else {
        ptr::null()
    };

    unsafe {
        let display = ffi::eglGetPlatformDisplay(platform, native_display, attribs);
        if display.is_null() {
            None
        } else {
            Some(display)
        }
    }
}

pub fn create_platform_window_surface(dpy: EGLDisplay, config: EGLConfig, native_window: *mut c_void, attrib_list: &[EGLAttrib]) -> Option<EGLSurface> {
    let attribs = if attrib_list.len() > 0 {
        attrib_list.as_ptr()
    } else {
        ptr::null()
    };

    unsafe {
        let surface = ffi::eglCreatePlatformWindowSurface(dpy, config, native_window, attribs);
        if surface.is_null() {
            None
        } else {
            Some(surface)
        }
    }
}

pub fn create_platform_pixmap_surface(dpy: EGLDisplay, config: EGLConfig, native_pixmap: *mut c_void, attrib_list: &[EGLAttrib]) -> Option<EGLSurface> {
    let attribs = if attrib_list.len() > 0 {
        attrib_list.as_ptr()
    } else {
        ptr::null()
    };

    unsafe {
        let surface = ffi::eglCreatePlatformPixmapSurface(dpy, config, native_pixmap, attribs);
        if surface.is_null() {
            None
        } else {
            Some(surface)
        }
    }
}

pub fn wait_sync(dpy: EGLDisplay, sync: EGLSync, flags: EGLint) -> bool {
    unsafe {
        ffi::eglWaitSync(dpy, sync, flags) == EGL_TRUE
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
