// This file was generated by gir (https://github.com/gtk-rs/gir @ f5fca82)
// from gir-files (https://github.com/gtk-rs/gir-files @ b8f5ef1)
// DO NOT EDIT

use DOMElement;
use DOMEventTarget;
use DOMHTMLCollection;
use DOMHTMLElement;
use DOMHTMLTableCaptionElement;
use DOMHTMLTableSectionElement;
use DOMNode;
use DOMObject;
use Error;
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
    pub struct DOMHTMLTableElement(Object<ffi::WebKitDOMHTMLTableElement, ffi::WebKitDOMHTMLTableElementClass>): DOMHTMLElement, DOMElement, DOMNode, DOMObject, DOMEventTarget;

    match fn {
        get_type => || ffi::webkit_dom_html_table_element_get_type(),
    }
}

pub trait DOMHTMLTableElementExt {
    fn create_caption(&self) -> Option<DOMHTMLElement>;

    fn create_t_foot(&self) -> Option<DOMHTMLElement>;

    fn create_t_head(&self) -> Option<DOMHTMLElement>;

    fn delete_caption(&self);

    fn delete_row(&self, index: libc::c_long) -> Result<(), Error>;

    fn delete_t_foot(&self);

    fn delete_t_head(&self);

    fn get_align(&self) -> Option<String>;

    fn get_bg_color(&self) -> Option<String>;

    fn get_border(&self) -> Option<String>;

    fn get_caption(&self) -> Option<DOMHTMLTableCaptionElement>;

    fn get_cell_padding(&self) -> Option<String>;

    fn get_cell_spacing(&self) -> Option<String>;

    fn get_rows(&self) -> Option<DOMHTMLCollection>;

    fn get_rules(&self) -> Option<String>;

    fn get_summary(&self) -> Option<String>;

    fn get_t_bodies(&self) -> Option<DOMHTMLCollection>;

    fn get_t_foot(&self) -> Option<DOMHTMLTableSectionElement>;

    fn get_t_head(&self) -> Option<DOMHTMLTableSectionElement>;

    fn get_width(&self) -> Option<String>;

    fn insert_row(&self, index: libc::c_long) -> Result<DOMHTMLElement, Error>;

    fn set_align(&self, value: &str);

    fn set_bg_color(&self, value: &str);

    fn set_border(&self, value: &str);

    fn set_caption(&self, value: &DOMHTMLTableCaptionElement) -> Result<(), Error>;

    fn set_cell_padding(&self, value: &str);

    fn set_cell_spacing(&self, value: &str);

    fn set_rules(&self, value: &str);

    fn set_summary(&self, value: &str);

    fn set_t_foot(&self, value: &DOMHTMLTableSectionElement) -> Result<(), Error>;

    fn set_t_head(&self, value: &DOMHTMLTableSectionElement) -> Result<(), Error>;

    fn set_width(&self, value: &str);

    fn connect_property_align_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_bg_color_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_border_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_caption_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_cell_padding_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_cell_spacing_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_rows_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_rules_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_summary_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_t_bodies_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_t_foot_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_t_head_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_width_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<DOMHTMLTableElement> + IsA<glib::object::Object>> DOMHTMLTableElementExt for O {
    fn create_caption(&self) -> Option<DOMHTMLElement> {
        unsafe {
            from_glib_none(ffi::webkit_dom_html_table_element_create_caption(self.to_glib_none().0))
        }
    }

    fn create_t_foot(&self) -> Option<DOMHTMLElement> {
        unsafe {
            from_glib_none(ffi::webkit_dom_html_table_element_create_t_foot(self.to_glib_none().0))
        }
    }

    fn create_t_head(&self) -> Option<DOMHTMLElement> {
        unsafe {
            from_glib_none(ffi::webkit_dom_html_table_element_create_t_head(self.to_glib_none().0))
        }
    }

    fn delete_caption(&self) {
        unsafe {
            ffi::webkit_dom_html_table_element_delete_caption(self.to_glib_none().0);
        }
    }

    fn delete_row(&self, index: libc::c_long) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::webkit_dom_html_table_element_delete_row(self.to_glib_none().0, index, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn delete_t_foot(&self) {
        unsafe {
            ffi::webkit_dom_html_table_element_delete_t_foot(self.to_glib_none().0);
        }
    }

    fn delete_t_head(&self) {
        unsafe {
            ffi::webkit_dom_html_table_element_delete_t_head(self.to_glib_none().0);
        }
    }

    fn get_align(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_table_element_get_align(self.to_glib_none().0))
        }
    }

