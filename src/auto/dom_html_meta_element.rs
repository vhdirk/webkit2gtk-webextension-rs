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
    pub struct DOMHTMLMetaElement(Object<ffi::WebKitDOMHTMLMetaElement>): DOMHTMLElement, DOMElement, DOMNode, DOMObject, DOMEventTarget;

    match fn {
        get_type => || ffi::webkit_dom_html_meta_element_get_type(),
    }
}

pub trait DOMHTMLMetaElementExt {
    fn get_content(&self) -> Option<String>;

    fn get_http_equiv(&self) -> Option<String>;

    fn get_name(&self) -> Option<String>;

    fn get_scheme(&self) -> Option<String>;

    fn set_content(&self, value: &str);

    fn set_http_equiv(&self, value: &str);

    fn set_name(&self, value: &str);

    fn set_scheme(&self, value: &str);
}

impl<O: IsA<DOMHTMLMetaElement>> DOMHTMLMetaElementExt for O {
    fn get_content(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_meta_element_get_content(self.to_glib_none().0))
        }
    }

    fn get_http_equiv(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_meta_element_get_http_equiv(self.to_glib_none().0))
        }
    }

    fn get_name(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_meta_element_get_name(self.to_glib_none().0))
        }
    }

    fn get_scheme(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_meta_element_get_scheme(self.to_glib_none().0))
        }
    }

    fn set_content(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_meta_element_set_content(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_http_equiv(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_meta_element_set_http_equiv(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_name(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_meta_element_set_name(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_scheme(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_meta_element_set_scheme(self.to_glib_none().0, value.to_glib_none().0);
        }
    }
}
