// This file was generated by gir (https://github.com/gtk-rs/gir @ f5fca82)
// from gir-files (https://github.com/gtk-rs/gir-files @ b8f5ef1)
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
use libc;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct DOMHTMLImageElement(Object<ffi::WebKitDOMHTMLImageElement, ffi::WebKitDOMHTMLImageElementClass>): DOMHTMLElement, DOMElement, DOMNode, DOMObject, DOMEventTarget;

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

    fn connect_property_align_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_alt_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_border_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_complete_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_height_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_hspace_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_is_map_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_long_desc_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_lowsrc_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_natural_height_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_natural_width_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_src_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_use_map_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_vspace_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_width_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_x_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_y_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<DOMHTMLImageElement> + IsA<glib::object::Object>> DOMHTMLImageElementExt for O {
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

    fn connect_property_border_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::border",
                transmute(notify_border_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_complete_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::complete",
                transmute(notify_complete_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_height_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::height",
                transmute(notify_height_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_hspace_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::hspace",
                transmute(notify_hspace_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_is_map_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::is-map",
                transmute(notify_is_map_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_long_desc_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::long-desc",
                transmute(notify_long_desc_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_lowsrc_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::lowsrc",
                transmute(notify_lowsrc_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::name",
                transmute(notify_name_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_natural_height_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::natural-height",
                transmute(notify_natural_height_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_natural_width_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::natural-width",
                transmute(notify_natural_width_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_src_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::src",
                transmute(notify_src_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_use_map_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::use-map",
                transmute(notify_use_map_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_vspace_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::vspace",
                transmute(notify_vspace_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_width_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::width",
                transmute(notify_width_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_x_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::x",
                transmute(notify_x_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_y_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::y",
                transmute(notify_y_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn notify_align_trampoline<P>(this: *mut ffi::WebKitDOMHTMLImageElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLImageElement> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLImageElement::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_alt_trampoline<P>(this: *mut ffi::WebKitDOMHTMLImageElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLImageElement> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLImageElement::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_border_trampoline<P>(this: *mut ffi::WebKitDOMHTMLImageElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLImageElement> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLImageElement::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_complete_trampoline<P>(this: *mut ffi::WebKitDOMHTMLImageElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLImageElement> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLImageElement::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_height_trampoline<P>(this: *mut ffi::WebKitDOMHTMLImageElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLImageElement> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLImageElement::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_hspace_trampoline<P>(this: *mut ffi::WebKitDOMHTMLImageElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLImageElement> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLImageElement::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_is_map_trampoline<P>(this: *mut ffi::WebKitDOMHTMLImageElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLImageElement> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLImageElement::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_long_desc_trampoline<P>(this: *mut ffi::WebKitDOMHTMLImageElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLImageElement> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLImageElement::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_lowsrc_trampoline<P>(this: *mut ffi::WebKitDOMHTMLImageElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLImageElement> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLImageElement::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_name_trampoline<P>(this: *mut ffi::WebKitDOMHTMLImageElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLImageElement> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLImageElement::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_natural_height_trampoline<P>(this: *mut ffi::WebKitDOMHTMLImageElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLImageElement> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLImageElement::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_natural_width_trampoline<P>(this: *mut ffi::WebKitDOMHTMLImageElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLImageElement> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLImageElement::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_src_trampoline<P>(this: *mut ffi::WebKitDOMHTMLImageElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLImageElement> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLImageElement::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_use_map_trampoline<P>(this: *mut ffi::WebKitDOMHTMLImageElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLImageElement> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLImageElement::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_vspace_trampoline<P>(this: *mut ffi::WebKitDOMHTMLImageElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLImageElement> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLImageElement::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_width_trampoline<P>(this: *mut ffi::WebKitDOMHTMLImageElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLImageElement> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLImageElement::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_x_trampoline<P>(this: *mut ffi::WebKitDOMHTMLImageElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLImageElement> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLImageElement::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_y_trampoline<P>(this: *mut ffi::WebKitDOMHTMLImageElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLImageElement> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLImageElement::from_glib_borrow(this).downcast_unchecked())
}
