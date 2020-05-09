use std::ffi::c_void;

mod wayland;

pub(super) use self::wayland::*;

use super::Nwin;

#[link(name = "wayland-client")]
#[link(name = "wayland-egl")]
#[link(name = "wayland-cursor")]
#[link(name = "EGL")]
//#[link(name = "GL")]
#[link(name = "GLESv2")]
extern "C" {
    fn wl_display_disconnect(display: *mut c_void) -> ();
    fn wl_display_flush(display: *mut c_void) -> i32;
    pub(super) fn wl_display_dispatch(display: *mut c_void) -> i32;
    pub(super) fn wl_proxy_marshal_constructor(
        name: *mut c_void,
        opcode: u32,
        interface: &WlInterface,
        p: *mut c_void,
    ) -> *mut c_void;
    pub(super) fn wl_proxy_add_listener(
        proxy: *mut c_void,
        implementation: *const *mut c_void,
        data: *mut c_void,
    ) -> i32;
    fn wl_proxy_marshal_constructor_versioned(
        proxy: *mut c_void,
        opcode: u32,
        interface: *const WlInterface,
        version: u32,
        name: u32,
        name2: *const c_void,
        version2: u32,
        pointer: *mut c_void,
    ) -> *mut c_void;
    fn wl_proxy_destroy(proxy: *mut c_void) -> ();
}

fn get(value: *mut dyn Nwin) -> *mut WaylandWindow {
    let value: [*mut c_void; 2] = unsafe { std::mem::transmute(value) };
    value[0] as *mut _ as *mut _
}

static mut ZXDG_SURFACE_V6_INTERFACE: WlInterface = WlInterface {
    /** Interface name */
    name: b"zxdg_surface_v6\0".as_ptr().cast(),
    /** Interface version */
    version: 1,
    /** Number of methods (requests) */
    method_count: 5,
    /** Method (request) signatures */
    methods: [
        WlMessage {
            name: b"destroy\0".as_ptr().cast(),
            signature: b"\0".as_ptr().cast(),
            wl_interface: std::ptr::null(),
        },
        WlMessage {
            name: b"get_toplevel\0".as_ptr().cast(),
            signature: b"n\0".as_ptr().cast(),
            wl_interface: unsafe { &(&wl_surface_interface as *const _) },
        },
        WlMessage {
            name: b"get_popup\0".as_ptr().cast(),
            signature: b"noo\0".as_ptr().cast(),
            wl_interface: &(&ZXDG_TOPLEVEL_V6_INTERFACE as *const _),
        },
        WlMessage {
            name: b"set_window_geometry\0".as_ptr().cast(),
            signature: b"iiii\0".as_ptr().cast(),
            wl_interface: std::ptr::null(),
        },
        WlMessage {
            name: b"ack_configure\0".as_ptr().cast(),
            signature: b"u\0".as_ptr().cast(),
            wl_interface: std::ptr::null(),
        },
    ]
    .as_ptr(),
    /** Number of events */
    event_count: 1,
    /** Event signatures */
    events: [WlMessage {
        name: b"configure\0".as_ptr().cast(),
        signature: b"u\0".as_ptr().cast(),
        wl_interface: std::ptr::null(),
    }]
    .as_ptr(), // *wl_message
};

