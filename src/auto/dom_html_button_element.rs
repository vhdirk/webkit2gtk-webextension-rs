// This file was generated by gir (https://github.com/gtk-rs/gir @ fbb95f4)
// from gir-files (https://github.com/gtk-rs/gir-files @ 77d1f70)
// DO NOT EDIT

use DOMElement;
use DOMEventTarget;
use DOMHTMLElement;
use DOMHTMLFormElement;
use DOMNode;
use DOMObject;
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
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct DOMHTMLButtonElement(Object<ffi::WebKitDOMHTMLButtonElement, ffi::WebKitDOMHTMLButtonElementClass>): DOMHTMLElement, DOMElement, DOMNode, DOMObject, DOMEventTarget;

    match fn {
        get_type => || ffi::webkit_dom_html_button_element_get_type(),
    }
}

pub trait DOMHTMLButtonElementExt {
    fn get_autofocus(&self) -> bool;

    fn get_button_type(&self) -> Option<String>;

    fn get_disabled(&self) -> bool;

    fn get_form(&self) -> Option<DOMHTMLFormElement>;

    fn get_name(&self) -> Option<String>;

    fn get_value(&self) -> Option<String>;

    fn get_will_validate(&self) -> bool;

    fn set_autofocus(&self, value: bool);

    fn set_button_type(&self, value: &str);

    fn set_disabled(&self, value: bool);

    fn set_name(&self, value: &str);

    fn set_value(&self, value: &str);

    fn get_property_type(&self) -> Option<String>;

    fn set_property_type(&self, type_: Option<&str>);

    fn connect_property_autofocus_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_disabled_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_form_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_value_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_will_validate_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<DOMHTMLButtonElement> + IsA<glib::object::Object>> DOMHTMLButtonElementExt for O {
    fn get_autofocus(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_dom_html_button_element_get_autofocus(self.to_glib_none().0))
        }
    }

    fn get_button_type(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_button_element_get_button_type(self.to_glib_none().0))
        }
    }

    fn get_disabled(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_dom_html_button_element_get_disabled(self.to_glib_none().0))
        }
    }

    fn get_form(&self) -> Option<DOMHTMLFormElement> {
        unsafe {
            from_glib_none(ffi::webkit_dom_html_button_element_get_form(self.to_glib_none().0))
        }
    }

    fn get_name(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_button_element_get_name(self.to_glib_none().0))
        }
    }

    fn get_value(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_button_element_get_value(self.to_glib_none().0))
        }
    }

    fn get_will_validate(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_dom_html_button_element_get_will_validate(self.to_glib_none().0))
        }
    }

    fn set_autofocus(&self, value: bool) {
        unsafe {
            ffi::webkit_dom_html_button_element_set_autofocus(self.to_glib_none().0, value.to_glib());
        }
    }

    fn set_button_type(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_button_element_set_button_type(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_disabled(&self, value: bool) {
        unsafe {
            ffi::webkit_dom_html_button_element_set_disabled(self.to_glib_none().0, value.to_glib());
        }
    }

    fn set_name(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_button_element_set_name(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_value(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_button_element_set_value(self.to_glib_none().0, value.to_glib_none().0);
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

    fn connect_property_autofocus_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::autofocus",
                transmute(notify_autofocus_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_disabled_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::disabled",
                transmute(notify_disabled_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_form_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::form",
                transmute(notify_form_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::name",
                transmute(notify_name_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::type",
                transmute(notify_type_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_value_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::value",
                transmute(notify_value_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
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

unsafe extern "C" fn notify_autofocus_trampoline<P>(this: *mut ffi::WebKitDOMHTMLButtonElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLButtonElement> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLButtonElement::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_disabled_trampoline<P>(this: *mut ffi::WebKitDOMHTMLButtonElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLButtonElement> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLButtonElement::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_form_trampoline<P>(this: *mut ffi::WebKitDOMHTMLButtonElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLButtonElement> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLButtonElement::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_name_trampoline<P>(this: *mut ffi::WebKitDOMHTMLButtonElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLButtonElement> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLButtonElement::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_type_trampoline<P>(this: *mut ffi::WebKitDOMHTMLButtonElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLButtonElement> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLButtonElement::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_value_trampoline<P>(this: *mut ffi::WebKitDOMHTMLButtonElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLButtonElement> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLButtonElement::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_will_validate_trampoline<P>(this: *mut ffi::WebKitDOMHTMLButtonElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLButtonElement> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLButtonElement::from_glib_borrow(this).downcast_unchecked())
}
