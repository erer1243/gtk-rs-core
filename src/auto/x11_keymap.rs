// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use gdk;
use gdk_x11_sys;
use glib::translate::*;
use std::fmt;

glib_wrapper! {
    pub struct X11Keymap(Object<gdk_x11_sys::GdkX11Keymap, gdk_x11_sys::GdkX11KeymapClass, X11KeymapClass>) @extends gdk::Keymap;

    match fn {
        get_type => || gdk_x11_sys::gdk_x11_keymap_get_type(),
    }
}

impl X11Keymap {
    pub fn get_group_for_state(&self, state: u32) -> i32 {
        unsafe { gdk_x11_sys::gdk_x11_keymap_get_group_for_state(self.to_glib_none().0, state) }
    }

    pub fn key_is_modifier(&self, keycode: u32) -> bool {
        unsafe {
            from_glib(gdk_x11_sys::gdk_x11_keymap_key_is_modifier(
                self.to_glib_none().0,
                keycode,
            ))
        }
    }
}

impl fmt::Display for X11Keymap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "X11Keymap")
    }
}
