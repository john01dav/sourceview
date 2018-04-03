// This file was generated by gir (https://github.com/gtk-rs/gir @ 6855214)
// from gir-files (https://github.com/gtk-rs/gir-files @ cf55bdc)
// DO NOT EDIT

use GutterRendererAlignmentMode;
use GutterRendererState;
use cairo;
use ffi;
use gdk;
use gdk_ffi;
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
use gtk_ffi;
use libc;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct GutterRenderer(Object<ffi::GtkSourceGutterRenderer, ffi::GtkSourceGutterRendererClass>);

    match fn {
        get_type => || ffi::gtk_source_gutter_renderer_get_type(),
    }
}

pub trait GutterRendererExt {
    fn activate(&self, iter: &mut gtk::TextIter, area: &mut gdk::Rectangle, event: &mut gdk::Event);

    fn begin(&self, cr: &cairo::Context, background_area: &mut gdk::Rectangle, cell_area: &mut gdk::Rectangle, start: &mut gtk::TextIter, end: &mut gtk::TextIter);

    fn draw(&self, cr: &cairo::Context, background_area: &mut gdk::Rectangle, cell_area: &mut gdk::Rectangle, start: &mut gtk::TextIter, end: &mut gtk::TextIter, state: GutterRendererState);

    fn end(&self);

    fn get_alignment(&self) -> (f32, f32);

    fn get_alignment_mode(&self) -> GutterRendererAlignmentMode;

    fn get_background(&self) -> Option<gdk::RGBA>;

    fn get_padding(&self) -> (i32, i32);

    fn get_size(&self) -> i32;

    fn get_view(&self) -> Option<gtk::TextView>;

    fn get_visible(&self) -> bool;

    fn get_window_type(&self) -> gtk::TextWindowType;

    fn query_activatable(&self, iter: &mut gtk::TextIter, area: &mut gdk::Rectangle, event: &mut gdk::Event) -> bool;

    fn query_data(&self, start: &mut gtk::TextIter, end: &mut gtk::TextIter, state: GutterRendererState);

    fn query_tooltip(&self, iter: &mut gtk::TextIter, area: &mut gdk::Rectangle, x: i32, y: i32, tooltip: &gtk::Tooltip) -> bool;

    fn queue_draw(&self);

    fn set_alignment(&self, xalign: f32, yalign: f32);

    fn set_alignment_mode(&self, mode: GutterRendererAlignmentMode);

    fn set_background<'a, P: Into<Option<&'a gdk::RGBA>>>(&self, color: P);

    fn set_padding(&self, xpad: i32, ypad: i32);

    fn set_size(&self, size: i32);

    fn set_visible(&self, visible: bool);

    fn get_property_background_rgba(&self) -> Option<gdk::RGBA>;

    fn set_property_background_rgba(&self, background_rgba: Option<&gdk::RGBA>);

    fn get_property_background_set(&self) -> bool;

    fn set_property_background_set(&self, background_set: bool);

    fn get_property_xalign(&self) -> f32;

    fn set_property_xalign(&self, xalign: f32);

    fn get_property_xpad(&self) -> i32;

    fn set_property_xpad(&self, xpad: i32);

    fn get_property_yalign(&self) -> f32;

    fn set_property_yalign(&self, yalign: f32);

    fn get_property_ypad(&self) -> i32;

    fn set_property_ypad(&self, ypad: i32);