    fn get_bg_color(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_table_element_get_bg_color(self.to_glib_none().0))
        }
    }

    fn get_border(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_table_element_get_border(self.to_glib_none().0))
        }
    }

    fn get_caption(&self) -> Option<DOMHTMLTableCaptionElement> {
        unsafe {
            from_glib_none(ffi::webkit_dom_html_table_element_get_caption(self.to_glib_none().0))
        }
    }

    fn get_cell_padding(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_table_element_get_cell_padding(self.to_glib_none().0))
        }
    }

    fn get_cell_spacing(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_table_element_get_cell_spacing(self.to_glib_none().0))
        }
    }

    fn get_rows(&self) -> Option<DOMHTMLCollection> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_table_element_get_rows(self.to_glib_none().0))
        }
    }

    fn get_rules(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_table_element_get_rules(self.to_glib_none().0))
        }
    }

    fn get_summary(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_table_element_get_summary(self.to_glib_none().0))
        }
    }

    fn get_t_bodies(&self) -> Option<DOMHTMLCollection> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_table_element_get_t_bodies(self.to_glib_none().0))
        }
    }

    fn get_t_foot(&self) -> Option<DOMHTMLTableSectionElement> {
        unsafe {
            from_glib_none(ffi::webkit_dom_html_table_element_get_t_foot(self.to_glib_none().0))
        }
    }

    fn get_t_head(&self) -> Option<DOMHTMLTableSectionElement> {
        unsafe {
            from_glib_none(ffi::webkit_dom_html_table_element_get_t_head(self.to_glib_none().0))
        }
    }

    fn get_width(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_table_element_get_width(self.to_glib_none().0))
        }
    }

    fn insert_row(&self, index: libc::c_long) -> Result<DOMHTMLElement, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::webkit_dom_html_table_element_insert_row(self.to_glib_none().0, index, &mut error);
            if error.is_null() { Ok(from_glib_none(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    fn set_align(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_table_element_set_align(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_bg_color(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_table_element_set_bg_color(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_border(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_table_element_set_border(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_caption(&self, value: &DOMHTMLTableCaptionElement) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::webkit_dom_html_table_element_set_caption(self.to_glib_none().0, value.to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn set_cell_padding(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_table_element_set_cell_padding(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_cell_spacing(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_table_element_set_cell_spacing(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_rules(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_table_element_set_rules(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_summary(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_table_element_set_summary(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_t_foot(&self, value: &DOMHTMLTableSectionElement) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::webkit_dom_html_table_element_set_t_foot(self.to_glib_none().0, value.to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn set_t_head(&self, value: &DOMHTMLTableSectionElement) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::webkit_dom_html_table_element_set_t_head(self.to_glib_none().0, value.to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn set_width(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_table_element_set_width(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn connect_property_align_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::align",
                transmute(notify_align_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_bg_color_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::bg-color",
                transmute(notify_bg_color_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_border_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::border",
                transmute(notify_border_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_caption_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::caption",
                transmute(notify_caption_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_cell_padding_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::cell-padding",
                transmute(notify_cell_padding_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_cell_spacing_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::cell-spacing",
                transmute(notify_cell_spacing_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_rows_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::rows",
                transmute(notify_rows_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_rules_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::rules",
                transmute(notify_rules_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_summary_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::summary",
                transmute(notify_summary_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_t_bodies_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::t-bodies",
                transmute(notify_t_bodies_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_t_foot_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::t-foot",
                transmute(notify_t_foot_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_t_head_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::t-head",
                transmute(notify_t_head_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_width_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::width",
                transmute(notify_width_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn notify_align_trampoline<P>(this: *mut ffi::WebKitDOMHTMLTableElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLTableElement> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLTableElement::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_bg_color_trampoline<P>(this: *mut ffi::WebKitDOMHTMLTableElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLTableElement> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLTableElement::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_border_trampoline<P>(this: *mut ffi::WebKitDOMHTMLTableElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLTableElement> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLTableElement::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_caption_trampoline<P>(this: *mut ffi::WebKitDOMHTMLTableElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLTableElement> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLTableElement::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_cell_padding_trampoline<P>(this: *mut ffi::WebKitDOMHTMLTableElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLTableElement> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLTableElement::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_cell_spacing_trampoline<P>(this: *mut ffi::WebKitDOMHTMLTableElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLTableElement> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLTableElement::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_rows_trampoline<P>(this: *mut ffi::WebKitDOMHTMLTableElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLTableElement> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLTableElement::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_rules_trampoline<P>(this: *mut ffi::WebKitDOMHTMLTableElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLTableElement> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLTableElement::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_summary_trampoline<P>(this: *mut ffi::WebKitDOMHTMLTableElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLTableElement> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLTableElement::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_t_bodies_trampoline<P>(this: *mut ffi::WebKitDOMHTMLTableElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLTableElement> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLTableElement::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_t_foot_trampoline<P>(this: *mut ffi::WebKitDOMHTMLTableElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLTableElement> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLTableElement::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_t_head_trampoline<P>(this: *mut ffi::WebKitDOMHTMLTableElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLTableElement> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLTableElement::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_width_trampoline<P>(this: *mut ffi::WebKitDOMHTMLTableElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLTableElement> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLTableElement::from_glib_borrow(this).downcast_unchecked())
}
