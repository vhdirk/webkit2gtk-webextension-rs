// This file was generated by gir (f00d658) from gir-files (1069259)
// DO NOT EDIT

use DOMElement;
use DOMEventTarget;
use DOMHTMLCollection;
use DOMHTMLElement;
use DOMNode;
use DOMObject;
use ffi;
use glib::object::IsA;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use libc;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct DOMHTMLFormElement(Object<ffi::WebKitDOMHTMLFormElement>): DOMHTMLElement, DOMElement, DOMNode, DOMObject, DOMEventTarget;

    match fn {
        get_type => || ffi::webkit_dom_html_form_element_get_type(),
    }
}

pub trait DOMHTMLFormElementExt {
    fn get_accept_charset(&self) -> Option<String>;

    fn get_action(&self) -> Option<String>;

    fn get_elements(&self) -> Option<DOMHTMLCollection>;

    fn get_encoding(&self) -> Option<String>;

    fn get_enctype(&self) -> Option<String>;

    fn get_length(&self) -> libc::c_long;

    fn get_method(&self) -> Option<String>;

    fn get_name(&self) -> Option<String>;

    fn get_target(&self) -> Option<String>;

    fn reset(&self);

    fn set_accept_charset(&self, value: &str);

    fn set_action(&self, value: &str);

    fn set_encoding(&self, value: &str);

    fn set_enctype(&self, value: &str);

    fn set_method(&self, value: &str);

    fn set_name(&self, value: &str);

    fn set_target(&self, value: &str);

    fn submit(&self);
}

impl<O: IsA<DOMHTMLFormElement>> DOMHTMLFormElementExt for O {
    fn get_accept_charset(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_form_element_get_accept_charset(self.to_glib_none().0))
        }
    }

    fn get_action(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_form_element_get_action(self.to_glib_none().0))
        }
    }

    fn get_elements(&self) -> Option<DOMHTMLCollection> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_form_element_get_elements(self.to_glib_none().0))
        }
    }

    fn get_encoding(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_form_element_get_encoding(self.to_glib_none().0))
        }
    }

    fn get_enctype(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_form_element_get_enctype(self.to_glib_none().0))
        }
    }

    fn get_length(&self) -> libc::c_long {
        unsafe {
            ffi::webkit_dom_html_form_element_get_length(self.to_glib_none().0)
        }
    }

    fn get_method(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_form_element_get_method(self.to_glib_none().0))
        }
    }

    fn get_name(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_form_element_get_name(self.to_glib_none().0))
        }
    }

    fn get_target(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_form_element_get_target(self.to_glib_none().0))
        }
    }

    fn reset(&self) {
        unsafe {
            ffi::webkit_dom_html_form_element_reset(self.to_glib_none().0);
        }
    }

    fn set_accept_charset(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_form_element_set_accept_charset(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_action(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_form_element_set_action(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_encoding(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_form_element_set_encoding(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_enctype(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_form_element_set_enctype(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_method(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_form_element_set_method(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_name(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_form_element_set_name(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_target(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_form_element_set_target(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn submit(&self) {
        unsafe {
            ffi::webkit_dom_html_form_element_submit(self.to_glib_none().0);
        }
    }
}
