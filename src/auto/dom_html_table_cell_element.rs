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
    pub struct DOMHTMLTableCellElement(Object<ffi::WebKitDOMHTMLTableCellElement, ffi::WebKitDOMHTMLTableCellElementClass>): DOMHTMLElement, DOMElement, DOMNode, DOMObject, DOMEventTarget;

    match fn {
        get_type => || ffi::webkit_dom_html_table_cell_element_get_type(),
    }
}

pub trait DOMHTMLTableCellElementExt {
    fn get_abbr(&self) -> Option<String>;

    fn get_align(&self) -> Option<String>;

    fn get_axis(&self) -> Option<String>;

    fn get_bg_color(&self) -> Option<String>;

    fn get_cell_index(&self) -> libc::c_long;

    fn get_ch(&self) -> Option<String>;

    fn get_ch_off(&self) -> Option<String>;

    fn get_col_span(&self) -> libc::c_long;

    fn get_headers(&self) -> Option<String>;

    fn get_height(&self) -> Option<String>;

    fn get_no_wrap(&self) -> bool;

    fn get_row_span(&self) -> libc::c_long;

    fn get_scope(&self) -> Option<String>;

    fn get_v_align(&self) -> Option<String>;

    fn get_width(&self) -> Option<String>;

    fn set_abbr(&self, value: &str);

    fn set_align(&self, value: &str);

    fn set_axis(&self, value: &str);

    fn set_bg_color(&self, value: &str);

    fn set_ch(&self, value: &str);

    fn set_ch_off(&self, value: &str);

    fn set_col_span(&self, value: libc::c_long);

    fn set_headers(&self, value: &str);

    fn set_height(&self, value: &str);

    fn set_no_wrap(&self, value: bool);

    fn set_row_span(&self, value: libc::c_long);

    fn set_scope(&self, value: &str);

    fn set_v_align(&self, value: &str);

    fn set_width(&self, value: &str);

