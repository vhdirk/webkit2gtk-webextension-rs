// This file was generated by gir (f00d658) from gir-files (1069259)
// DO NOT EDIT

use DOMDOMWindow;
use DOMDocument;
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
    pub struct DOMHTMLFrameElement(Object<ffi::WebKitDOMHTMLFrameElement>): DOMHTMLElement, DOMElement, DOMNode, DOMObject, DOMEventTarget;

    match fn {
        get_type => || ffi::webkit_dom_html_frame_element_get_type(),
    }
}

pub trait DOMHTMLFrameElementExt {
    fn get_content_document(&self) -> Option<DOMDocument>;

    fn get_content_window(&self) -> Option<DOMDOMWindow>;

    fn get_frame_border(&self) -> Option<String>;

    fn get_height(&self) -> libc::c_long;

    fn get_long_desc(&self) -> Option<String>;

    fn get_margin_height(&self) -> Option<String>;

    fn get_margin_width(&self) -> Option<String>;

    fn get_name(&self) -> Option<String>;

    fn get_no_resize(&self) -> bool;

    fn get_scrolling(&self) -> Option<String>;

    fn get_src(&self) -> Option<String>;

    fn get_width(&self) -> libc::c_long;

    fn set_frame_border(&self, value: &str);

    fn set_long_desc(&self, value: &str);

    fn set_margin_height(&self, value: &str);

    fn set_margin_width(&self, value: &str);

    fn set_name(&self, value: &str);

    fn set_no_resize(&self, value: bool);

    fn set_scrolling(&self, value: &str);

    fn set_src(&self, value: &str);
}

impl<O: IsA<DOMHTMLFrameElement>> DOMHTMLFrameElementExt for O {
    fn get_content_document(&self) -> Option<DOMDocument> {
        unsafe {
            from_glib_none(ffi::webkit_dom_html_frame_element_get_content_document(self.to_glib_none().0))
        }
    }

    fn get_content_window(&self) -> Option<DOMDOMWindow> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_frame_element_get_content_window(self.to_glib_none().0))
        }
    }

    fn get_frame_border(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_frame_element_get_frame_border(self.to_glib_none().0))
        }
    }

    fn get_height(&self) -> libc::c_long {
        unsafe {
            ffi::webkit_dom_html_frame_element_get_height(self.to_glib_none().0)
        }
    }

    fn get_long_desc(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_frame_element_get_long_desc(self.to_glib_none().0))
        }
    }

    fn get_margin_height(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_frame_element_get_margin_height(self.to_glib_none().0))
        }
    }

    fn get_margin_width(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_frame_element_get_margin_width(self.to_glib_none().0))
        }
    }

    fn get_name(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_frame_element_get_name(self.to_glib_none().0))
        }
    }

    fn get_no_resize(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_dom_html_frame_element_get_no_resize(self.to_glib_none().0))
        }
    }

    fn get_scrolling(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_frame_element_get_scrolling(self.to_glib_none().0))
        }
    }

    fn get_src(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_frame_element_get_src(self.to_glib_none().0))
        }
    }

    fn get_width(&self) -> libc::c_long {
        unsafe {
            ffi::webkit_dom_html_frame_element_get_width(self.to_glib_none().0)
        }
    }

    fn set_frame_border(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_frame_element_set_frame_border(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_long_desc(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_frame_element_set_long_desc(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_margin_height(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_frame_element_set_margin_height(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_margin_width(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_frame_element_set_margin_width(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_name(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_frame_element_set_name(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_no_resize(&self, value: bool) {
        unsafe {
            ffi::webkit_dom_html_frame_element_set_no_resize(self.to_glib_none().0, value.to_glib());
        }
    }

    fn set_scrolling(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_frame_element_set_scrolling(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_src(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_frame_element_set_src(self.to_glib_none().0, value.to_glib_none().0);
        }
    }
}
