// This file was generated by gir (e95e5d8) from gir-files (db49619)
// DO NOT EDIT

use ffi;
use glib;
use glib::Value;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use gtk;
use gtk_ffi;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct Mark(Object<ffi::GtkSourceMark>): [
        gtk::TextMark => gtk_ffi::GtkTextMark,
    ];

    match fn {
        get_type => || ffi::gtk_source_mark_get_type(),
    }
}

impl Mark {
    pub fn new(name: &str, category: &str) -> Mark {
        unsafe {
            from_glib_full(ffi::gtk_source_mark_new(name.to_glib_none().0, category.to_glib_none().0))
        }
    }
}

pub trait MarkExt {
    fn get_category(&self) -> Option<String>;

    fn next<'a, P: Into<Option<&'a str>>>(&self, category: P) -> Option<Mark>;

    fn prev(&self, category: &str) -> Option<Mark>;

    fn get_property_category(&self) -> Option<String>;

    fn connect_property_category_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Mark> + IsA<glib::object::Object>> MarkExt for O {
    fn get_category(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_source_mark_get_category(self.to_glib_none().0))
        }
    }

    fn next<'a, P: Into<Option<&'a str>>>(&self, category: P) -> Option<Mark> {
        let category = category.into();
        let category = category.to_glib_none();
        unsafe {
            from_glib_none(ffi::gtk_source_mark_next(self.to_glib_none().0, category.0))
        }
    }

    fn prev(&self, category: &str) -> Option<Mark> {
        unsafe {
            from_glib_none(ffi::gtk_source_mark_prev(self.to_glib_none().0, category.to_glib_none().0))
        }
    }

    fn get_property_category(&self) -> Option<String> {
        let mut value = Value::from(None::<&str>);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "category".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get()
    }

    fn connect_property_category_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::category",
                transmute(notify_category_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn notify_category_trampoline<P>(this: *mut ffi::GtkSourceMark, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Mark> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Mark::from_glib_borrow(this).downcast_unchecked())
}
