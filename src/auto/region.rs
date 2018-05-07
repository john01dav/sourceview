// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

#[cfg(any(feature = "v3_22", feature = "dox"))]
use RegionIter;
use ffi;
use glib;
#[cfg(any(feature = "v3_22", feature = "dox"))]
use glib::object::Downcast;
use glib::object::IsA;
#[cfg(any(feature = "v3_22", feature = "dox"))]
use glib::signal::SignalHandlerId;
#[cfg(any(feature = "v3_22", feature = "dox"))]
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
#[cfg(any(feature = "v3_22", feature = "dox"))]
use gtk;
#[cfg(any(feature = "v3_22", feature = "dox"))]
use std::boxed::Box as Box_;
use std::fmt;
use std::mem;
#[cfg(any(feature = "v3_22", feature = "dox"))]
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct Region(Object<ffi::GtkSourceRegion, ffi::GtkSourceRegionClass>);

    match fn {
        get_type => || ffi::gtk_source_region_get_type(),
    }
}

impl Region {
    #[cfg(any(feature = "v3_22", feature = "dox"))]
    pub fn new<P: IsA<gtk::TextBuffer>>(buffer: &P) -> Region {
        unsafe {
            from_glib_full(ffi::gtk_source_region_new(buffer.to_glib_none().0))
        }
    }
}

impl fmt::Display for Region {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", RegionExt::to_string(self))
    }
}

pub trait RegionExt {
    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn add_region<'a, P: Into<Option<&'a Region>>>(&self, region_to_add: P);

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn add_subregion(&self, _start: &gtk::TextIter, _end: &gtk::TextIter);

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn get_bounds(&self) -> Option<(gtk::TextIter, gtk::TextIter)>;

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn get_buffer(&self) -> Option<gtk::TextBuffer>;

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn get_start_region_iter(&self) -> RegionIter;

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn intersect_region<'a, P: Into<Option<&'a Region>>>(&self, region2: P) -> Option<Region>;

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn intersect_subregion(&self, _start: &gtk::TextIter, _end: &gtk::TextIter) -> Option<Region>;

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn is_empty(&self) -> bool;

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn subtract_region<'a, P: Into<Option<&'a Region>>>(&self, region_to_subtract: P);

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn subtract_subregion(&self, _start: &gtk::TextIter, _end: &gtk::TextIter);

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn to_string(&self) -> String;

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn connect_property_buffer_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Region> + IsA<glib::object::Object>> RegionExt for O {
    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn add_region<'a, P: Into<Option<&'a Region>>>(&self, region_to_add: P) {
        let region_to_add = region_to_add.into();
        let region_to_add = region_to_add.to_glib_none();
        unsafe {
            ffi::gtk_source_region_add_region(self.to_glib_none().0, region_to_add.0);
        }
    }

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn add_subregion(&self, _start: &gtk::TextIter, _end: &gtk::TextIter) {
        unsafe {
            ffi::gtk_source_region_add_subregion(self.to_glib_none().0, _start.to_glib_none().0, _end.to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn get_bounds(&self) -> Option<(gtk::TextIter, gtk::TextIter)> {
        unsafe {
            let mut start = gtk::TextIter::uninitialized();
            let mut end = gtk::TextIter::uninitialized();
            let ret = from_glib(ffi::gtk_source_region_get_bounds(self.to_glib_none().0, start.to_glib_none_mut().0, end.to_glib_none_mut().0));
            if ret { Some((start, end)) } else { None }
        }
    }

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn get_buffer(&self) -> Option<gtk::TextBuffer> {
        unsafe {
            from_glib_none(ffi::gtk_source_region_get_buffer(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn get_start_region_iter(&self) -> RegionIter {
        unsafe {
            let mut iter = RegionIter::uninitialized();
            ffi::gtk_source_region_get_start_region_iter(self.to_glib_none().0, iter.to_glib_none_mut().0);
            iter
        }
    }

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn intersect_region<'a, P: Into<Option<&'a Region>>>(&self, region2: P) -> Option<Region> {
        let region2 = region2.into();
        let region2 = region2.to_glib_none();
        unsafe {
            from_glib_full(ffi::gtk_source_region_intersect_region(self.to_glib_none().0, region2.0))
        }
    }

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn intersect_subregion(&self, _start: &gtk::TextIter, _end: &gtk::TextIter) -> Option<Region> {
        unsafe {
            from_glib_full(ffi::gtk_source_region_intersect_subregion(self.to_glib_none().0, _start.to_glib_none().0, _end.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn is_empty(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_source_region_is_empty(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn subtract_region<'a, P: Into<Option<&'a Region>>>(&self, region_to_subtract: P) {
        let region_to_subtract = region_to_subtract.into();
        let region_to_subtract = region_to_subtract.to_glib_none();
        unsafe {
            ffi::gtk_source_region_subtract_region(self.to_glib_none().0, region_to_subtract.0);
        }
    }

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn subtract_subregion(&self, _start: &gtk::TextIter, _end: &gtk::TextIter) {
        unsafe {
            ffi::gtk_source_region_subtract_subregion(self.to_glib_none().0, _start.to_glib_none().0, _end.to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn to_string(&self) -> String {
        unsafe {
            from_glib_full(ffi::gtk_source_region_to_string(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn connect_property_buffer_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::buffer",
                transmute(notify_buffer_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

#[cfg(any(feature = "v3_22", feature = "dox"))]
unsafe extern "C" fn notify_buffer_trampoline<P>(this: *mut ffi::GtkSourceRegion, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Region> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Region::from_glib_borrow(this).downcast_unchecked())
}
