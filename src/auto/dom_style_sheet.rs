// This file was generated by gir (https://github.com/gtk-rs/gir @ 8b9d0bb)
// from gir-files (https://github.com/gtk-rs/gir-files @ 77d1f70)
// DO NOT EDIT

use DOMMediaList;
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
    pub struct DOMStyleSheet(Object<ffi::WebKitDOMStyleSheet, ffi::WebKitDOMStyleSheetClass>): DOMObject;

    match fn {
        get_type => || ffi::webkit_dom_style_sheet_get_type(),
    }
}

pub trait DOMStyleSheetExt {
    fn get_content_type(&self) -> Option<String>;

    fn get_disabled(&self) -> bool;

    fn get_href(&self) -> Option<String>;

    fn get_media(&self) -> Option<DOMMediaList>;

    fn get_owner_node(&self) -> Option<DOMNode>;

    fn get_parent_style_sheet(&self) -> Option<DOMStyleSheet>;

    fn get_title(&self) -> Option<String>;

    fn set_disabled(&self, value: bool);

    fn get_property_type(&self) -> Option<String>;

    fn connect_property_disabled_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_href_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_media_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_owner_node_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_parent_style_sheet_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_title_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<DOMStyleSheet> + IsA<glib::object::Object>> DOMStyleSheetExt for O {
    fn get_content_type(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_style_sheet_get_content_type(self.to_glib_none().0))
        }
    }

    fn get_disabled(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_dom_style_sheet_get_disabled(self.to_glib_none().0))
        }
    }

    fn get_href(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_style_sheet_get_href(self.to_glib_none().0))
        }
    }

    fn get_media(&self) -> Option<DOMMediaList> {
        unsafe {
            from_glib_full(ffi::webkit_dom_style_sheet_get_media(self.to_glib_none().0))
        }
    }

    fn get_owner_node(&self) -> Option<DOMNode> {
        unsafe {
            from_glib_none(ffi::webkit_dom_style_sheet_get_owner_node(self.to_glib_none().0))
        }
    }

    fn get_parent_style_sheet(&self) -> Option<DOMStyleSheet> {
        unsafe {
            from_glib_full(ffi::webkit_dom_style_sheet_get_parent_style_sheet(self.to_glib_none().0))
        }
    }

    fn get_title(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_style_sheet_get_title(self.to_glib_none().0))
        }
    }

    fn set_disabled(&self, value: bool) {
        unsafe {
            ffi::webkit_dom_style_sheet_set_disabled(self.to_glib_none().0, value.to_glib());
        }
    }

    fn get_property_type(&self) -> Option<String> {
        unsafe {
            let mut value = Value::from_type(<String as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "type".to_glib_none().0, value.to_glib_none_mut().0);
            value.get()
        }
    }

    fn connect_property_disabled_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::disabled",
                transmute(notify_disabled_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_href_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::href",
                transmute(notify_href_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_media_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::media",
                transmute(notify_media_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_owner_node_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::owner-node",
                transmute(notify_owner_node_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_parent_style_sheet_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::parent-style-sheet",
                transmute(notify_parent_style_sheet_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_title_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::title",
                transmute(notify_title_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
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

unsafe extern "C" fn notify_disabled_trampoline<P>(this: *mut ffi::WebKitDOMStyleSheet, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMStyleSheet> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMStyleSheet::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_href_trampoline<P>(this: *mut ffi::WebKitDOMStyleSheet, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMStyleSheet> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMStyleSheet::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_media_trampoline<P>(this: *mut ffi::WebKitDOMStyleSheet, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMStyleSheet> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMStyleSheet::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_owner_node_trampoline<P>(this: *mut ffi::WebKitDOMStyleSheet, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMStyleSheet> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMStyleSheet::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_parent_style_sheet_trampoline<P>(this: *mut ffi::WebKitDOMStyleSheet, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMStyleSheet> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMStyleSheet::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_title_trampoline<P>(this: *mut ffi::WebKitDOMStyleSheet, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMStyleSheet> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMStyleSheet::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_type_trampoline<P>(this: *mut ffi::WebKitDOMStyleSheet, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMStyleSheet> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMStyleSheet::from_glib_borrow(this).downcast_unchecked())
}
