// This file was generated by gir (https://github.com/gtk-rs/gir @ f5fca82)
// from gir-files (https://github.com/gtk-rs/gir-files @ b8f5ef1)
// DO NOT EDIT

#[cfg(any(feature = "v2_2", feature = "dox"))]
use Frame;
#[cfg(any(feature = "v2_2", feature = "dox"))]
use WebPage;
use ffi;
use glib;
#[cfg(any(feature = "v2_2", feature = "dox"))]
use glib::object::Downcast;
use glib::object::IsA;
#[cfg(any(feature = "v2_2", feature = "dox"))]
use glib::signal::SignalHandlerId;
#[cfg(any(feature = "v2_2", feature = "dox"))]
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
#[cfg(any(feature = "v2_2", feature = "dox"))]
use std::boxed::Box as Box_;
use std::mem;
#[cfg(any(feature = "v2_2", feature = "dox"))]
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct ScriptWorld(Object<ffi::WebKitScriptWorld, ffi::WebKitScriptWorldClass>);

    match fn {
        get_type => || ffi::webkit_script_world_get_type(),
    }
}

impl ScriptWorld {
    #[cfg(any(feature = "v2_2", feature = "dox"))]
    pub fn new() -> ScriptWorld {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::webkit_script_world_new())
        }
    }

    #[cfg(any(feature = "v2_2", feature = "dox"))]
    pub fn get_default() -> Option<ScriptWorld> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ffi::webkit_script_world_get_default())
        }
    }
}

#[cfg(any(feature = "v2_2", feature = "dox"))]
impl Default for ScriptWorld {
    fn default() -> Self {
        Self::new()
    }
}

pub trait ScriptWorldExt {
    #[cfg(any(feature = "v2_2", feature = "dox"))]
    fn connect_window_object_cleared<F: Fn(&Self, &WebPage, &Frame) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<ScriptWorld> + IsA<glib::object::Object>> ScriptWorldExt for O {
    #[cfg(any(feature = "v2_2", feature = "dox"))]
    fn connect_window_object_cleared<F: Fn(&Self, &WebPage, &Frame) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, &WebPage, &Frame) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "window-object-cleared",
                transmute(window_object_cleared_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

#[cfg(any(feature = "v2_2", feature = "dox"))]
unsafe extern "C" fn window_object_cleared_trampoline<P>(this: *mut ffi::WebKitScriptWorld, page: *mut ffi::WebKitWebPage, frame: *mut ffi::WebKitFrame, f: glib_ffi::gpointer)
where P: IsA<ScriptWorld> {
    let f: &&(Fn(&P, &WebPage, &Frame) + 'static) = transmute(f);
    f(&ScriptWorld::from_glib_borrow(this).downcast_unchecked(), &from_glib_borrow(page), &from_glib_borrow(frame))
}
