// This file was generated by gir (0f7cd61) from gir-files (857b8f5)
// DO NOT EDIT

use DOMElement;
use DOMEventTarget;
use DOMHTMLElement;
use DOMHTMLFormElement;
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
    pub struct DOMHTMLLabelElement(Object<ffi::WebKitDOMHTMLLabelElement>): DOMHTMLElement, DOMElement, DOMNode, DOMObject, DOMEventTarget;

    match fn {
        get_type => || ffi::webkit_dom_html_label_element_get_type(),
    }
}

pub trait DOMHTMLLabelElementExt {
    fn get_form(&self) -> Option<DOMHTMLFormElement>;

    fn get_html_for(&self) -> Option<String>;

    fn set_html_for(&self, value: &str);
}

impl<O: IsA<DOMHTMLLabelElement>> DOMHTMLLabelElementExt for O {
    fn get_form(&self) -> Option<DOMHTMLFormElement> {
        unsafe {
            from_glib_none(ffi::webkit_dom_html_label_element_get_form(self.to_glib_none().0))
        }
    }

    fn get_html_for(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_label_element_get_html_for(self.to_glib_none().0))
        }
    }

    fn set_html_for(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_label_element_set_html_for(self.to_glib_none().0, value.to_glib_none().0);
        }
    }
}
