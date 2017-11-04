// This file was generated by gir (94e079d) from gir-files (469db10)
// DO NOT EDIT

use GutterRenderer;
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
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct GutterRendererText(Object<ffi::GtkSourceGutterRendererText>): GutterRenderer;

    match fn {
        get_type => || ffi::gtk_source_gutter_renderer_text_get_type(),
    }
}

impl GutterRendererText {
    pub fn new() -> GutterRendererText {
        unsafe {
            GutterRenderer::from_glib_full(ffi::gtk_source_gutter_renderer_text_new()).downcast_unchecked()
        }
    }
}

impl Default for GutterRendererText {
    fn default() -> Self {
        Self::new()
    }
}

pub trait GutterRendererTextExt {
    fn measure(&self, text: &str) -> (i32, i32);

    fn measure_markup(&self, markup: &str) -> (i32, i32);

    fn set_markup(&self, markup: &str);

    fn set_text(&self, text: &str);

    fn get_property_markup(&self) -> Option<String>;

    fn get_property_text(&self) -> Option<String>;

    fn connect_property_markup_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_text_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<GutterRendererText> + IsA<glib::object::Object>> GutterRendererTextExt for O {
    fn measure(&self, text: &str) -> (i32, i32) {
        unsafe {
            let mut width = mem::uninitialized();
            let mut height = mem::uninitialized();
            ffi::gtk_source_gutter_renderer_text_measure(self.to_glib_none().0, text.to_glib_none().0, &mut width, &mut height);
            (width, height)
        }
    }

    fn measure_markup(&self, markup: &str) -> (i32, i32) {
        unsafe {
            let mut width = mem::uninitialized();
            let mut height = mem::uninitialized();
            ffi::gtk_source_gutter_renderer_text_measure_markup(self.to_glib_none().0, markup.to_glib_none().0, &mut width, &mut height);
            (width, height)
        }
    }

    fn set_markup(&self, markup: &str) {
        let length = markup.len() as i32;
        unsafe {
            ffi::gtk_source_gutter_renderer_text_set_markup(self.to_glib_none().0, markup.to_glib_none().0, length);
        }
    }

    fn set_text(&self, text: &str) {
        let length = text.len() as i32;
        unsafe {
            ffi::gtk_source_gutter_renderer_text_set_text(self.to_glib_none().0, text.to_glib_none().0, length);
        }
    }

    fn get_property_markup(&self) -> Option<String> {
        let mut value = Value::from(None::<&str>);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "markup".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get()
    }

    fn get_property_text(&self) -> Option<String> {
        let mut value = Value::from(None::<&str>);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "text".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get()
    }

    fn connect_property_markup_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::markup",
                transmute(notify_markup_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_text_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::text",
                transmute(notify_text_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn notify_markup_trampoline<P>(this: *mut ffi::GtkSourceGutterRendererText, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<GutterRendererText> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&GutterRendererText::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_text_trampoline<P>(this: *mut ffi::GtkSourceGutterRendererText, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<GutterRendererText> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&GutterRendererText::from_glib_borrow(this).downcast_unchecked())
}
