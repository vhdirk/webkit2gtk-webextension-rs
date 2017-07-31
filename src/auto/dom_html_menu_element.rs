// This file was generated by gir (f00d658) from gir-files (1069259)
// DO NOT EDIT

use DOMElement;
use DOMEventTarget;
use DOMHTMLElement;
use DOMNode;
use DOMObject;
use ffi;
use glib::object::IsA;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct DOMHTMLMenuElement(Object<ffi::WebKitDOMHTMLMenuElement>): DOMHTMLElement, DOMElement, DOMNode, DOMObject, DOMEventTarget;

    match fn {
        get_type => || ffi::webkit_dom_html_menu_element_get_type(),
    }
}

pub trait DOMHTMLMenuElementExt {
    fn get_compact(&self) -> bool;

    fn set_compact(&self, value: bool);
}

impl<O: IsA<DOMHTMLMenuElement>> DOMHTMLMenuElementExt for O {
    fn get_compact(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_dom_html_menu_element_get_compact(self.to_glib_none().0))
        }
    }

    fn set_compact(&self, value: bool) {
        unsafe {
            ffi::webkit_dom_html_menu_element_set_compact(self.to_glib_none().0, value.to_glib());
        }
    }
}
