// This file was generated by gir (https://github.com/gtk-rs/gir @ fbb95f4)
// from gir-files (https://github.com/gtk-rs/gir-files @ 77d1f70)
// DO NOT EDIT

use DOMElement;
use DOMEventTarget;
use DOMHTMLElement;
use DOMNode;
use DOMObject;
use ffi;
use glib;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct DOMHTMLModElement(Object<ffi::WebKitDOMHTMLModElement, ffi::WebKitDOMHTMLModElementClass>): DOMHTMLElement, DOMElement, DOMNode, DOMObject, DOMEventTarget;

    match fn {
        get_type => || ffi::webkit_dom_html_mod_element_get_type(),
    }
}

pub trait DOMHTMLModElementExt {
    fn get_cite(&self) -> Option<String>;

    fn get_date_time(&self) -> Option<String>;

    fn set_cite(&self, value: &str);

    fn set_date_time(&self, value: &str);

    fn connect_property_cite_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_date_time_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<DOMHTMLModElement> + IsA<glib::object::Object>> DOMHTMLModElementExt for O {
    fn get_cite(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_mod_element_get_cite(self.to_glib_none().0))
        }
    }

    fn get_date_time(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_mod_element_get_date_time(self.to_glib_none().0))
        }
    }

    fn set_cite(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_mod_element_set_cite(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_date_time(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_mod_element_set_date_time(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn connect_property_cite_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::cite",
                transmute(notify_cite_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_date_time_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::date-time",
                transmute(notify_date_time_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn notify_cite_trampoline<P>(this: *mut ffi::WebKitDOMHTMLModElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLModElement> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLModElement::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_date_time_trampoline<P>(this: *mut ffi::WebKitDOMHTMLModElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLModElement> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLModElement::from_glib_borrow(this).downcast_unchecked())
}
