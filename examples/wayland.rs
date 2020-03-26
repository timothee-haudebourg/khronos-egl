extern crate wayland_client;
extern crate wayland_protocols;
extern crate wayland_egl;
extern crate khronos_egl as egl;
extern crate gl;

use std::cmp::min;
use std::io::Write;
use std::os::unix::io::AsRawFd;

// use wayland_client::protocol::{wl_compositor, wl_keyboard, wl_pointer, wl_seat, wl_shell, wl_shm};
use wayland_client::{
	Display,
	GlobalManager,
	EventQueue,
	Main,
	Attached,
	DispatchData,
	protocol::{
		wl_compositor::WlCompositor,
		wl_shell::WlShell,
		wl_shm::WlShm,
	}
};

use wayland_protocols::xdg_shell::client::{
	xdg_wm_base::{self, XdgWmBase},
	xdg_surface::{self, XdgSurface}
};

fn process_xdg_event(xdg: Main<XdgWmBase>, event: xdg_wm_base::Event, _dd: DispatchData) {
	use xdg_wm_base::Event::*;

	match event {
		Ping { serial } => xdg.pong(serial),
		_ => ()
	}
}

fn handle_xdg_surface_event(xdg_surface: Main<XdgSurface>, event: xdg_surface::Event, _dd: DispatchData) {
	use xdg_surface::Event::*;

	match event {
		Configure { serial } => {
			println!("configure");
			xdg_surface.ack_configure(serial);
		},
		_ => ()
	}
}

struct DisplayConnection {
	display: Display,
	event_queue: EventQueue,
	compositor: Main<WlCompositor>,
	shell: Main<WlShell>,
	shm: Main<WlShm>,
	xdg: Main<XdgWmBase>
}

fn setup_wayland() -> DisplayConnection {
	let display = wayland_client::Display::connect_to_env()
		.expect("unable to connect to the wayland server");
	let mut event_queue = display.create_event_queue();
	let attached_display = display.clone().attach(event_queue.token());

	let globals = wayland_client::GlobalManager::new(&attached_display);

	// Roundtrip to retrieve the globals list
	event_queue.sync_roundtrip(&mut (), |_, _, _| unreachable!()).unwrap();

	// Get the compositor.
	let compositor: Main<WlCompositor> = globals.instantiate_exact(1).unwrap();
	let shell: Main<WlShell> = globals.instantiate_exact(1).unwrap();
	let shm: Main<WlShm> = globals.instantiate_exact(1).unwrap();

	// Xdg protocol.
	let xdg: Main<XdgWmBase> = globals.instantiate_exact(1).unwrap();
	xdg.quick_assign(process_xdg_event);

	DisplayConnection {
		display,
		event_queue,
		compositor,
		shell,
		shm,
		xdg
	}
}

fn setup_egl(display: &Display) -> egl::Display {
	let egl_display = egl::get_display(display.get_display_ptr() as *mut std::ffi::c_void).unwrap();
    let egl_version = egl::initialize(egl_display).unwrap();

	egl_display
}

fn create_context(display: egl::Display) -> (egl::Context, egl::Config) {
	let attributes = [
		egl::RED_SIZE, 8,
		egl::GREEN_SIZE, 8,
		egl::BLUE_SIZE, 8,
		egl::NONE,
	];

	let config = egl::choose_first_config(display, &attributes)
		.expect("unable to choose an EGL configuration")
		.expect("no EGL configuration found");

	let context_attributes = [
		egl::CONTEXT_MAJOR_VERSION, 4,
		egl::CONTEXT_MINOR_VERSION, 0,
		egl::CONTEXT_OPENGL_PROFILE_MASK, egl::CONTEXT_OPENGL_CORE_PROFILE_BIT,
		egl::NONE,
	];

	let context = egl::create_context(display, config, None, &context_attributes)
		.expect("unable to create an EGL context");

	(context, config)
}

fn create_surface(ctx: &DisplayConnection, egl_display: egl::Display, egl_config: egl::Config, width: i32, height: i32) {
	let surface = ctx.compositor.create_surface();
	let xdg_surface = ctx.xdg.get_xdg_surface(&surface);
	xdg_surface.quick_assign(handle_xdg_surface_event);

	let xdg_toplevel = xdg_surface.get_toplevel();

	xdg_toplevel.set_app_id("khronos-egl-test".to_string());
	xdg_toplevel.set_title("Test".to_string());

	surface.commit();
	ctx.display.flush();

	let wl_egl_surface = wayland_egl::WlEglSurface::new(&surface, width, height);

	let egl_surface = unsafe {
		egl::create_window_surface(egl_display, egl_config, wl_egl_surface.ptr() as egl::NativeWindowType, None)
			.expect("unable to create an EGL surface")
	};

	surface.commit();
	ctx.display.flush();
}

fn main() {
	// Setup Open GL.
	egl::bind_api(egl::OPENGL_API);
    gl::load_with(|name| egl::get_proc_address(name).unwrap() as *const std::ffi::c_void);

	// Setup the Wayland client.
	let mut ctx = setup_wayland();

	// Setup EGL.
	let egl_display = setup_egl(&ctx.display);
	let (egl_context, egl_config) = create_context(egl_display);

	create_surface(&ctx, egl_display, egl_config, 800, 600);

	loop {
		ctx.event_queue.dispatch(&mut (), |_, _, _| { /* we ignore unfiltered messages */ }).unwrap();
    }
}
