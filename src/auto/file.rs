// This file was generated by gir (94e079d) from gir-files (469db10)
// DO NOT EDIT

#[cfg(any(feature = "v3_14", feature = "dox"))]
use CompressionType;
#[cfg(any(feature = "v3_14", feature = "dox"))]
use Encoding;
#[cfg(any(feature = "v3_14", feature = "dox"))]
use NewlineType;
use ffi;
#[cfg(any(feature = "v3_14", feature = "dox"))]
use gio;
use glib;
#[cfg(any(feature = "v3_18", feature = "dox"))]
use glib::Value;
#[cfg(any(feature = "v3_14", feature = "dox"))]
use glib::object::Downcast;
use glib::object::IsA;
#[cfg(any(feature = "v3_14", feature = "dox"))]
use glib::signal::SignalHandlerId;
#[cfg(any(feature = "v3_14", feature = "dox"))]
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
#[cfg(any(feature = "v3_14", feature = "dox"))]
use std::boxed::Box as Box_;
use std::mem;
#[cfg(any(feature = "v3_14", feature = "dox"))]
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct File(Object<ffi::GtkSourceFile>);

    match fn {
        get_type => || ffi::gtk_source_file_get_type(),
    }
}

impl File {
    #[cfg(any(feature = "v3_14", feature = "dox"))]
    pub fn new() -> File {
        unsafe {
            from_glib_full(ffi::gtk_source_file_new())
        }
    }
}

#[cfg(any(feature = "v3_14", feature = "dox"))]
impl Default for File {
    fn default() -> Self {
        Self::new()
    }
}

pub trait FileExt {
    #[cfg(any(feature = "v3_18", feature = "dox"))]
    fn check_file_on_disk(&self);

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn get_compression_type(&self) -> CompressionType;

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn get_encoding(&self) -> Option<Encoding>;

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn get_location(&self) -> Option<gio::File>;

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn get_newline_type(&self) -> NewlineType;

    #[cfg(any(feature = "v3_18", feature = "dox"))]
    fn is_deleted(&self) -> bool;

    #[cfg(any(feature = "v3_18", feature = "dox"))]
    fn is_externally_modified(&self) -> bool;

    #[cfg(any(feature = "v3_18", feature = "dox"))]
    fn is_local(&self) -> bool;

    #[cfg(any(feature = "v3_18", feature = "dox"))]
    fn is_readonly(&self) -> bool;

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn set_location<'a, P: IsA<gio::File> + 'a, Q: Into<Option<&'a P>>>(&self, location: Q);

    //#[cfg(any(feature = "v3_14", feature = "dox"))]
    //fn set_mount_operation_factory<'a, P: Into<Option</*Unimplemented*/Fundamental: Pointer>>, Q: Into<Option<&'a /*Ignored*/glib::DestroyNotify>>>(&self, callback: /*Unknown conversion*//*Unimplemented*/MountOperationFactory, user_data: P, notify: Q);

    #[cfg(any(feature = "v3_18", feature = "dox"))]
    fn get_property_read_only(&self) -> bool;

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn connect_property_compression_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn connect_property_encoding_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn connect_property_location_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn connect_property_newline_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v3_18", feature = "dox"))]
    fn connect_property_read_only_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<File> + IsA<glib::object::Object>> FileExt for O {
    #[cfg(any(feature = "v3_18", feature = "dox"))]
    fn check_file_on_disk(&self) {
        unsafe {
            ffi::gtk_source_file_check_file_on_disk(self.to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn get_compression_type(&self) -> CompressionType {
        unsafe {
            from_glib(ffi::gtk_source_file_get_compression_type(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn get_encoding(&self) -> Option<Encoding> {
        unsafe {
            from_glib_none(ffi::gtk_source_file_get_encoding(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn get_location(&self) -> Option<gio::File> {
        unsafe {
            from_glib_none(ffi::gtk_source_file_get_location(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn get_newline_type(&self) -> NewlineType {
        unsafe {
            from_glib(ffi::gtk_source_file_get_newline_type(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_18", feature = "dox"))]
    fn is_deleted(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_source_file_is_deleted(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_18", feature = "dox"))]
    fn is_externally_modified(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_source_file_is_externally_modified(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_18", feature = "dox"))]
    fn is_local(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_source_file_is_local(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_18", feature = "dox"))]
    fn is_readonly(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_source_file_is_readonly(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn set_location<'a, P: IsA<gio::File> + 'a, Q: Into<Option<&'a P>>>(&self, location: Q) {
        let location = location.into();
        let location = location.to_glib_none();
        unsafe {
            ffi::gtk_source_file_set_location(self.to_glib_none().0, location.0);
        }
    }

    //#[cfg(any(feature = "v3_14", feature = "dox"))]
    //fn set_mount_operation_factory<'a, P: Into<Option</*Unimplemented*/Fundamental: Pointer>>, Q: Into<Option<&'a /*Ignored*/glib::DestroyNotify>>>(&self, callback: /*Unknown conversion*//*Unimplemented*/MountOperationFactory, user_data: P, notify: Q) {
    //    unsafe { TODO: call ffi::gtk_source_file_set_mount_operation_factory() }
    //}

    #[cfg(any(feature = "v3_18", feature = "dox"))]
    fn get_property_read_only(&self) -> bool {
        let mut value = Value::from(&false);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "read-only".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn connect_property_compression_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::compression-type",
                transmute(notify_compression_type_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn connect_property_encoding_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::encoding",
                transmute(notify_encoding_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn connect_property_location_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::location",
                transmute(notify_location_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn connect_property_newline_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::newline-type",
                transmute(notify_newline_type_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(any(feature = "v3_18", feature = "dox"))]
    fn connect_property_read_only_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::read-only",
                transmute(notify_read_only_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

#[cfg(any(feature = "v3_14", feature = "dox"))]
unsafe extern "C" fn notify_compression_type_trampoline<P>(this: *mut ffi::GtkSourceFile, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<File> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&File::from_glib_borrow(this).downcast_unchecked())
}

#[cfg(any(feature = "v3_14", feature = "dox"))]
unsafe extern "C" fn notify_encoding_trampoline<P>(this: *mut ffi::GtkSourceFile, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<File> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&File::from_glib_borrow(this).downcast_unchecked())
}

#[cfg(any(feature = "v3_14", feature = "dox"))]
unsafe extern "C" fn notify_location_trampoline<P>(this: *mut ffi::GtkSourceFile, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<File> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&File::from_glib_borrow(this).downcast_unchecked())
}

#[cfg(any(feature = "v3_14", feature = "dox"))]
unsafe extern "C" fn notify_newline_type_trampoline<P>(this: *mut ffi::GtkSourceFile, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<File> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&File::from_glib_borrow(this).downcast_unchecked())
}

#[cfg(any(feature = "v3_18", feature = "dox"))]
unsafe extern "C" fn notify_read_only_trampoline<P>(this: *mut ffi::GtkSourceFile, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<File> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&File::from_glib_borrow(this).downcast_unchecked())
}
