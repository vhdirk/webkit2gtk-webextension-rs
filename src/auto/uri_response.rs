// This file was generated by gir (https://github.com/gtk-rs/gir @ 8b9d0bb)
// from gir-files (https://github.com/gtk-rs/gir-files @ 77d1f70)
// DO NOT EDIT

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
    pub struct URIResponse(Object<ffi::WebKitURIResponse, ffi::WebKitURIResponseClass>);

    match fn {
        get_type => || ffi::webkit_uri_response_get_type(),
    }
}

pub trait URIResponseExt {
    fn get_content_length(&self) -> u64;

    //#[cfg(any(feature = "v2_6", feature = "dox"))]
    //fn get_http_headers(&self) -> /*Ignored*/Option<soup::MessageHeaders>;

    fn get_mime_type(&self) -> Option<String>;

    fn get_status_code(&self) -> u32;

    fn get_suggested_filename(&self) -> Option<String>;

    fn get_uri(&self) -> Option<String>;

    fn connect_property_content_length_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v2_6", feature = "dox"))]
    fn connect_property_http_headers_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_mime_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_status_code_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_suggested_filename_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_uri_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<URIResponse> + IsA<glib::object::Object>> URIResponseExt for O {
    fn get_content_length(&self) -> u64 {
        unsafe {
            ffi::webkit_uri_response_get_content_length(self.to_glib_none().0)
        }
    }

    //#[cfg(any(feature = "v2_6", feature = "dox"))]
    //fn get_http_headers(&self) -> /*Ignored*/Option<soup::MessageHeaders> {
    //    unsafe { TODO: call ffi::webkit_uri_response_get_http_headers() }
    //}

    fn get_mime_type(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::webkit_uri_response_get_mime_type(self.to_glib_none().0))
        }
    }

    fn get_status_code(&self) -> u32 {
        unsafe {
            ffi::webkit_uri_response_get_status_code(self.to_glib_none().0)
        }
    }

    fn get_suggested_filename(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::webkit_uri_response_get_suggested_filename(self.to_glib_none().0))
        }
    }

    fn get_uri(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::webkit_uri_response_get_uri(self.to_glib_none().0))
        }
    }

    fn connect_property_content_length_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::content-length",
                transmute(notify_content_length_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(any(feature = "v2_6", feature = "dox"))]
    fn connect_property_http_headers_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::http-headers",
                transmute(notify_http_headers_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_mime_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::mime-type",
                transmute(notify_mime_type_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_status_code_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::status-code",
                transmute(notify_status_code_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_suggested_filename_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::suggested-filename",
                transmute(notify_suggested_filename_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_uri_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::uri",
                transmute(notify_uri_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn notify_content_length_trampoline<P>(this: *mut ffi::WebKitURIResponse, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<URIResponse> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&URIResponse::from_glib_borrow(this).downcast_unchecked())
}

#[cfg(any(feature = "v2_6", feature = "dox"))]
unsafe extern "C" fn notify_http_headers_trampoline<P>(this: *mut ffi::WebKitURIResponse, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<URIResponse> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&URIResponse::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_mime_type_trampoline<P>(this: *mut ffi::WebKitURIResponse, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<URIResponse> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&URIResponse::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_status_code_trampoline<P>(this: *mut ffi::WebKitURIResponse, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<URIResponse> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&URIResponse::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_suggested_filename_trampoline<P>(this: *mut ffi::WebKitURIResponse, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<URIResponse> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&URIResponse::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_uri_trampoline<P>(this: *mut ffi::WebKitURIResponse, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<URIResponse> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&URIResponse::from_glib_borrow(this).downcast_unchecked())
}
