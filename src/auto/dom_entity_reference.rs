// This file was generated by gir (https://github.com/gtk-rs/gir @ 8b9d0bb)
// from gir-files (https://github.com/gtk-rs/gir-files @ 77d1f70)
// DO NOT EDIT

use DOMEventTarget;
use DOMNode;
use DOMObject;
use ffi;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct DOMEntityReference(Object<ffi::WebKitDOMEntityReference, ffi::WebKitDOMEntityReferenceClass>): DOMNode, DOMObject, DOMEventTarget;

    match fn {
        get_type => || ffi::webkit_dom_entity_reference_get_type(),
    }
}

impl DOMEntityReference {}