    fn connect_activate<F: Fn(&Self, &gtk::TextIter, &gdk::Rectangle, &gdk::Event) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_query_activatable<F: Fn(&Self, &gtk::TextIter, &gdk::Rectangle, &gdk::Event) -> bool + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_query_data<F: Fn(&Self, &gtk::TextIter, &gtk::TextIter, GutterRendererState) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_query_tooltip<F: Fn(&Self, &gtk::TextIter, &gdk::Rectangle, i32, i32, &gtk::Tooltip) -> bool + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_queue_draw<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_alignment_mode_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_background_rgba_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_background_set_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_view_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_visible_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_window_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_xalign_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_xpad_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_yalign_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_ypad_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<GutterRenderer> + IsA<glib::object::Object>> GutterRendererExt for O {
    fn activate(&self, iter: &mut gtk::TextIter, area: &mut gdk::Rectangle, event: &mut gdk::Event) {
        unsafe {
            ffi::gtk_source_gutter_renderer_activate(self.to_glib_none().0, iter.to_glib_none_mut().0, area.to_glib_none_mut().0, event.to_glib_none_mut().0);
        }
    }

    fn begin(&self, cr: &cairo::Context, background_area: &mut gdk::Rectangle, cell_area: &mut gdk::Rectangle, start: &mut gtk::TextIter, end: &mut gtk::TextIter) {
        unsafe {
            ffi::gtk_source_gutter_renderer_begin(self.to_glib_none().0, mut_override(cr.to_glib_none().0), background_area.to_glib_none_mut().0, cell_area.to_glib_none_mut().0, start.to_glib_none_mut().0, end.to_glib_none_mut().0);
        }
    }

    fn draw(&self, cr: &cairo::Context, background_area: &mut gdk::Rectangle, cell_area: &mut gdk::Rectangle, start: &mut gtk::TextIter, end: &mut gtk::TextIter, state: GutterRendererState) {
        unsafe {
            ffi::gtk_source_gutter_renderer_draw(self.to_glib_none().0, mut_override(cr.to_glib_none().0), background_area.to_glib_none_mut().0, cell_area.to_glib_none_mut().0, start.to_glib_none_mut().0, end.to_glib_none_mut().0, state.to_glib());
        }
    }

    fn end(&self) {
        unsafe {
            ffi::gtk_source_gutter_renderer_end(self.to_glib_none().0);
        }
    }

    fn get_alignment(&self) -> (f32, f32) {
        unsafe {
            let mut xalign = mem::uninitialized();
            let mut yalign = mem::uninitialized();
            ffi::gtk_source_gutter_renderer_get_alignment(self.to_glib_none().0, &mut xalign, &mut yalign);
            (xalign, yalign)
        }
    }

    fn get_alignment_mode(&self) -> GutterRendererAlignmentMode {
        unsafe {
            from_glib(ffi::gtk_source_gutter_renderer_get_alignment_mode(self.to_glib_none().0))
        }
    }

    fn get_background(&self) -> Option<gdk::RGBA> {
        unsafe {
            let mut color = gdk::RGBA::uninitialized();
            let ret = from_glib(ffi::gtk_source_gutter_renderer_get_background(self.to_glib_none().0, color.to_glib_none_mut().0));
            if ret { Some(color) } else { None }
        }
    }

    fn get_padding(&self) -> (i32, i32) {
        unsafe {
            let mut xpad = mem::uninitialized();
            let mut ypad = mem::uninitialized();
            ffi::gtk_source_gutter_renderer_get_padding(self.to_glib_none().0, &mut xpad, &mut ypad);
            (xpad, ypad)
        }
    }

    fn get_size(&self) -> i32 {
        unsafe {
            ffi::gtk_source_gutter_renderer_get_size(self.to_glib_none().0)
        }
    }

    fn get_view(&self) -> Option<gtk::TextView> {
        unsafe {
            from_glib_none(ffi::gtk_source_gutter_renderer_get_view(self.to_glib_none().0))
        }
    }

    fn get_visible(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_source_gutter_renderer_get_visible(self.to_glib_none().0))
        }
    }

    fn get_window_type(&self) -> gtk::TextWindowType {
        unsafe {
            from_glib(ffi::gtk_source_gutter_renderer_get_window_type(self.to_glib_none().0))
        }
    }

    fn query_activatable(&self, iter: &mut gtk::TextIter, area: &mut gdk::Rectangle, event: &mut gdk::Event) -> bool {
        unsafe {
            from_glib(ffi::gtk_source_gutter_renderer_query_activatable(self.to_glib_none().0, iter.to_glib_none_mut().0, area.to_glib_none_mut().0, event.to_glib_none_mut().0))
        }
    }

    fn query_data(&self, start: &mut gtk::TextIter, end: &mut gtk::TextIter, state: GutterRendererState) {
        unsafe {
            ffi::gtk_source_gutter_renderer_query_data(self.to_glib_none().0, start.to_glib_none_mut().0, end.to_glib_none_mut().0, state.to_glib());
        }
    }

    fn query_tooltip(&self, iter: &mut gtk::TextIter, area: &mut gdk::Rectangle, x: i32, y: i32, tooltip: &gtk::Tooltip) -> bool {
        unsafe {
            from_glib(ffi::gtk_source_gutter_renderer_query_tooltip(self.to_glib_none().0, iter.to_glib_none_mut().0, area.to_glib_none_mut().0, x, y, tooltip.to_glib_none().0))
        }
    }

    fn queue_draw(&self) {
        unsafe {
            ffi::gtk_source_gutter_renderer_queue_draw(self.to_glib_none().0);
        }
    }

    fn set_alignment(&self, xalign: f32, yalign: f32) {
        unsafe {
            ffi::gtk_source_gutter_renderer_set_alignment(self.to_glib_none().0, xalign, yalign);
        }
    }

    fn set_alignment_mode(&self, mode: GutterRendererAlignmentMode) {
        unsafe {
            ffi::gtk_source_gutter_renderer_set_alignment_mode(self.to_glib_none().0, mode.to_glib());
        }
    }

    fn set_background<'a, P: Into<Option<&'a gdk::RGBA>>>(&self, color: P) {
        let color = color.into();
        let color = color.to_glib_none();
        unsafe {
            ffi::gtk_source_gutter_renderer_set_background(self.to_glib_none().0, color.0);
        }
    }

    fn set_padding(&self, xpad: i32, ypad: i32) {
        unsafe {
            ffi::gtk_source_gutter_renderer_set_padding(self.to_glib_none().0, xpad, ypad);
        }
    }

    fn set_size(&self, size: i32) {
        unsafe {
            ffi::gtk_source_gutter_renderer_set_size(self.to_glib_none().0, size);
        }
    }

    fn set_visible(&self, visible: bool) {
        unsafe {
            ffi::gtk_source_gutter_renderer_set_visible(self.to_glib_none().0, visible.to_glib());
        }
    }

    fn get_property_background_rgba(&self) -> Option<gdk::RGBA> {
        unsafe {
            let mut value = Value::from_type(<gdk::RGBA as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "background-rgba".to_glib_none().0, value.to_glib_none_mut().0);
            value.get()
        }
    }

    fn set_property_background_rgba(&self, background_rgba: Option<&gdk::RGBA>) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "background-rgba".to_glib_none().0, Value::from(background_rgba).to_glib_none().0);
        }
    }

    fn get_property_background_set(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "background-set".to_glib_none().0, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_property_background_set(&self, background_set: bool) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "background-set".to_glib_none().0, Value::from(&background_set).to_glib_none().0);
        }
    }

    fn get_property_xalign(&self) -> f32 {
        unsafe {
            let mut value = Value::from_type(<f32 as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "xalign".to_glib_none().0, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_property_xalign(&self, xalign: f32) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "xalign".to_glib_none().0, Value::from(&xalign).to_glib_none().0);
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

    fn get_property_yalign(&self) -> f32 {
        unsafe {
            let mut value = Value::from_type(<f32 as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "yalign".to_glib_none().0, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_property_yalign(&self, yalign: f32) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "yalign".to_glib_none().0, Value::from(&yalign).to_glib_none().0);
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

    fn connect_activate<F: Fn(&Self, &gtk::TextIter, &gdk::Rectangle, &gdk::Event) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, &gtk::TextIter, &gdk::Rectangle, &gdk::Event) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "activate",
                transmute(activate_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_query_activatable<F: Fn(&Self, &gtk::TextIter, &gdk::Rectangle, &gdk::Event) -> bool + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, &gtk::TextIter, &gdk::Rectangle, &gdk::Event) -> bool + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "query-activatable",
                transmute(query_activatable_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_query_data<F: Fn(&Self, &gtk::TextIter, &gtk::TextIter, GutterRendererState) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, &gtk::TextIter, &gtk::TextIter, GutterRendererState) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "query-data",
                transmute(query_data_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_query_tooltip<F: Fn(&Self, &gtk::TextIter, &gdk::Rectangle, i32, i32, &gtk::Tooltip) -> bool + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, &gtk::TextIter, &gdk::Rectangle, i32, i32, &gtk::Tooltip) -> bool + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "query-tooltip",
                transmute(query_tooltip_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_queue_draw<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "queue-draw",
                transmute(queue_draw_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_alignment_mode_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::alignment-mode",
                transmute(notify_alignment_mode_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_background_rgba_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::background-rgba",
                transmute(notify_background_rgba_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_background_set_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::background-set",
                transmute(notify_background_set_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::size",
                transmute(notify_size_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_view_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::view",
                transmute(notify_view_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_visible_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::visible",
                transmute(notify_visible_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_window_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::window-type",
                transmute(notify_window_type_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_xalign_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::xalign",
                transmute(notify_xalign_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_xpad_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::xpad",
                transmute(notify_xpad_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_yalign_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::yalign",
                transmute(notify_yalign_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
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

unsafe extern "C" fn activate_trampoline<P>(this: *mut ffi::GtkSourceGutterRenderer, iter: *mut gtk_ffi::GtkTextIter, area: *mut gdk_ffi::GdkRectangle, event: *mut gdk_ffi::GdkEvent, f: glib_ffi::gpointer)
where P: IsA<GutterRenderer> {
    callback_guard!();
    let f: &&(Fn(&P, &gtk::TextIter, &gdk::Rectangle, &gdk::Event) + 'static) = transmute(f);
    f(&GutterRenderer::from_glib_borrow(this).downcast_unchecked(), &from_glib_borrow(iter), &from_glib_borrow(area), &from_glib_none(event))
}

unsafe extern "C" fn query_activatable_trampoline<P>(this: *mut ffi::GtkSourceGutterRenderer, iter: *mut gtk_ffi::GtkTextIter, area: *mut gdk_ffi::GdkRectangle, event: *mut gdk_ffi::GdkEvent, f: glib_ffi::gpointer) -> glib_ffi::gboolean
where P: IsA<GutterRenderer> {
    callback_guard!();
    let f: &&(Fn(&P, &gtk::TextIter, &gdk::Rectangle, &gdk::Event) -> bool + 'static) = transmute(f);
    f(&GutterRenderer::from_glib_borrow(this).downcast_unchecked(), &from_glib_borrow(iter), &from_glib_borrow(area), &from_glib_none(event)).to_glib()
}

unsafe extern "C" fn query_data_trampoline<P>(this: *mut ffi::GtkSourceGutterRenderer, start: *mut gtk_ffi::GtkTextIter, end: *mut gtk_ffi::GtkTextIter, state: ffi::GtkSourceGutterRendererState, f: glib_ffi::gpointer)
where P: IsA<GutterRenderer> {
    callback_guard!();
    let f: &&(Fn(&P, &gtk::TextIter, &gtk::TextIter, GutterRendererState) + 'static) = transmute(f);
    f(&GutterRenderer::from_glib_borrow(this).downcast_unchecked(), &from_glib_borrow(start), &from_glib_borrow(end), from_glib(state))
}

unsafe extern "C" fn query_tooltip_trampoline<P>(this: *mut ffi::GtkSourceGutterRenderer, iter: *mut gtk_ffi::GtkTextIter, area: *mut gdk_ffi::GdkRectangle, x: libc::c_int, y: libc::c_int, tooltip: *mut gtk_ffi::GtkTooltip, f: glib_ffi::gpointer) -> glib_ffi::gboolean
where P: IsA<GutterRenderer> {
    callback_guard!();
    let f: &&(Fn(&P, &gtk::TextIter, &gdk::Rectangle, i32, i32, &gtk::Tooltip) -> bool + 'static) = transmute(f);
    f(&GutterRenderer::from_glib_borrow(this).downcast_unchecked(), &from_glib_borrow(iter), &from_glib_borrow(area), x, y, &from_glib_borrow(tooltip)).to_glib()
}

unsafe extern "C" fn queue_draw_trampoline<P>(this: *mut ffi::GtkSourceGutterRenderer, f: glib_ffi::gpointer)
where P: IsA<GutterRenderer> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&GutterRenderer::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_alignment_mode_trampoline<P>(this: *mut ffi::GtkSourceGutterRenderer, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<GutterRenderer> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&GutterRenderer::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_background_rgba_trampoline<P>(this: *mut ffi::GtkSourceGutterRenderer, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<GutterRenderer> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&GutterRenderer::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_background_set_trampoline<P>(this: *mut ffi::GtkSourceGutterRenderer, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<GutterRenderer> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&GutterRenderer::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_size_trampoline<P>(this: *mut ffi::GtkSourceGutterRenderer, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<GutterRenderer> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&GutterRenderer::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_view_trampoline<P>(this: *mut ffi::GtkSourceGutterRenderer, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<GutterRenderer> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&GutterRenderer::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_visible_trampoline<P>(this: *mut ffi::GtkSourceGutterRenderer, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<GutterRenderer> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&GutterRenderer::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_window_type_trampoline<P>(this: *mut ffi::GtkSourceGutterRenderer, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<GutterRenderer> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&GutterRenderer::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_xalign_trampoline<P>(this: *mut ffi::GtkSourceGutterRenderer, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<GutterRenderer> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&GutterRenderer::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_xpad_trampoline<P>(this: *mut ffi::GtkSourceGutterRenderer, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<GutterRenderer> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&GutterRenderer::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_yalign_trampoline<P>(this: *mut ffi::GtkSourceGutterRenderer, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<GutterRenderer> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&GutterRenderer::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_ypad_trampoline<P>(this: *mut ffi::GtkSourceGutterRenderer, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<GutterRenderer> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&GutterRenderer::from_glib_borrow(this).downcast_unchecked())
}
