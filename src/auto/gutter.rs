// This file was generated by gir (https://github.com/gtk-rs/gir @ 6855214)
// from gir-files (https://github.com/gtk-rs/gir-files @ cf55bdc)
// DO NOT EDIT

use GutterRenderer;
use View;
use ffi;
use gdk;
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
use gtk;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct Gutter(Object<ffi::GtkSourceGutter, ffi::GtkSourceGutterClass>);

    match fn {
        get_type => || ffi::gtk_source_gutter_get_type(),
    }
}

pub trait GutterExt {
    fn get_renderer_at_pos(&self, x: i32, y: i32) -> Option<GutterRenderer>;

    #[cfg(any(feature = "v3_24", feature = "dox"))]
    fn get_view(&self) -> Option<View>;

    #[cfg_attr(feature = "v3_12", deprecated)]
    fn get_window(&self) -> Option<gdk::Window>;

    #[cfg(any(feature = "v3_24", feature = "dox"))]
    fn get_window_type(&self) -> gtk::TextWindowType;

    fn insert<P: IsA<GutterRenderer>>(&self, renderer: &P, position: i32) -> bool;

    fn queue_draw(&self);

    fn remove<P: IsA<GutterRenderer>>(&self, renderer: &P);

    fn reorder<P: IsA<GutterRenderer>>(&self, renderer: &P, position: i32);

    #[cfg_attr(feature = "v3_12", deprecated)]
    fn set_padding(&self, xpad: i32, ypad: i32);

    fn get_property_view(&self) -> Option<View>;

    fn get_property_window_type(&self) -> gtk::TextWindowType;

    #[cfg_attr(feature = "v3_12", deprecated)]
    fn get_property_xpad(&self) -> i32;

    #[cfg_attr(feature = "v3_12", deprecated)]
    fn set_property_xpad(&self, xpad: i32);

    #[cfg_attr(feature = "v3_12", deprecated)]
    fn get_property_ypad(&self) -> i32;

    #[cfg_attr(feature = "v3_12", deprecated)]
    fn set_property_ypad(&self, ypad: i32);

    fn connect_property_view_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_window_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg_attr(feature = "v3_12", deprecated)]
    fn connect_property_xpad_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg_attr(feature = "v3_12", deprecated)]
    fn connect_property_ypad_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Gutter> + IsA<glib::object::Object>> GutterExt for O {
    fn get_renderer_at_pos(&self, x: i32, y: i32) -> Option<GutterRenderer> {
        unsafe {
            from_glib_none(ffi::gtk_source_gutter_get_renderer_at_pos(self.to_glib_none().0, x, y))
        }
    }

    #[cfg(any(feature = "v3_24", feature = "dox"))]
    fn get_view(&self) -> Option<View> {
        unsafe {
            from_glib_none(ffi::gtk_source_gutter_get_view(self.to_glib_none().0))
        }
    }

    fn get_window(&self) -> Option<gdk::Window> {
        unsafe {
            from_glib_none(ffi::gtk_source_gutter_get_window(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_24", feature = "dox"))]
    fn get_window_type(&self) -> gtk::TextWindowType {
        unsafe {
            from_glib(ffi::gtk_source_gutter_get_window_type(self.to_glib_none().0))
        }
    }

    fn insert<P: IsA<GutterRenderer>>(&self, renderer: &P, position: i32) -> bool {
        unsafe {
            from_glib(ffi::gtk_source_gutter_insert(self.to_glib_none().0, renderer.to_glib_none().0, position))
        }
    }

    fn queue_draw(&self) {
        unsafe {
            ffi::gtk_source_gutter_queue_draw(self.to_glib_none().0);
        }
    }

    fn remove<P: IsA<GutterRenderer>>(&self, renderer: &P) {
        unsafe {
            ffi::gtk_source_gutter_remove(self.to_glib_none().0, renderer.to_glib_none().0);
        }
    }

    fn reorder<P: IsA<GutterRenderer>>(&self, renderer: &P, position: i32) {
        unsafe {
            ffi::gtk_source_gutter_reorder(self.to_glib_none().0, renderer.to_glib_none().0, position);
        }
    }

    fn set_padding(&self, xpad: i32, ypad: i32) {
        unsafe {
            ffi::gtk_source_gutter_set_padding(self.to_glib_none().0, xpad, ypad);
        }
    }

    fn get_property_view(&self) -> Option<View> {
        unsafe {
            let mut value = Value::from_type(<View as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "view".to_glib_none().0, value.to_glib_none_mut().0);
            value.get()
        }
    }

    fn get_property_window_type(&self) -> gtk::TextWindowType {
        unsafe {
            let mut value = Value::from_type(<gtk::TextWindowType as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "window-type".to_glib_none().0, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn get_property_xpad(&self) -> i32 {
        unsafe {
            let mut value = Value::from_type(<i32 as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "xpad".to_glib_none().0, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_property_xpad(&self, xpad: i32) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "xpad".to_glib_none().0, Value::from(&xpad).to_glib_none().0);
        }
    }

    fn get_property_ypad(&self) -> i32 {
        unsafe {
            let mut value = Value::from_type(<i32 as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "ypad".to_glib_none().0, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_property_ypad(&self, ypad: i32) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "ypad".to_glib_none().0, Value::from(&ypad).to_glib_none().0);
        }
    }

    fn connect_property_view_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::view",
                transmute(notify_view_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_window_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::window-type",
                transmute(notify_window_type_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_xpad_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::xpad",
                transmute(notify_xpad_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_ypad_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::ypad",
                transmute(notify_ypad_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn notify_view_trampoline<P>(this: *mut ffi::GtkSourceGutter, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Gutter> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Gutter::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_window_type_trampoline<P>(this: *mut ffi::GtkSourceGutter, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Gutter> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Gutter::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_xpad_trampoline<P>(this: *mut ffi::GtkSourceGutter, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Gutter> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Gutter::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_ypad_trampoline<P>(this: *mut ffi::GtkSourceGutter, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Gutter> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Gutter::from_glib_borrow(this).downcast_unchecked())
}
