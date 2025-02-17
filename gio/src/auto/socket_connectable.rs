// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{ffi, SocketAddressEnumerator};
use glib::{prelude::*, translate::*};

glib::wrapper! {
    #[doc(alias = "GSocketConnectable")]
    pub struct SocketConnectable(Interface<ffi::GSocketConnectable, ffi::GSocketConnectableIface>);

    match fn {
        type_ => || ffi::g_socket_connectable_get_type(),
    }
}

impl SocketConnectable {
    pub const NONE: Option<&'static SocketConnectable> = None;
}

pub trait SocketConnectableExt: IsA<SocketConnectable> + 'static {
    #[doc(alias = "g_socket_connectable_enumerate")]
    fn enumerate(&self) -> SocketAddressEnumerator {
        unsafe {
            from_glib_full(ffi::g_socket_connectable_enumerate(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "g_socket_connectable_proxy_enumerate")]
    fn proxy_enumerate(&self) -> SocketAddressEnumerator {
        unsafe {
            from_glib_full(ffi::g_socket_connectable_proxy_enumerate(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "g_socket_connectable_to_string")]
    fn to_string(&self) -> glib::GString {
        unsafe {
            from_glib_full(ffi::g_socket_connectable_to_string(
                self.as_ref().to_glib_none().0,
            ))
        }
    }
}

impl<O: IsA<SocketConnectable>> SocketConnectableExt for O {}
