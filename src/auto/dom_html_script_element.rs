// This file was generated by gir (https://github.com/gtk-rs/gir @ 8b9d0bb)
// from gir-files (https://github.com/gtk-rs/gir-files @ 77d1f70)
// DO NOT EDIT

use DOMElement;
use DOMEventTarget;
use DOMHTMLElement;
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
    pub struct DOMHTMLScriptElement(Object<ffi::WebKitDOMHTMLScriptElement, ffi::WebKitDOMHTMLScriptElementClass>): DOMHTMLElement, DOMElement, DOMNode, DOMObject, DOMEventTarget;

    match fn {
        get_type => || ffi::webkit_dom_html_script_element_get_type(),
    }
}

pub trait DOMHTMLScriptElementExt {
    fn get_charset(&self) -> Option<String>;

    fn get_defer(&self) -> bool;

    fn get_event(&self) -> Option<String>;

    fn get_html_for(&self) -> Option<String>;

    fn get_src(&self) -> Option<String>;

    fn get_text(&self) -> Option<String>;

    fn get_type_attr(&self) -> Option<String>;

    #[cfg(any(feature = "v2_16", feature = "dox"))]
    fn set_charset(&self, value: &str);

    fn set_defer(&self, value: bool);

    fn set_event(&self, value: &str);

    fn set_html_for(&self, value: &str);

    fn set_src(&self, value: &str);

    fn set_text(&self, value: &str);

    fn set_type_attr(&self, value: &str);

    fn set_property_charset(&self, charset: Option<&str>);

    fn get_property_type(&self) -> Option<String>;

    fn set_property_type(&self, type_: Option<&str>);

    fn connect_property_charset_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_defer_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_event_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_html_for_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_src_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_text_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<DOMHTMLScriptElement> + IsA<glib::object::Object>> DOMHTMLScriptElementExt for O {
    fn get_charset(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_script_element_get_charset(self.to_glib_none().0))
        }
    }

    fn get_defer(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_dom_html_script_element_get_defer(self.to_glib_none().0))
        }
    }

    fn get_event(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_script_element_get_event(self.to_glib_none().0))
        }
    }

    fn get_html_for(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_script_element_get_html_for(self.to_glib_none().0))
        }
    }

    fn get_src(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_script_element_get_src(self.to_glib_none().0))
        }
    }

    fn get_text(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_script_element_get_text(self.to_glib_none().0))
        }
    }

    fn get_type_attr(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_script_element_get_type_attr(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v2_16", feature = "dox"))]
    fn set_charset(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_script_element_set_charset(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_defer(&self, value: bool) {
        unsafe {
            ffi::webkit_dom_html_script_element_set_defer(self.to_glib_none().0, value.to_glib());
        }
    }

    fn set_event(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_script_element_set_event(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_html_for(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_script_element_set_html_for(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_src(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_script_element_set_src(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_text(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_script_element_set_text(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_type_attr(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_script_element_set_type_attr(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_property_charset(&self, charset: Option<&str>) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "charset".to_glib_none().0, Value::from(charset).to_glib_none().0);
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

    fn connect_property_charset_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::charset",
                transmute(notify_charset_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_defer_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::defer",
                transmute(notify_defer_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_event_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::event",
                transmute(notify_event_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_html_for_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::html-for",
                transmute(notify_html_for_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_src_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::src",
                transmute(notify_src_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_text_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::text",
                transmute(notify_text_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::type",
                transmute(notify_type_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn notify_charset_trampoline<P>(this: *mut ffi::WebKitDOMHTMLScriptElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLScriptElement> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLScriptElement::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_defer_trampoline<P>(this: *mut ffi::WebKitDOMHTMLScriptElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLScriptElement> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLScriptElement::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_event_trampoline<P>(this: *mut ffi::WebKitDOMHTMLScriptElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLScriptElement> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLScriptElement::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_html_for_trampoline<P>(this: *mut ffi::WebKitDOMHTMLScriptElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLScriptElement> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLScriptElement::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_src_trampoline<P>(this: *mut ffi::WebKitDOMHTMLScriptElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLScriptElement> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLScriptElement::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_text_trampoline<P>(this: *mut ffi::WebKitDOMHTMLScriptElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLScriptElement> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLScriptElement::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_type_trampoline<P>(this: *mut ffi::WebKitDOMHTMLScriptElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLScriptElement> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLScriptElement::from_glib_borrow(this).downcast_unchecked())
}
