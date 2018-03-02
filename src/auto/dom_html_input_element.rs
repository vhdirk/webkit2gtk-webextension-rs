// This file was generated by gir (https://github.com/gtk-rs/gir @ fbb95f4)
// from gir-files (https://github.com/gtk-rs/gir-files @ 77d1f70)
// DO NOT EDIT

use DOMElement;
use DOMEventTarget;
use DOMFileList;
use DOMHTMLElement;
use DOMHTMLFormElement;
use DOMNode;
use DOMObject;
use Error;
use ffi;
use glib;
use glib::StaticType;
use glib::Value;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use libc;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct DOMHTMLInputElement(Object<ffi::WebKitDOMHTMLInputElement, ffi::WebKitDOMHTMLInputElementClass>): DOMHTMLElement, DOMElement, DOMNode, DOMObject, DOMEventTarget;

    match fn {
        get_type => || ffi::webkit_dom_html_input_element_get_type(),
    }
}

pub trait DOMHTMLInputElementExt {
    fn get_accept(&self) -> Option<String>;

    fn get_align(&self) -> Option<String>;

    fn get_alt(&self) -> Option<String>;

    #[cfg(any(feature = "v2_16", feature = "dox"))]
    fn get_auto_filled(&self) -> bool;

    fn get_autofocus(&self) -> bool;

    #[cfg_attr(feature = "v2_14", deprecated)]
    fn get_capture(&self) -> bool;

    #[cfg(any(feature = "v2_14", feature = "dox"))]
    fn get_capture_type(&self) -> Option<String>;

    fn get_checked(&self) -> bool;

    fn get_default_checked(&self) -> bool;

    fn get_default_value(&self) -> Option<String>;

    fn get_disabled(&self) -> bool;

    fn get_files(&self) -> Option<DOMFileList>;

    fn get_form(&self) -> Option<DOMHTMLFormElement>;

    fn get_height(&self) -> libc::c_ulong;

    fn get_indeterminate(&self) -> bool;

    fn get_input_type(&self) -> Option<String>;

    fn get_max_length(&self) -> libc::c_long;

    fn get_multiple(&self) -> bool;

    fn get_name(&self) -> Option<String>;

    #[cfg(any(feature = "v2_16", feature = "dox"))]
    fn get_read_only(&self) -> bool;

    fn get_size(&self) -> libc::c_ulong;

    fn get_src(&self) -> Option<String>;

    fn get_use_map(&self) -> Option<String>;

    fn get_value(&self) -> Option<String>;

    fn get_width(&self) -> libc::c_ulong;

    fn get_will_validate(&self) -> bool;

    fn is_edited(&self) -> bool;

    fn select(&self);

    fn set_accept(&self, value: &str);

    fn set_align(&self, value: &str);

    fn set_alt(&self, value: &str);

    #[cfg(any(feature = "v2_16", feature = "dox"))]
    fn set_auto_filled(&self, value: bool);

    fn set_autofocus(&self, value: bool);

    #[cfg(any(feature = "v2_16", feature = "dox"))]
    fn set_capture_type(&self, value: &str);

    fn set_checked(&self, value: bool);

    #[cfg(any(feature = "v2_16", feature = "dox"))]
    fn set_default_checked(&self, value: bool);

    fn set_default_value(&self, value: &str);

    fn set_disabled(&self, value: bool);

    #[cfg(any(feature = "v2_16", feature = "dox"))]
    fn set_editing_value(&self, value: &str);

    fn set_files(&self, value: &DOMFileList);

    fn set_height(&self, value: libc::c_ulong);

    fn set_indeterminate(&self, value: bool);

    fn set_input_type(&self, value: &str);

    fn set_max_length(&self, value: libc::c_long) -> Result<(), Error>;

    fn set_multiple(&self, value: bool);

    fn set_name(&self, value: &str);

    fn set_read_only(&self, value: bool);

    fn set_size(&self, value: libc::c_ulong) -> Result<(), Error>;

    fn set_src(&self, value: &str);

    fn set_use_map(&self, value: &str);

    fn set_value(&self, value: &str);