const ZXDG_TOPLEVEL_V6_INTERFACE: WlInterface = WlInterface {
    /** Interface name */
    name: b"zxdg_toplevel_v6\0".as_ptr().cast(),
    /** Interface version */
    version: 1,
    /** Number of methods (requests) */
    method_count: 14,
    /** Method (request) signatures */
    methods: [
        WlMessage {
            name: b"destroy\0".as_ptr().cast(),
            signature: b"\0".as_ptr().cast(),
            wl_interface: std::ptr::null(),
        },
        WlMessage {
            name: b"set_parent\0".as_ptr().cast(),
            signature: b"?o\0".as_ptr().cast(),
            wl_interface: std::ptr::null(),
        },
        WlMessage {
            name: b"set_title\0".as_ptr().cast(),
            signature: b"s\0".as_ptr().cast(),
            wl_interface: std::ptr::null(),
        },
        WlMessage {
            name: b"set_app_id\0".as_ptr().cast(),
            signature: b"s\0".as_ptr().cast(),
            wl_interface: std::ptr::null(),
        },
        WlMessage {
            name: b"show_window_menu\0".as_ptr().cast(),
            signature: b"ouii\0".as_ptr().cast(),
            wl_interface: std::ptr::null(),
        },
        WlMessage {
            name: b"move\0".as_ptr().cast(),
            signature: b"ou\0".as_ptr().cast(),
            wl_interface: std::ptr::null(),
        },
        WlMessage {
            name: b"resize\0".as_ptr().cast(),
            signature: b"ouu\0".as_ptr().cast(),
            wl_interface: std::ptr::null(),
        },
        WlMessage {
            name: b"set_max_size\0".as_ptr().cast(),
            signature: b"ii\0".as_ptr().cast(),
            wl_interface: std::ptr::null(),
        },
        WlMessage {
            name: b"set_min_size\0".as_ptr().cast(),
            signature: b"ii\0".as_ptr().cast(),
            wl_interface: std::ptr::null(),
        },
        WlMessage {
            name: b"set_maximized\0".as_ptr().cast(),
            signature: b"\0".as_ptr().cast(),
            wl_interface: std::ptr::null(),
        },
        WlMessage {
            name: b"unset_maximized\0".as_ptr().cast(),
            signature: b"\0".as_ptr().cast(),
            wl_interface: std::ptr::null(),
        },
        WlMessage {
            name: b"set_fullscreen\0".as_ptr().cast(),
            signature: b"?o\0".as_ptr().cast(),
            wl_interface: std::ptr::null(),
        },
        WlMessage {
            name: b"unset_fullscreen\0".as_ptr().cast(),
            signature: b"\0".as_ptr().cast(),
            wl_interface: std::ptr::null(),
        },
        WlMessage {
            name: b"set_minimized\0".as_ptr().cast(),
            signature: b"\0".as_ptr().cast(),
            wl_interface: std::ptr::null(),
        },
    ]
    .as_ptr(),
    /** Number of events */
    event_count: 2,
    /** Event signatures */
    events: [
        WlMessage {
            name: b"configure\0".as_ptr().cast(),
            signature: b"iia\0".as_ptr().cast(),
            wl_interface: std::ptr::null(),
        },
        WlMessage {
            name: b"close\0".as_ptr().cast(),
            signature: b"\0".as_ptr().cast(),
            wl_interface: std::ptr::null(),
        },
    ]
    .as_ptr(), // *wl_message
};

static mut ZXDG_SHELL_V6_INTERFACE: WlInterface = WlInterface {
    /** Interface name */
    name: b"zxdg_shell_v6\0".as_ptr().cast(),
    /** Interface version */
    version: 1,
    /** Number of methods (requests) */
    method_count: 4,
    /** Method (request) signatures */
    methods: [
        WlMessage {
            name: b"destroy\0".as_ptr().cast(),
            signature: b"\0".as_ptr().cast(),
            wl_interface: std::ptr::null(),
        },
        WlMessage {
            name: b"create_positioner\0".as_ptr().cast(),
            signature: b"n\0".as_ptr().cast(),
            wl_interface: unsafe { &(&wl_surface_interface as *const _) },
        },
        WlMessage {
            name: b"get_xdg_surface\0".as_ptr().cast(),
            signature: b"no\0".as_ptr().cast(),
            wl_interface: &(&ZXDG_TOPLEVEL_V6_INTERFACE as *const _),
        },
        WlMessage {
            name: b"pong\0".as_ptr().cast(),
            signature: b"u\0".as_ptr().cast(),
            wl_interface: std::ptr::null(),
        },
    ]
    .as_ptr(),
    /** Number of events */
    event_count: 1,
    /** Event signatures */
    events: [WlMessage {
        name: b"ping\0".as_ptr().cast(),
        signature: b"u\0".as_ptr().cast(),
        wl_interface: std::ptr::null(),
    }]
    .as_ptr(), // *wl_message
};