    fn connect_property_abbr_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_align_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_axis_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_bg_color_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_cell_index_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_ch_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_ch_off_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_col_span_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_headers_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_height_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_no_wrap_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_row_span_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_scope_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_v_align_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_width_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<DOMHTMLTableCellElement> + IsA<glib::object::Object>> DOMHTMLTableCellElementExt for O {
    fn get_abbr(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_table_cell_element_get_abbr(self.to_glib_none().0))
        }
    }

    fn get_align(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_table_cell_element_get_align(self.to_glib_none().0))
        }
    }

    fn get_axis(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_table_cell_element_get_axis(self.to_glib_none().0))
        }
    }

    fn get_bg_color(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_table_cell_element_get_bg_color(self.to_glib_none().0))
        }
    }

    fn get_cell_index(&self) -> libc::c_long {
        unsafe {
            ffi::webkit_dom_html_table_cell_element_get_cell_index(self.to_glib_none().0)
        }
    }

    fn get_ch(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_table_cell_element_get_ch(self.to_glib_none().0))
        }
    }

    fn get_ch_off(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_table_cell_element_get_ch_off(self.to_glib_none().0))
        }
    }

    fn get_col_span(&self) -> libc::c_long {
        unsafe {
            ffi::webkit_dom_html_table_cell_element_get_col_span(self.to_glib_none().0)
        }
    }

    fn get_headers(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_table_cell_element_get_headers(self.to_glib_none().0))
        }
    }

    fn get_height(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_table_cell_element_get_height(self.to_glib_none().0))
        }
    }

    fn get_no_wrap(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_dom_html_table_cell_element_get_no_wrap(self.to_glib_none().0))
        }
    }

    fn get_row_span(&self) -> libc::c_long {
        unsafe {
            ffi::webkit_dom_html_table_cell_element_get_row_span(self.to_glib_none().0)
        }
    }

    fn get_scope(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_table_cell_element_get_scope(self.to_glib_none().0))
        }
    }

    fn get_v_align(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_table_cell_element_get_v_align(self.to_glib_none().0))
        }
    }

    fn get_width(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_table_cell_element_get_width(self.to_glib_none().0))
        }
    }

    fn set_abbr(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_table_cell_element_set_abbr(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_align(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_table_cell_element_set_align(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_axis(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_table_cell_element_set_axis(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_bg_color(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_table_cell_element_set_bg_color(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_ch(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_table_cell_element_set_ch(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_ch_off(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_table_cell_element_set_ch_off(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_col_span(&self, value: libc::c_long) {
        unsafe {
            ffi::webkit_dom_html_table_cell_element_set_col_span(self.to_glib_none().0, value);
        }
    }

    fn set_headers(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_table_cell_element_set_headers(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_height(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_table_cell_element_set_height(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_no_wrap(&self, value: bool) {
        unsafe {
            ffi::webkit_dom_html_table_cell_element_set_no_wrap(self.to_glib_none().0, value.to_glib());
        }
    }

    fn set_row_span(&self, value: libc::c_long) {
        unsafe {
            ffi::webkit_dom_html_table_cell_element_set_row_span(self.to_glib_none().0, value);
        }
    }

    fn set_scope(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_table_cell_element_set_scope(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_v_align(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_table_cell_element_set_v_align(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_width(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_table_cell_element_set_width(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn connect_property_abbr_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::abbr",
                transmute(notify_abbr_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_align_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::align",
                transmute(notify_align_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_axis_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::axis",
                transmute(notify_axis_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_bg_color_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::bg-color",
                transmute(notify_bg_color_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_cell_index_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::cell-index",
                transmute(notify_cell_index_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_ch_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::ch",
                transmute(notify_ch_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_ch_off_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::ch-off",
                transmute(notify_ch_off_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_col_span_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::col-span",
                transmute(notify_col_span_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_headers_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::headers",
                transmute(notify_headers_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_height_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::height",
                transmute(notify_height_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_no_wrap_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::no-wrap",
                transmute(notify_no_wrap_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_row_span_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::row-span",
                transmute(notify_row_span_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_scope_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::scope",
                transmute(notify_scope_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_v_align_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::v-align",
                transmute(notify_v_align_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
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

unsafe extern "C" fn notify_abbr_trampoline<P>(this: *mut ffi::WebKitDOMHTMLTableCellElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLTableCellElement> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLTableCellElement::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_align_trampoline<P>(this: *mut ffi::WebKitDOMHTMLTableCellElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLTableCellElement> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLTableCellElement::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_axis_trampoline<P>(this: *mut ffi::WebKitDOMHTMLTableCellElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLTableCellElement> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLTableCellElement::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_bg_color_trampoline<P>(this: *mut ffi::WebKitDOMHTMLTableCellElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLTableCellElement> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLTableCellElement::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_cell_index_trampoline<P>(this: *mut ffi::WebKitDOMHTMLTableCellElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLTableCellElement> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLTableCellElement::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_ch_trampoline<P>(this: *mut ffi::WebKitDOMHTMLTableCellElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLTableCellElement> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLTableCellElement::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_ch_off_trampoline<P>(this: *mut ffi::WebKitDOMHTMLTableCellElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLTableCellElement> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLTableCellElement::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_col_span_trampoline<P>(this: *mut ffi::WebKitDOMHTMLTableCellElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLTableCellElement> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLTableCellElement::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_headers_trampoline<P>(this: *mut ffi::WebKitDOMHTMLTableCellElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLTableCellElement> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLTableCellElement::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_height_trampoline<P>(this: *mut ffi::WebKitDOMHTMLTableCellElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLTableCellElement> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLTableCellElement::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_no_wrap_trampoline<P>(this: *mut ffi::WebKitDOMHTMLTableCellElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLTableCellElement> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLTableCellElement::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_row_span_trampoline<P>(this: *mut ffi::WebKitDOMHTMLTableCellElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLTableCellElement> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLTableCellElement::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_scope_trampoline<P>(this: *mut ffi::WebKitDOMHTMLTableCellElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLTableCellElement> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLTableCellElement::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_v_align_trampoline<P>(this: *mut ffi::WebKitDOMHTMLTableCellElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLTableCellElement> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLTableCellElement::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_width_trampoline<P>(this: *mut ffi::WebKitDOMHTMLTableCellElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLTableCellElement> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLTableCellElement::from_glib_borrow(this).downcast_unchecked())
}
