// This file was generated by gir (7484d29) from gir-files (71d73f0)
// DO NOT EDIT

use DOMDOMWindow;
use DOMEvent;
use DOMEventTarget;
use DOMNode;
use DOMObject;
use DOMUIEvent;
use ffi;
use glib;
use glib::object::Downcast;
use glib::object::IsA;
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
    pub struct DOMMouseEvent(Object<ffi::WebKitDOMMouseEvent>): DOMUIEvent, DOMEvent, DOMObject;

    match fn {
        get_type => || ffi::webkit_dom_mouse_event_get_type(),
    }
}

pub trait DOMMouseEventExt {
    fn get_alt_key(&self) -> bool;

    fn get_button(&self) -> libc::c_ushort;

    fn get_client_x(&self) -> libc::c_long;

    fn get_client_y(&self) -> libc::c_long;

    fn get_ctrl_key(&self) -> bool;

    fn get_from_element(&self) -> Option<DOMNode>;

    fn get_meta_key(&self) -> bool;

    fn get_offset_x(&self) -> libc::c_long;

    fn get_offset_y(&self) -> libc::c_long;

    fn get_related_target(&self) -> Option<DOMEventTarget>;

    fn get_screen_x(&self) -> libc::c_long;

    fn get_screen_y(&self) -> libc::c_long;

    fn get_shift_key(&self) -> bool;

    fn get_to_element(&self) -> Option<DOMNode>;

    fn get_x(&self) -> libc::c_long;

    fn get_y(&self) -> libc::c_long;

    fn init_mouse_event<P: IsA<DOMEventTarget>>(&self, type_: &str, canBubble: bool, cancelable: bool, view: &DOMDOMWindow, detail: libc::c_long, screenX: libc::c_long, screenY: libc::c_long, clientX: libc::c_long, clientY: libc::c_long, ctrlKey: bool, altKey: bool, shiftKey: bool, metaKey: bool, button: libc::c_ushort, relatedTarget: &P);

    fn connect_property_alt_key_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    fn connect_property_button_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    fn connect_property_client_x_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    fn connect_property_client_y_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    fn connect_property_ctrl_key_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    fn connect_property_from_element_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    fn connect_property_meta_key_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    fn connect_property_offset_x_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    fn connect_property_offset_y_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    fn connect_property_related_target_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    fn connect_property_screen_x_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    fn connect_property_screen_y_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    fn connect_property_shift_key_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    fn connect_property_to_element_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    fn connect_property_x_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    fn connect_property_y_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64;
}

