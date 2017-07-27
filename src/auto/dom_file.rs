// This file was generated by gir (0f7cd61) from gir-files (857b8f5)
// DO NOT EDIT

use DOMBlob;
use DOMObject;
use ffi;
use glib::object::IsA;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct DOMFile(Object<ffi::WebKitDOMFile>): DOMBlob, DOMObject;

    match fn {
        get_type => || ffi::webkit_dom_file_get_type(),
    }
}

pub trait DOMFileExt {
    fn get_name(&self) -> Option<String>;
}

impl<O: IsA<DOMFile>> DOMFileExt for O {
    fn get_name(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_file_get_name(self.to_glib_none().0))
        }
    }
}
