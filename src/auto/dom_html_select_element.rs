// This file was generated by gir (f00d658) from gir-files (1069259)
// DO NOT EDIT

use DOMElement;
use DOMEventTarget;
use DOMHTMLElement;
use DOMHTMLFormElement;
use DOMHTMLOptionsCollection;
use DOMNode;
use DOMObject;
use Error;
use ffi;
use glib;
use glib::Value;
use glib::object::IsA;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use libc;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct DOMHTMLSelectElement(Object<ffi::WebKitDOMHTMLSelectElement>): DOMHTMLElement, DOMElement, DOMNode, DOMObject, DOMEventTarget;

    match fn {
        get_type => || ffi::webkit_dom_html_select_element_get_type(),
    }
}

pub trait DOMHTMLSelectElementExt {
    fn add<P: IsA<DOMHTMLElement>, Q: IsA<DOMHTMLElement>>(&self, element: &P, before: &Q) -> Result<(), Error>;

    fn get_autofocus(&self) -> bool;

    fn get_disabled(&self) -> bool;

    fn get_form(&self) -> Option<DOMHTMLFormElement>;

    fn get_length(&self) -> libc::c_ulong;

    fn get_multiple(&self) -> bool;

    fn get_name(&self) -> Option<String>;

    fn get_options(&self) -> Option<DOMHTMLOptionsCollection>;

    fn get_select_type(&self) -> Option<String>;

    fn get_selected_index(&self) -> libc::c_long;

    fn get_size(&self) -> libc::c_long;

    fn get_value(&self) -> Option<String>;

    fn get_will_validate(&self) -> bool;

    fn item(&self, index: libc::c_ulong) -> Option<DOMNode>;

    fn named_item(&self, name: &str) -> Option<DOMNode>;

    fn remove(&self, index: libc::c_long);

    fn set_autofocus(&self, value: bool);

    fn set_disabled(&self, value: bool);

    fn set_length(&self, value: libc::c_ulong) -> Result<(), Error>;

    fn set_multiple(&self, value: bool);

    fn set_name(&self, value: &str);

    fn set_selected_index(&self, value: libc::c_long);

    fn set_size(&self, value: libc::c_long);

    fn set_value(&self, value: &str);

    fn get_property_type(&self) -> Option<String>;
}

impl<O: IsA<DOMHTMLSelectElement> + IsA<glib::object::Object>> DOMHTMLSelectElementExt for O {
    fn add<P: IsA<DOMHTMLElement>, Q: IsA<DOMHTMLElement>>(&self, element: &P, before: &Q) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::webkit_dom_html_select_element_add(self.to_glib_none().0, element.to_glib_none().0, before.to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn get_autofocus(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_dom_html_select_element_get_autofocus(self.to_glib_none().0))
        }
    }

    fn get_disabled(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_dom_html_select_element_get_disabled(self.to_glib_none().0))
        }
    }

    fn get_form(&self) -> Option<DOMHTMLFormElement> {
        unsafe {
            from_glib_none(ffi::webkit_dom_html_select_element_get_form(self.to_glib_none().0))
        }
    }

    fn get_length(&self) -> libc::c_ulong {
        unsafe {
            ffi::webkit_dom_html_select_element_get_length(self.to_glib_none().0)
        }
    }

    fn get_multiple(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_dom_html_select_element_get_multiple(self.to_glib_none().0))
        }
    }

    fn get_name(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_select_element_get_name(self.to_glib_none().0))
        }
    }

    fn get_options(&self) -> Option<DOMHTMLOptionsCollection> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_select_element_get_options(self.to_glib_none().0))
        }
    }

    fn get_select_type(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_select_element_get_select_type(self.to_glib_none().0))
        }
    }

    fn get_selected_index(&self) -> libc::c_long {
        unsafe {
            ffi::webkit_dom_html_select_element_get_selected_index(self.to_glib_none().0)
        }
    }

    fn get_size(&self) -> libc::c_long {
        unsafe {
            ffi::webkit_dom_html_select_element_get_size(self.to_glib_none().0)
        }
    }

    fn get_value(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_select_element_get_value(self.to_glib_none().0))
        }
    }

    fn get_will_validate(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_dom_html_select_element_get_will_validate(self.to_glib_none().0))
        }
    }

    fn item(&self, index: libc::c_ulong) -> Option<DOMNode> {
        unsafe {
            from_glib_none(ffi::webkit_dom_html_select_element_item(self.to_glib_none().0, index))
        }
    }

    fn named_item(&self, name: &str) -> Option<DOMNode> {
        unsafe {
            from_glib_none(ffi::webkit_dom_html_select_element_named_item(self.to_glib_none().0, name.to_glib_none().0))
        }
    }

    fn remove(&self, index: libc::c_long) {
        unsafe {
            ffi::webkit_dom_html_select_element_remove(self.to_glib_none().0, index);
        }
    }

    fn set_autofocus(&self, value: bool) {
        unsafe {
            ffi::webkit_dom_html_select_element_set_autofocus(self.to_glib_none().0, value.to_glib());
        }
    }

    fn set_disabled(&self, value: bool) {
        unsafe {
            ffi::webkit_dom_html_select_element_set_disabled(self.to_glib_none().0, value.to_glib());
        }
    }

    fn set_length(&self, value: libc::c_ulong) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::webkit_dom_html_select_element_set_length(self.to_glib_none().0, value, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn set_multiple(&self, value: bool) {
        unsafe {
            ffi::webkit_dom_html_select_element_set_multiple(self.to_glib_none().0, value.to_glib());
        }
    }

    fn set_name(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_select_element_set_name(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_selected_index(&self, value: libc::c_long) {
        unsafe {
            ffi::webkit_dom_html_select_element_set_selected_index(self.to_glib_none().0, value);
        }
    }

    fn set_size(&self, value: libc::c_long) {
        unsafe {
            ffi::webkit_dom_html_select_element_set_size(self.to_glib_none().0, value);
        }
    }

    fn set_value(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_select_element_set_value(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn get_property_type(&self) -> Option<String> {
        let mut value = Value::from(None::<&str>);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "type".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get()
    }
}