impl<O: IsA<DOMMouseEvent> + IsA<glib::object::Object>> DOMMouseEventExt for O {
    fn get_alt_key(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_dom_mouse_event_get_alt_key(self.to_glib_none().0))
        }
    }

    fn get_button(&self) -> libc::c_ushort {
        unsafe {
            ffi::webkit_dom_mouse_event_get_button(self.to_glib_none().0)
        }
    }

    fn get_client_x(&self) -> libc::c_long {
        unsafe {
            ffi::webkit_dom_mouse_event_get_client_x(self.to_glib_none().0)
        }
    }

    fn get_client_y(&self) -> libc::c_long {
        unsafe {
            ffi::webkit_dom_mouse_event_get_client_y(self.to_glib_none().0)
        }
    }

    fn get_ctrl_key(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_dom_mouse_event_get_ctrl_key(self.to_glib_none().0))
        }
    }

    fn get_from_element(&self) -> Option<DOMNode> {
        unsafe {
            from_glib_none(ffi::webkit_dom_mouse_event_get_from_element(self.to_glib_none().0))
        }
    }

    fn get_meta_key(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_dom_mouse_event_get_meta_key(self.to_glib_none().0))
        }
    }

    fn get_offset_x(&self) -> libc::c_long {
        unsafe {
            ffi::webkit_dom_mouse_event_get_offset_x(self.to_glib_none().0)
        }
    }

    fn get_offset_y(&self) -> libc::c_long {
        unsafe {
            ffi::webkit_dom_mouse_event_get_offset_y(self.to_glib_none().0)
        }
    }

    fn get_related_target(&self) -> Option<DOMEventTarget> {
        unsafe {
            from_glib_full(ffi::webkit_dom_mouse_event_get_related_target(self.to_glib_none().0))
        }
    }

    fn get_screen_x(&self) -> libc::c_long {
        unsafe {
            ffi::webkit_dom_mouse_event_get_screen_x(self.to_glib_none().0)
        }
    }

    fn get_screen_y(&self) -> libc::c_long {
        unsafe {
            ffi::webkit_dom_mouse_event_get_screen_y(self.to_glib_none().0)
        }
    }

    fn get_shift_key(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_dom_mouse_event_get_shift_key(self.to_glib_none().0))
        }
    }

    fn get_to_element(&self) -> Option<DOMNode> {
        unsafe {
            from_glib_none(ffi::webkit_dom_mouse_event_get_to_element(self.to_glib_none().0))
        }
    }

    fn get_x(&self) -> libc::c_long {
        unsafe {
            ffi::webkit_dom_mouse_event_get_x(self.to_glib_none().0)
        }
    }

    fn get_y(&self) -> libc::c_long {
        unsafe {
            ffi::webkit_dom_mouse_event_get_y(self.to_glib_none().0)
        }
    }

    fn init_mouse_event<P: IsA<DOMEventTarget>>(&self, type_: &str, canBubble: bool, cancelable: bool, view: &DOMDOMWindow, detail: libc::c_long, screenX: libc::c_long, screenY: libc::c_long, clientX: libc::c_long, clientY: libc::c_long, ctrlKey: bool, altKey: bool, shiftKey: bool, metaKey: bool, button: libc::c_ushort, relatedTarget: &P) {
        unsafe {
            ffi::webkit_dom_mouse_event_init_mouse_event(self.to_glib_none().0, type_.to_glib_none().0, canBubble.to_glib(), cancelable.to_glib(), view.to_glib_none().0, detail, screenX, screenY, clientX, clientY, ctrlKey.to_glib(), altKey.to_glib(), shiftKey.to_glib(), metaKey.to_glib(), button, relatedTarget.to_glib_none().0);
        }
    }

    fn connect_property_alt_key_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::alt-key",
                transmute(notify_alt_key_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_button_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::button",
                transmute(notify_button_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_client_x_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::client-x",
                transmute(notify_client_x_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_client_y_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::client-y",
                transmute(notify_client_y_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_ctrl_key_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::ctrl-key",
                transmute(notify_ctrl_key_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_from_element_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::from-element",
                transmute(notify_from_element_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_meta_key_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::meta-key",
                transmute(notify_meta_key_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_offset_x_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::offset-x",
                transmute(notify_offset_x_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_offset_y_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::offset-y",
                transmute(notify_offset_y_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_related_target_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::related-target",
                transmute(notify_related_target_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_screen_x_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::screen-x",
                transmute(notify_screen_x_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_screen_y_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::screen-y",
                transmute(notify_screen_y_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_shift_key_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::shift-key",
                transmute(notify_shift_key_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_to_element_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::to-element",
                transmute(notify_to_element_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_x_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::x",
                transmute(notify_x_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_y_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::y",
                transmute(notify_y_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn notify_alt_key_trampoline<P>(this: *mut ffi::WebKitDOMMouseEvent, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMMouseEvent> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMMouseEvent::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_button_trampoline<P>(this: *mut ffi::WebKitDOMMouseEvent, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMMouseEvent> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMMouseEvent::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_client_x_trampoline<P>(this: *mut ffi::WebKitDOMMouseEvent, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMMouseEvent> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMMouseEvent::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_client_y_trampoline<P>(this: *mut ffi::WebKitDOMMouseEvent, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMMouseEvent> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMMouseEvent::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_ctrl_key_trampoline<P>(this: *mut ffi::WebKitDOMMouseEvent, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMMouseEvent> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMMouseEvent::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_from_element_trampoline<P>(this: *mut ffi::WebKitDOMMouseEvent, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMMouseEvent> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMMouseEvent::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_meta_key_trampoline<P>(this: *mut ffi::WebKitDOMMouseEvent, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMMouseEvent> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMMouseEvent::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_offset_x_trampoline<P>(this: *mut ffi::WebKitDOMMouseEvent, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMMouseEvent> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMMouseEvent::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_offset_y_trampoline<P>(this: *mut ffi::WebKitDOMMouseEvent, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMMouseEvent> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMMouseEvent::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_related_target_trampoline<P>(this: *mut ffi::WebKitDOMMouseEvent, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMMouseEvent> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMMouseEvent::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_screen_x_trampoline<P>(this: *mut ffi::WebKitDOMMouseEvent, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMMouseEvent> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMMouseEvent::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_screen_y_trampoline<P>(this: *mut ffi::WebKitDOMMouseEvent, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMMouseEvent> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMMouseEvent::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_shift_key_trampoline<P>(this: *mut ffi::WebKitDOMMouseEvent, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMMouseEvent> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMMouseEvent::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_to_element_trampoline<P>(this: *mut ffi::WebKitDOMMouseEvent, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMMouseEvent> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMMouseEvent::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_x_trampoline<P>(this: *mut ffi::WebKitDOMMouseEvent, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMMouseEvent> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMMouseEvent::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_y_trampoline<P>(this: *mut ffi::WebKitDOMMouseEvent, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMMouseEvent> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMMouseEvent::from_glib_borrow(this).downcast_unchecked())
}
