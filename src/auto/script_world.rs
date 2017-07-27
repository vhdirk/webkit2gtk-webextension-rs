// This file was generated by gir (0f7cd61) from gir-files (857b8f5)
// DO NOT EDIT

#[cfg(feature = "v2_2")]
use Frame;
#[cfg(feature = "v2_2")]
use WebPage;
use ffi;
use glib;
#[cfg(feature = "v2_2")]
use glib::object::Downcast;
use glib::object::IsA;
#[cfg(feature = "v2_2")]
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
#[cfg(feature = "v2_2")]
use std::boxed::Box as Box_;
use std::mem;
#[cfg(feature = "v2_2")]
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct ScriptWorld(Object<ffi::WebKitScriptWorld>);

    match fn {
        get_type => || ffi::webkit_script_world_get_type(),
    }
}

impl ScriptWorld {
    #[cfg(feature = "v2_2")]
    pub fn new() -> ScriptWorld {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::webkit_script_world_new())
        }
    }

    #[cfg(feature = "v2_2")]
    pub fn get_default() -> Option<ScriptWorld> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ffi::webkit_script_world_get_default())
        }
    }
}

pub trait ScriptWorldExt {
    #[cfg(feature = "v2_2")]
    fn connect_window_object_cleared<F: Fn(&Self, &WebPage, &Frame) + 'static>(&self, f: F) -> u64;
}

impl<O: IsA<ScriptWorld> + IsA<glib::object::Object>> ScriptWorldExt for O {
    #[cfg(feature = "v2_2")]
    fn connect_window_object_cleared<F: Fn(&Self, &WebPage, &Frame) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self, &WebPage, &Frame) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "window-object-cleared",
                transmute(window_object_cleared_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

#[cfg(feature = "v2_2")]
unsafe extern "C" fn window_object_cleared_trampoline<P>(this: *mut ffi::WebKitScriptWorld, page: *mut ffi::WebKitWebPage, frame: *mut ffi::WebKitFrame, f: glib_ffi::gpointer)
where P: IsA<ScriptWorld> {
    callback_guard!();
    let f: &Box_<Fn(&P, &WebPage, &Frame) + 'static> = transmute(f);
    f(&ScriptWorld::from_glib_none(this).downcast_unchecked(), &from_glib_none(page), &from_glib_none(frame))
}