unsafe extern "C" fn redraw_wl(
    c: *mut crate::Window,
    callback: *mut c_void,
    millis: u32,
) {
    let wayland = get(&mut *(*c).nwin);

    let diff_millis = if !callback.is_null() {
        wl_proxy_destroy(callback);
        if (*wayland).start_time == 0 {
            (*wayland).start_time = millis;
            0u32
        } else {
            // TODO: overflowing subtract.
            millis - (*wayland).last_millis
        }
    } else {
        0u32
    };
    assert!((*wayland).callback == callback);
    (*wayland).callback = std::ptr::null_mut();
    let orig_nanos = u64::from(diff_millis) * 1_000_000;
    (*wayland).last_millis = millis;

    let temp_nanos = orig_nanos + (*wayland).refresh_rate / 2;
    let diff_nanos = temp_nanos - (temp_nanos % (*wayland).refresh_rate);

    // Redraw on the screen.
    (*c).draw.begin_draw();
    (*c).draw_toolbar(
        &(*c).toolbar_shader,
        &mut (*c).toolbar_shape,
        &(*c).toolbar_graphic,
    );

    ((*c).redraw)(diff_nanos);

    // Get ready for next frame.
    (*wayland).callback = wl_proxy_marshal_constructor(
        (*wayland).surface,
        3, /*WL_SURFACE_FRAME*/
        &wl_callback_interface,
        std::ptr::null_mut(),
    );

    wl_proxy_add_listener(
        (*wayland).callback,
        FRAME_LISTENER.as_ptr(),
        c as *mut _ as *mut _,
    );

    // Redraw on the screen.
    (*c).draw.finish_draw();
}

static mut FRAME_LISTENER: [*mut c_void; 1] = [redraw_wl as *mut _];

#[repr(C)]
pub struct WaylandWindow {
    // Is program still running?
    pub(super) running: i32,
    // Is program restored (not fullscreen)?
    pub(super) is_restored: i32,

    // Current window width.
    pub(super) window_width: i32,
    // Current window height.
    pub(super) window_height: i32,

    // Window width to restore (exit fullscreen) to.
    pub(super) restore_width: i32,
    // Window height to restore (exit fullscreen) to.
    pub(super) restore_height: i32,

    // Millisecond counter on last frame.
    last_millis: u32,
    start_time: u32,
    // Monitor refresh rate (nanoseconds).
    refresh_rate: u64,

    // Input Information.
    pointer_xy: (f32, f32), // mouse or touch

    // NULL if not using EGL (NULL when using Vulkan + Wayland).
    pub(super) egl_window: *mut c_void, // wl_egl_window*
    pub(super) surface: *mut c_void,    // wl_surface*
    pub(super) shell_surface: *mut c_void, // wl_shell_surface*

    pub(super) callback: *mut c_void, // wl_callback*
    pub(super) configured: i32,
    pub(super) fullscreen: bool,

    pub(super) wldisplay: *mut c_void, // wl_display*
    pub(super) registry: *mut c_void,  // wl_registry*
    pub(super) compositor: *mut c_void, // wl_compositor*
    pub(super) shell: *mut c_void,     // wl_shell*
    pub(super) seat: *mut c_void,      // wl_seat*
    pub(super) pointer: *mut c_void,   // wl_pointer*
    pub(super) keyboard: *mut c_void,  // wl_keyboard*
    pub(super) shm: *mut c_void,       // wl_shm*
    pub(super) cursor_theme: *mut c_void, // wl_cursor_theme*
    pub(super) default_cursor: *mut WlCursor, // wl_cursor*
    pub(super) cursor_surface: *mut c_void, // wl_surface*
    pub(super) toplevel: *mut c_void,  // void*

    keys_states: u128,
}

impl Drop for WaylandWindow {
    fn drop(&mut self) {
        extern "C" {
            fn wl_proxy_marshal(p: *mut c_void, opcode: u32) -> ();
        }

        unsafe {
            //
            wl_surface_destroy(self.surface);
            wl_egl_window_destroy(self.egl_window);

            // Free
            wl_proxy_marshal(self.shell_surface, 0);
            wl_proxy_destroy(self.shell_surface);

            if !self.callback.is_null() {
                wl_proxy_destroy(self.callback);
            }

            // ---
            wl_surface_destroy(self.cursor_surface);
            if !self.cursor_theme.is_null() {
                wl_cursor_theme_destroy(self.cursor_theme);
            }
            if !self.shell.is_null() {
                wl_proxy_destroy(self.shell);
            }
            if !self.compositor.is_null() {
                wl_proxy_destroy(self.compositor);
            }
            wl_proxy_destroy(self.registry);
            wl_display_flush(self.wldisplay);
            wl_display_disconnect(self.wldisplay);
        }
    }
}
