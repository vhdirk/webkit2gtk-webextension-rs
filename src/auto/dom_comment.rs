// This file was generated by gir (7484d29) from gir-files (71d73f0)
// DO NOT EDIT

use DOMCharacterData;
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
    pub struct DOMComment(Object<ffi::WebKitDOMComment>): DOMCharacterData, DOMNode, DOMObject, DOMEventTarget;

    match fn {
        get_type => || ffi::webkit_dom_comment_get_type(),
    }
}

impl DOMComment {}
