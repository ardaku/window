// Window
// Copyright © 2019-2021 Jeron Aldaron Lau.
//
// Licensed under any of:
// - Apache License, Version 2.0 (https://www.apache.org/licenses/LICENSE-2.0)
// - MIT License (https://mit-license.org/)
// - Boost Software License, Version 1.0 (https://www.boost.org/LICENSE_1_0.txt)
// At your choosing (See accompanying files LICENSE_APACHE_2_0.txt,
// LICENSE_MIT.txt and LICENSE_BOOST_1_0.txt).

use std::ffi::c_void;

mod wayland;

pub(super) use self::wayland::*;

use super::Nwin;

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
