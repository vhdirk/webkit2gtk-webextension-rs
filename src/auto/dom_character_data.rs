// This file was generated by gir (https://github.com/gtk-rs/gir @ fbb95f4)
// from gir-files (https://github.com/gtk-rs/gir-files @ 77d1f70)
// DO NOT EDIT

use DOMEventTarget;
use DOMNode;
use DOMObject;
use Error;
use ffi;
use glib;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use libc;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct DOMCharacterData(Object<ffi::WebKitDOMCharacterData, ffi::WebKitDOMCharacterDataClass>): DOMNode, DOMObject, DOMEventTarget;

    match fn {
        get_type => || ffi::webkit_dom_character_data_get_type(),
    }
}

pub trait DOMCharacterDataExt {
    fn append_data(&self, data: &str) -> Result<(), Error>;

    fn delete_data(&self, offset: libc::c_ulong, length: libc::c_ulong) -> Result<(), Error>;

    fn get_data(&self) -> Option<String>;

    fn get_length(&self) -> libc::c_ulong;

    fn insert_data(&self, offset: libc::c_ulong, data: &str) -> Result<(), Error>;

    fn replace_data(&self, offset: libc::c_ulong, length: libc::c_ulong, data: &str) -> Result<(), Error>;

    fn set_data(&self, value: &str) -> Result<(), Error>;

    fn substring_data(&self, offset: libc::c_ulong, length: libc::c_ulong) -> Result<String, Error>;

    fn connect_property_data_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_length_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<DOMCharacterData> + IsA<glib::object::Object>> DOMCharacterDataExt for O {
    fn append_data(&self, data: &str) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::webkit_dom_character_data_append_data(self.to_glib_none().0, data.to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn delete_data(&self, offset: libc::c_ulong, length: libc::c_ulong) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::webkit_dom_character_data_delete_data(self.to_glib_none().0, offset, length, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn get_data(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_character_data_get_data(self.to_glib_none().0))
        }
    }

    fn get_length(&self) -> libc::c_ulong {
        unsafe {
            ffi::webkit_dom_character_data_get_length(self.to_glib_none().0)
        }
    }

    fn insert_data(&self, offset: libc::c_ulong, data: &str) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::webkit_dom_character_data_insert_data(self.to_glib_none().0, offset, data.to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn replace_data(&self, offset: libc::c_ulong, length: libc::c_ulong, data: &str) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::webkit_dom_character_data_replace_data(self.to_glib_none().0, offset, length, data.to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn set_data(&self, value: &str) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::webkit_dom_character_data_set_data(self.to_glib_none().0, value.to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn substring_data(&self, offset: libc::c_ulong, length: libc::c_ulong) -> Result<String, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::webkit_dom_character_data_substring_data(self.to_glib_none().0, offset, length, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    fn connect_property_data_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::data",
                transmute(notify_data_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_length_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::length",
                transmute(notify_length_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn notify_data_trampoline<P>(this: *mut ffi::WebKitDOMCharacterData, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMCharacterData> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMCharacterData::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_length_trampoline<P>(this: *mut ffi::WebKitDOMCharacterData, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMCharacterData> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMCharacterData::from_glib_borrow(this).downcast_unchecked())
}