    fn set_width(&self, value: libc::c_ulong);

    fn set_property_capture(&self, capture: Option<&str>);

    fn set_property_default_checked(&self, default_checked: bool);

    fn get_property_read_only(&self) -> bool;

    fn get_property_type(&self) -> Option<String>;

    fn set_property_type(&self, type_: Option<&str>);

    fn connect_property_accept_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_align_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_alt_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_autofocus_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_capture_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_checked_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_default_checked_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_default_value_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_disabled_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_files_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_form_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_height_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_indeterminate_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_max_length_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_multiple_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_read_only_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_src_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_use_map_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_value_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_width_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_will_validate_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<DOMHTMLInputElement> + IsA<glib::object::Object>> DOMHTMLInputElementExt for O {
    fn get_accept(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_input_element_get_accept(self.to_glib_none().0))
        }
    }

    fn get_align(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_input_element_get_align(self.to_glib_none().0))
        }
    }

    fn get_alt(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_input_element_get_alt(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v2_16", feature = "dox"))]
    fn get_auto_filled(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_dom_html_input_element_get_auto_filled(self.to_glib_none().0))
        }
    }

    fn get_autofocus(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_dom_html_input_element_get_autofocus(self.to_glib_none().0))
        }
    }

    fn get_capture(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_dom_html_input_element_get_capture(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v2_14", feature = "dox"))]
    fn get_capture_type(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_input_element_get_capture_type(self.to_glib_none().0))
        }
    }

    fn get_checked(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_dom_html_input_element_get_checked(self.to_glib_none().0))
        }
    }

    fn get_default_checked(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_dom_html_input_element_get_default_checked(self.to_glib_none().0))
        }
    }

    fn get_default_value(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_input_element_get_default_value(self.to_glib_none().0))
        }
    }

    fn get_disabled(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_dom_html_input_element_get_disabled(self.to_glib_none().0))
        }
    }

    fn get_files(&self) -> Option<DOMFileList> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_input_element_get_files(self.to_glib_none().0))
        }
    }

    fn get_form(&self) -> Option<DOMHTMLFormElement> {
        unsafe {
            from_glib_none(ffi::webkit_dom_html_input_element_get_form(self.to_glib_none().0))
        }
    }

    fn get_height(&self) -> libc::c_ulong {
        unsafe {
            ffi::webkit_dom_html_input_element_get_height(self.to_glib_none().0)
        }
    }

    fn get_indeterminate(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_dom_html_input_element_get_indeterminate(self.to_glib_none().0))
        }
    }

    fn get_input_type(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_input_element_get_input_type(self.to_glib_none().0))
        }
    }

    fn get_max_length(&self) -> libc::c_long {
        unsafe {
            ffi::webkit_dom_html_input_element_get_max_length(self.to_glib_none().0)
        }
    }

    fn get_multiple(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_dom_html_input_element_get_multiple(self.to_glib_none().0))
        }
    }

    fn get_name(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_input_element_get_name(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v2_16", feature = "dox"))]
    fn get_read_only(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_dom_html_input_element_get_read_only(self.to_glib_none().0))
        }
    }

    fn get_size(&self) -> libc::c_ulong {
        unsafe {
            ffi::webkit_dom_html_input_element_get_size(self.to_glib_none().0)
        }
    }

    fn get_src(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_input_element_get_src(self.to_glib_none().0))
        }
    }

    fn get_use_map(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_input_element_get_use_map(self.to_glib_none().0))
        }
    }

    fn get_value(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_input_element_get_value(self.to_glib_none().0))
        }
    }

    fn get_width(&self) -> libc::c_ulong {
        unsafe {
            ffi::webkit_dom_html_input_element_get_width(self.to_glib_none().0)
        }
    }

    fn get_will_validate(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_dom_html_input_element_get_will_validate(self.to_glib_none().0))
        }
    }

    fn is_edited(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_dom_html_input_element_is_edited(self.to_glib_none().0))
        }
    }

    fn select(&self) {
        unsafe {
            ffi::webkit_dom_html_input_element_select(self.to_glib_none().0);
        }
    }

    fn set_accept(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_input_element_set_accept(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_align(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_input_element_set_align(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_alt(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_input_element_set_alt(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v2_16", feature = "dox"))]
    fn set_auto_filled(&self, value: bool) {
        unsafe {
            ffi::webkit_dom_html_input_element_set_auto_filled(self.to_glib_none().0, value.to_glib());
        }
    }

    fn set_autofocus(&self, value: bool) {
        unsafe {
            ffi::webkit_dom_html_input_element_set_autofocus(self.to_glib_none().0, value.to_glib());
        }
    }

    #[cfg(any(feature = "v2_16", feature = "dox"))]
    fn set_capture_type(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_input_element_set_capture_type(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_checked(&self, value: bool) {
        unsafe {
            ffi::webkit_dom_html_input_element_set_checked(self.to_glib_none().0, value.to_glib());
        }
    }

    #[cfg(any(feature = "v2_16", feature = "dox"))]
    fn set_default_checked(&self, value: bool) {
        unsafe {
            ffi::webkit_dom_html_input_element_set_default_checked(self.to_glib_none().0, value.to_glib());
        }
    }

    fn set_default_value(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_input_element_set_default_value(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_disabled(&self, value: bool) {
        unsafe {
            ffi::webkit_dom_html_input_element_set_disabled(self.to_glib_none().0, value.to_glib());
        }
    }

    #[cfg(any(feature = "v2_16", feature = "dox"))]
    fn set_editing_value(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_input_element_set_editing_value(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_files(&self, value: &DOMFileList) {
        unsafe {
            ffi::webkit_dom_html_input_element_set_files(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_height(&self, value: libc::c_ulong) {
        unsafe {
            ffi::webkit_dom_html_input_element_set_height(self.to_glib_none().0, value);
        }
    }

    fn set_indeterminate(&self, value: bool) {
        unsafe {
            ffi::webkit_dom_html_input_element_set_indeterminate(self.to_glib_none().0, value.to_glib());
        }
    }

    fn set_input_type(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_input_element_set_input_type(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_max_length(&self, value: libc::c_long) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::webkit_dom_html_input_element_set_max_length(self.to_glib_none().0, value, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn set_multiple(&self, value: bool) {
        unsafe {
            ffi::webkit_dom_html_input_element_set_multiple(self.to_glib_none().0, value.to_glib());
        }
    }

    fn set_name(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_input_element_set_name(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_read_only(&self, value: bool) {
        unsafe {
            ffi::webkit_dom_html_input_element_set_read_only(self.to_glib_none().0, value.to_glib());
        }
    }

    fn set_size(&self, value: libc::c_ulong) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::webkit_dom_html_input_element_set_size(self.to_glib_none().0, value, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn set_src(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_input_element_set_src(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_use_map(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_input_element_set_use_map(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_value(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_input_element_set_value(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_width(&self, value: libc::c_ulong) {
        unsafe {
            ffi::webkit_dom_html_input_element_set_width(self.to_glib_none().0, value);
        }
    }

    fn set_property_capture(&self, capture: Option<&str>) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "capture".to_glib_none().0, Value::from(capture).to_glib_none().0);
        }
    }

    fn set_property_default_checked(&self, default_checked: bool) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "default-checked".to_glib_none().0, Value::from(&default_checked).to_glib_none().0);
        }
    }

    fn get_property_read_only(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "read-only".to_glib_none().0, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn get_property_type(&self) -> Option<String> {
        unsafe {
            let mut value = Value::from_type(<String as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "type".to_glib_none().0, value.to_glib_none_mut().0);
            value.get()
        }
    }

    fn set_property_type(&self, type_: Option<&str>) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "type".to_glib_none().0, Value::from(type_).to_glib_none().0);
        }
    }

    fn connect_property_accept_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::accept",
                transmute(notify_accept_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_align_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::align",
                transmute(notify_align_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_alt_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::alt",
                transmute(notify_alt_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_autofocus_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::autofocus",
                transmute(notify_autofocus_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_capture_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::capture",
                transmute(notify_capture_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_checked_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::checked",
                transmute(notify_checked_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_default_checked_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::default-checked",
                transmute(notify_default_checked_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_default_value_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::default-value",
                transmute(notify_default_value_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_disabled_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::disabled",
                transmute(notify_disabled_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_files_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::files",
                transmute(notify_files_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_form_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::form",
                transmute(notify_form_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_height_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::height",
                transmute(notify_height_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_indeterminate_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::indeterminate",
                transmute(notify_indeterminate_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_max_length_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::max-length",
                transmute(notify_max_length_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_multiple_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::multiple",
                transmute(notify_multiple_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::name",
                transmute(notify_name_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_read_only_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::read-only",
                transmute(notify_read_only_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::size",
                transmute(notify_size_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_src_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::src",
                transmute(notify_src_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::type",
                transmute(notify_type_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_use_map_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::use-map",
                transmute(notify_use_map_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_value_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::value",
                transmute(notify_value_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_width_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::width",
                transmute(notify_width_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_will_validate_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::will-validate",
                transmute(notify_will_validate_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn notify_accept_trampoline<P>(this: *mut ffi::WebKitDOMHTMLInputElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLInputElement> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLInputElement::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_align_trampoline<P>(this: *mut ffi::WebKitDOMHTMLInputElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLInputElement> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLInputElement::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_alt_trampoline<P>(this: *mut ffi::WebKitDOMHTMLInputElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLInputElement> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLInputElement::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_autofocus_trampoline<P>(this: *mut ffi::WebKitDOMHTMLInputElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLInputElement> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLInputElement::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_capture_trampoline<P>(this: *mut ffi::WebKitDOMHTMLInputElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLInputElement> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLInputElement::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_checked_trampoline<P>(this: *mut ffi::WebKitDOMHTMLInputElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLInputElement> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLInputElement::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_default_checked_trampoline<P>(this: *mut ffi::WebKitDOMHTMLInputElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLInputElement> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLInputElement::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_default_value_trampoline<P>(this: *mut ffi::WebKitDOMHTMLInputElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLInputElement> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLInputElement::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_disabled_trampoline<P>(this: *mut ffi::WebKitDOMHTMLInputElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLInputElement> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLInputElement::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_files_trampoline<P>(this: *mut ffi::WebKitDOMHTMLInputElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLInputElement> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLInputElement::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_form_trampoline<P>(this: *mut ffi::WebKitDOMHTMLInputElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLInputElement> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLInputElement::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_height_trampoline<P>(this: *mut ffi::WebKitDOMHTMLInputElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLInputElement> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLInputElement::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_indeterminate_trampoline<P>(this: *mut ffi::WebKitDOMHTMLInputElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLInputElement> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLInputElement::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_max_length_trampoline<P>(this: *mut ffi::WebKitDOMHTMLInputElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLInputElement> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLInputElement::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_multiple_trampoline<P>(this: *mut ffi::WebKitDOMHTMLInputElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLInputElement> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLInputElement::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_name_trampoline<P>(this: *mut ffi::WebKitDOMHTMLInputElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLInputElement> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLInputElement::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_read_only_trampoline<P>(this: *mut ffi::WebKitDOMHTMLInputElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLInputElement> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLInputElement::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_size_trampoline<P>(this: *mut ffi::WebKitDOMHTMLInputElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLInputElement> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLInputElement::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_src_trampoline<P>(this: *mut ffi::WebKitDOMHTMLInputElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLInputElement> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLInputElement::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_type_trampoline<P>(this: *mut ffi::WebKitDOMHTMLInputElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLInputElement> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLInputElement::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_use_map_trampoline<P>(this: *mut ffi::WebKitDOMHTMLInputElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLInputElement> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLInputElement::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_value_trampoline<P>(this: *mut ffi::WebKitDOMHTMLInputElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLInputElement> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLInputElement::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_width_trampoline<P>(this: *mut ffi::WebKitDOMHTMLInputElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLInputElement> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLInputElement::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_will_validate_trampoline<P>(this: *mut ffi::WebKitDOMHTMLInputElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLInputElement> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLInputElement::from_glib_borrow(this).downcast_unchecked())
}
