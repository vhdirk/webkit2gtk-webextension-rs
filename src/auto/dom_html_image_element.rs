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
use libc;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct DOMHTMLImageElement(Object<ffi::WebKitDOMHTMLImageElement>): DOMHTMLElement, DOMElement, DOMNode, DOMObject, DOMEventTarget;

    match fn {
        get_type => || ffi::webkit_dom_html_image_element_get_type(),
    }
}

pub trait DOMHTMLImageElementExt {
    fn get_align(&self) -> Option<String>;

    fn get_alt(&self) -> Option<String>;

    fn get_border(&self) -> Option<String>;

    fn get_complete(&self) -> bool;

    fn get_height(&self) -> libc::c_long;

    fn get_hspace(&self) -> libc::c_long;

    fn get_is_map(&self) -> bool;

    fn get_long_desc(&self) -> Option<String>;

    fn get_lowsrc(&self) -> Option<String>;

    fn get_name(&self) -> Option<String>;

    fn get_natural_height(&self) -> libc::c_long;

    fn get_natural_width(&self) -> libc::c_long;

    fn get_src(&self) -> Option<String>;

    fn get_use_map(&self) -> Option<String>;

    fn get_vspace(&self) -> libc::c_long;

    fn get_width(&self) -> libc::c_long;

    fn get_x(&self) -> libc::c_long;

    fn get_y(&self) -> libc::c_long;

    fn set_align(&self, value: &str);

    fn set_alt(&self, value: &str);

    fn set_border(&self, value: &str);

    fn set_height(&self, value: libc::c_long);

    fn set_hspace(&self, value: libc::c_long);

    fn set_is_map(&self, value: bool);

    fn set_long_desc(&self, value: &str);

    fn set_lowsrc(&self, value: &str);

    fn set_name(&self, value: &str);

    fn set_src(&self, value: &str);

    fn set_use_map(&self, value: &str);

    fn set_vspace(&self, value: libc::c_long);

    fn set_width(&self, value: libc::c_long);
}

impl<O: IsA<DOMHTMLImageElement>> DOMHTMLImageElementExt for O {
    fn get_align(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_image_element_get_align(self.to_glib_none().0))
        }
    }

    fn get_alt(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_image_element_get_alt(self.to_glib_none().0))
        }
    }

    fn get_border(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_image_element_get_border(self.to_glib_none().0))
        }
    }

    fn get_complete(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_dom_html_image_element_get_complete(self.to_glib_none().0))
        }
    }

    fn get_height(&self) -> libc::c_long {
        unsafe {
            ffi::webkit_dom_html_image_element_get_height(self.to_glib_none().0)
        }
    }

    fn get_hspace(&self) -> libc::c_long {
        unsafe {
            ffi::webkit_dom_html_image_element_get_hspace(self.to_glib_none().0)
        }
    }

    fn get_is_map(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_dom_html_image_element_get_is_map(self.to_glib_none().0))
        }
    }

    fn get_long_desc(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_image_element_get_long_desc(self.to_glib_none().0))
        }
    }

    fn get_lowsrc(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_image_element_get_lowsrc(self.to_glib_none().0))
        }
    }

    fn get_name(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_image_element_get_name(self.to_glib_none().0))
        }
    }

    fn get_natural_height(&self) -> libc::c_long {
        unsafe {
            ffi::webkit_dom_html_image_element_get_natural_height(self.to_glib_none().0)
        }
    }

    fn get_natural_width(&self) -> libc::c_long {
        unsafe {
            ffi::webkit_dom_html_image_element_get_natural_width(self.to_glib_none().0)
        }
    }

    fn get_src(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_image_element_get_src(self.to_glib_none().0))
        }
    }

    fn get_use_map(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_image_element_get_use_map(self.to_glib_none().0))
        }
    }

    fn get_vspace(&self) -> libc::c_long {
        unsafe {
            ffi::webkit_dom_html_image_element_get_vspace(self.to_glib_none().0)
        }
    }

    fn get_width(&self) -> libc::c_long {
        unsafe {
            ffi::webkit_dom_html_image_element_get_width(self.to_glib_none().0)
        }
    }

    fn get_x(&self) -> libc::c_long {
        unsafe {
            ffi::webkit_dom_html_image_element_get_x(self.to_glib_none().0)
        }
    }

    fn get_y(&self) -> libc::c_long {
        unsafe {
            ffi::webkit_dom_html_image_element_get_y(self.to_glib_none().0)
        }
    }

    fn set_align(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_image_element_set_align(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_alt(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_image_element_set_alt(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_border(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_image_element_set_border(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_height(&self, value: libc::c_long) {
        unsafe {
            ffi::webkit_dom_html_image_element_set_height(self.to_glib_none().0, value);
        }
    }

    fn set_hspace(&self, value: libc::c_long) {
        unsafe {
            ffi::webkit_dom_html_image_element_set_hspace(self.to_glib_none().0, value);
        }
    }

    fn set_is_map(&self, value: bool) {
        unsafe {
            ffi::webkit_dom_html_image_element_set_is_map(self.to_glib_none().0, value.to_glib());
        }
    }

    fn set_long_desc(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_image_element_set_long_desc(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_lowsrc(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_image_element_set_lowsrc(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_name(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_image_element_set_name(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_src(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_image_element_set_src(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_use_map(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_image_element_set_use_map(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_vspace(&self, value: libc::c_long) {
        unsafe {
            ffi::webkit_dom_html_image_element_set_vspace(self.to_glib_none().0, value);
        }
    }

    fn set_width(&self, value: libc::c_long) {
        unsafe {
            ffi::webkit_dom_html_image_element_set_width(self.to_glib_none().0, value);
        }
    }
}
