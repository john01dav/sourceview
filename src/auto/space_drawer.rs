// This file was generated by gir (e95e5d8) from gir-files (db49619)
// DO NOT EDIT

use ffi;
use glib;
#[cfg(feature = "v3_24")]
use glib::object::Downcast;
use glib::object::IsA;
#[cfg(feature = "v3_24")]
use glib::signal::SignalHandlerId;
#[cfg(feature = "v3_24")]
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
#[cfg(feature = "v3_24")]
use std::boxed::Box as Box_;
use std::mem;
#[cfg(feature = "v3_24")]
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct SpaceDrawer(Object<ffi::GtkSourceSpaceDrawer>);

    match fn {
        get_type => || ffi::gtk_source_space_drawer_get_type(),
    }
}

impl SpaceDrawer {
    #[cfg(feature = "v3_24")]
    pub fn new() -> SpaceDrawer {
        unsafe {
            from_glib_full(ffi::gtk_source_space_drawer_new())
        }
    }
}

#[cfg(feature = "v3_24")]
impl Default for SpaceDrawer {
    fn default() -> Self {
        Self::new()
    }
}

pub trait SpaceDrawerExt {
    //#[cfg(feature = "v3_24")]
    //fn bind_matrix_setting(&self, settings: /*Ignored*/&gio::Settings, key: &str, flags: /*Ignored*/gio::SettingsBindFlags);

    #[cfg(feature = "v3_24")]
    fn get_enable_matrix(&self) -> bool;

    //#[cfg(feature = "v3_24")]
    //fn get_matrix(&self) -> /*Ignored*/Option<glib::Variant>;

    //#[cfg(feature = "v3_24")]
    //fn get_types_for_locations(&self, locations: /*Ignored*/SpaceLocationFlags) -> /*Ignored*/SpaceTypeFlags;

    #[cfg(feature = "v3_24")]
    fn set_enable_matrix(&self, enable_matrix: bool);

    //#[cfg(feature = "v3_24")]
    //fn set_matrix<'a, P: Into<Option<&'a /*Ignored*/glib::Variant>>>(&self, matrix: P);

    //#[cfg(feature = "v3_24")]
    //fn set_types_for_locations(&self, locations: /*Ignored*/SpaceLocationFlags, types: /*Ignored*/SpaceTypeFlags);

    #[cfg(feature = "v3_24")]
    fn connect_property_enable_matrix_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(feature = "v3_24")]
    fn connect_property_matrix_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<SpaceDrawer> + IsA<glib::object::Object>> SpaceDrawerExt for O {
    //#[cfg(feature = "v3_24")]
    //fn bind_matrix_setting(&self, settings: /*Ignored*/&gio::Settings, key: &str, flags: /*Ignored*/gio::SettingsBindFlags) {
    //    unsafe { TODO: call ffi::gtk_source_space_drawer_bind_matrix_setting() }
    //}

    #[cfg(feature = "v3_24")]
    fn get_enable_matrix(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_source_space_drawer_get_enable_matrix(self.to_glib_none().0))
        }
    }

    //#[cfg(feature = "v3_24")]
    //fn get_matrix(&self) -> /*Ignored*/Option<glib::Variant> {
    //    unsafe { TODO: call ffi::gtk_source_space_drawer_get_matrix() }
    //}

    //#[cfg(feature = "v3_24")]
    //fn get_types_for_locations(&self, locations: /*Ignored*/SpaceLocationFlags) -> /*Ignored*/SpaceTypeFlags {
    //    unsafe { TODO: call ffi::gtk_source_space_drawer_get_types_for_locations() }
    //}

    #[cfg(feature = "v3_24")]
    fn set_enable_matrix(&self, enable_matrix: bool) {
        unsafe {
            ffi::gtk_source_space_drawer_set_enable_matrix(self.to_glib_none().0, enable_matrix.to_glib());
        }
    }

    //#[cfg(feature = "v3_24")]
    //fn set_matrix<'a, P: Into<Option<&'a /*Ignored*/glib::Variant>>>(&self, matrix: P) {
    //    unsafe { TODO: call ffi::gtk_source_space_drawer_set_matrix() }
    //}

    //#[cfg(feature = "v3_24")]
    //fn set_types_for_locations(&self, locations: /*Ignored*/SpaceLocationFlags, types: /*Ignored*/SpaceTypeFlags) {
    //    unsafe { TODO: call ffi::gtk_source_space_drawer_set_types_for_locations() }
    //}

    #[cfg(feature = "v3_24")]
    fn connect_property_enable_matrix_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::enable-matrix",
                transmute(notify_enable_matrix_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(feature = "v3_24")]
    fn connect_property_matrix_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::matrix",
                transmute(notify_matrix_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

#[cfg(feature = "v3_24")]
unsafe extern "C" fn notify_enable_matrix_trampoline<P>(this: *mut ffi::GtkSourceSpaceDrawer, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<SpaceDrawer> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&SpaceDrawer::from_glib_borrow(this).downcast_unchecked())
}

#[cfg(feature = "v3_24")]
unsafe extern "C" fn notify_matrix_trampoline<P>(this: *mut ffi::GtkSourceSpaceDrawer, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<SpaceDrawer> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&SpaceDrawer::from_glib_borrow(this).downcast_unchecked())
}
