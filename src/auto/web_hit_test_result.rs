// This file was generated by gir (https://github.com/gtk-rs/gir @ f5fca82)
// from gir-files (https://github.com/gtk-rs/gir-files @ b8f5ef1)
// DO NOT EDIT

use DOMNode;
use HitTestResult;
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
    pub struct WebHitTestResult(Object<ffi::WebKitWebHitTestResult, ffi::WebKitWebHitTestResultClass>): HitTestResult;

    match fn {
        get_type => || ffi::webkit_web_hit_test_result_get_type(),
    }
}

pub trait WebHitTestResultExt {
    #[cfg(any(feature = "v2_8", feature = "dox"))]
    fn get_node(&self) -> Option<DOMNode>;

    fn get_property_node(&self) -> Option<DOMNode>;

    fn connect_property_node_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<WebHitTestResult> + IsA<glib::object::Object>> WebHitTestResultExt for O {
    #[cfg(any(feature = "v2_8", feature = "dox"))]
    fn get_node(&self) -> Option<DOMNode> {
        unsafe {
            from_glib_none(ffi::webkit_web_hit_test_result_get_node(self.to_glib_none().0))
        }
    }

    fn get_property_node(&self) -> Option<DOMNode> {
        unsafe {
            let mut value = Value::from_type(<DOMNode as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "node".to_glib_none().0, value.to_glib_none_mut().0);
            value.get()
        }
    }

    fn connect_property_node_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::node",
                transmute(notify_node_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn notify_node_trampoline<P>(this: *mut ffi::WebKitWebHitTestResult, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<WebHitTestResult> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&WebHitTestResult::from_glib_borrow(this).downcast_unchecked())
}
