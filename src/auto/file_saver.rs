// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

#[cfg(any(feature = "v3_14", feature = "dox"))]
use Buffer;
#[cfg(any(feature = "v3_14", feature = "dox"))]
use CompressionType;
#[cfg(any(feature = "v3_14", feature = "dox"))]
use Encoding;
#[cfg(any(feature = "v3_14", feature = "dox"))]
use File;
#[cfg(any(feature = "v3_14", feature = "dox"))]
use FileSaverFlags;
#[cfg(any(feature = "v3_14", feature = "dox"))]
use NewlineType;
use ffi;
#[cfg(any(feature = "v3_14", feature = "dox"))]
use gio;
use glib;
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
    pub struct FileSaver(Object<ffi::GtkSourceFileSaver, ffi::GtkSourceFileSaverClass>);

    match fn {
        get_type => || ffi::gtk_source_file_saver_get_type(),
    }
}

impl FileSaver {
    #[cfg(any(feature = "v3_14", feature = "dox"))]
    pub fn new(buffer: &Buffer, file: &File) -> FileSaver {
        skip_assert_initialized!();
        unsafe {
            from_glib_full(ffi::gtk_source_file_saver_new(buffer.to_glib_none().0, file.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    pub fn new_with_target<P: IsA<gio::File>>(buffer: &Buffer, file: &File, target_location: &P) -> FileSaver {
        skip_assert_initialized!();
        unsafe {
            from_glib_full(ffi::gtk_source_file_saver_new_with_target(buffer.to_glib_none().0, file.to_glib_none().0, target_location.to_glib_none().0))
        }
    }
}

pub trait FileSaverExt {
    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn get_buffer(&self) -> Option<Buffer>;

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn get_compression_type(&self) -> CompressionType;

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn get_encoding(&self) -> Option<Encoding>;

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn get_file(&self) -> Option<File>;

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn get_flags(&self) -> FileSaverFlags;

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn get_location(&self) -> Option<gio::File>;

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn get_newline_type(&self) -> NewlineType;

    //#[cfg(any(feature = "v3_14", feature = "dox"))]
    //fn save_async<'a, 'b, 'c, P: Into<Option<&'a gio::Cancellable>>, Q: Into<Option<&'b /*Ignored*/gio::FileProgressCallback>>, R: Into<Option</*Unimplemented*/Fundamental: Pointer>>, S: Into<Option<&'c /*Ignored*/glib::DestroyNotify>>, T: FnOnce(Result<(), Error>) + Send + 'static>(&self, io_priority: /*Ignored*/glib::Priority, cancellable: P, progress_callback: Q, progress_callback_data: R, progress_callback_notify: S, callback: T);

    //#[cfg(feature = "futures")]
    //#[cfg(any(feature = "v3_14", feature = "dox"))]
    //fn save_async_future<'b, 'c, Q: Into<Option<&'b /*Ignored*/gio::FileProgressCallback>>, R: Into<Option</*Unimplemented*/Fundamental: Pointer>>, S: Into<Option<&'c /*Ignored*/glib::DestroyNotify>>>(&self, io_priority: /*Ignored*/glib::Priority, progress_callback: Q, progress_callback_data: R, progress_callback_notify: S) -> Box_<futures_core::Future<Item = (Self, ()), Error = (Self, Error)>>;

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn set_compression_type(&self, compression_type: CompressionType);

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn set_encoding<'a, P: Into<Option<&'a Encoding>>>(&self, encoding: P);

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn set_flags(&self, flags: FileSaverFlags);

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn set_newline_type(&self, newline_type: NewlineType);

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn connect_property_buffer_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn connect_property_compression_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn connect_property_encoding_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn connect_property_file_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn connect_property_flags_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn connect_property_location_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn connect_property_newline_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<FileSaver> + IsA<glib::object::Object>> FileSaverExt for O {
    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn get_buffer(&self) -> Option<Buffer> {
        unsafe {
            from_glib_none(ffi::gtk_source_file_saver_get_buffer(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn get_compression_type(&self) -> CompressionType {
        unsafe {
            from_glib(ffi::gtk_source_file_saver_get_compression_type(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn get_encoding(&self) -> Option<Encoding> {
        unsafe {
            from_glib_none(ffi::gtk_source_file_saver_get_encoding(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn get_file(&self) -> Option<File> {
        unsafe {
            from_glib_none(ffi::gtk_source_file_saver_get_file(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn get_flags(&self) -> FileSaverFlags {
        unsafe {
            from_glib(ffi::gtk_source_file_saver_get_flags(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn get_location(&self) -> Option<gio::File> {
        unsafe {
            from_glib_none(ffi::gtk_source_file_saver_get_location(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn get_newline_type(&self) -> NewlineType {
        unsafe {
            from_glib(ffi::gtk_source_file_saver_get_newline_type(self.to_glib_none().0))
        }
    }

    //#[cfg(any(feature = "v3_14", feature = "dox"))]
    //fn save_async<'a, 'b, 'c, P: Into<Option<&'a gio::Cancellable>>, Q: Into<Option<&'b /*Ignored*/gio::FileProgressCallback>>, R: Into<Option</*Unimplemented*/Fundamental: Pointer>>, S: Into<Option<&'c /*Ignored*/glib::DestroyNotify>>, T: FnOnce(Result<(), Error>) + Send + 'static>(&self, io_priority: /*Ignored*/glib::Priority, cancellable: P, progress_callback: Q, progress_callback_data: R, progress_callback_notify: S, callback: T) {
    //    unsafe { TODO: call ffi::gtk_source_file_saver_save_async() }
    //}

    //#[cfg(feature = "futures")]
    //#[cfg(any(feature = "v3_14", feature = "dox"))]
    //fn save_async_future<'b, 'c, Q: Into<Option<&'b /*Ignored*/gio::FileProgressCallback>>, R: Into<Option</*Unimplemented*/Fundamental: Pointer>>, S: Into<Option<&'c /*Ignored*/glib::DestroyNotify>>>(&self, io_priority: /*Ignored*/glib::Priority, progress_callback: Q, progress_callback_data: R, progress_callback_notify: S) -> Box_<futures_core::Future<Item = (Self, ()), Error = (Self, Error)>> {
        //use gio::GioFuture;
        //use send_cell::SendCell;

        //let progress_callback = progress_callback.into();
        //let progress_callback = progress_callback.map(ToOwned::to_owned);
        //let progress_callback_data = progress_callback_data.into();
        //let progress_callback_data = progress_callback_data.map(ToOwned::to_owned);
        //let progress_callback_notify = progress_callback_notify.into();
        //let progress_callback_notify = progress_callback_notify.map(ToOwned::to_owned);
        //GioFuture::new(self, move |obj, send| {
        //    let cancellable = gio::Cancellable::new();
        //    let send = SendCell::new(send);
        //    let obj_clone = SendCell::new(obj.clone());
        //    obj.save_async(
        //         io_priority,
        //         Some(&cancellable),
        //         progress_callback.as_ref().map(::std::borrow::Borrow::borrow),
        //         progress_callback_data.as_ref().map(::std::borrow::Borrow::borrow),
        //         progress_callback_notify.as_ref().map(::std::borrow::Borrow::borrow),
        //         move |res| {
        //             let obj = obj_clone.into_inner();
        //             let res = res.map(|v| (obj.clone(), v)).map_err(|v| (obj.clone(), v));
        //             let _ = send.into_inner().send(res);
        //         },
        //    );

        //    cancellable
        //})
    //}

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn set_compression_type(&self, compression_type: CompressionType) {
        unsafe {
            ffi::gtk_source_file_saver_set_compression_type(self.to_glib_none().0, compression_type.to_glib());
        }
    }

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn set_encoding<'a, P: Into<Option<&'a Encoding>>>(&self, encoding: P) {
        let encoding = encoding.into();
        let encoding = encoding.to_glib_none();
        unsafe {
            ffi::gtk_source_file_saver_set_encoding(self.to_glib_none().0, encoding.0);
        }
    }

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn set_flags(&self, flags: FileSaverFlags) {
        unsafe {
            ffi::gtk_source_file_saver_set_flags(self.to_glib_none().0, flags.to_glib());
        }
    }

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn set_newline_type(&self, newline_type: NewlineType) {
        unsafe {
            ffi::gtk_source_file_saver_set_newline_type(self.to_glib_none().0, newline_type.to_glib());
        }
    }

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn connect_property_buffer_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::buffer",
                transmute(notify_buffer_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
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
    fn connect_property_file_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::file",
                transmute(notify_file_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn connect_property_flags_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::flags",
                transmute(notify_flags_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
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
}

#[cfg(any(feature = "v3_14", feature = "dox"))]
unsafe extern "C" fn notify_buffer_trampoline<P>(this: *mut ffi::GtkSourceFileSaver, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<FileSaver> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&FileSaver::from_glib_borrow(this).downcast_unchecked())
}

#[cfg(any(feature = "v3_14", feature = "dox"))]
unsafe extern "C" fn notify_compression_type_trampoline<P>(this: *mut ffi::GtkSourceFileSaver, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<FileSaver> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&FileSaver::from_glib_borrow(this).downcast_unchecked())
}

#[cfg(any(feature = "v3_14", feature = "dox"))]
unsafe extern "C" fn notify_encoding_trampoline<P>(this: *mut ffi::GtkSourceFileSaver, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<FileSaver> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&FileSaver::from_glib_borrow(this).downcast_unchecked())
}

#[cfg(any(feature = "v3_14", feature = "dox"))]
unsafe extern "C" fn notify_file_trampoline<P>(this: *mut ffi::GtkSourceFileSaver, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<FileSaver> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&FileSaver::from_glib_borrow(this).downcast_unchecked())
}

#[cfg(any(feature = "v3_14", feature = "dox"))]
unsafe extern "C" fn notify_flags_trampoline<P>(this: *mut ffi::GtkSourceFileSaver, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<FileSaver> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&FileSaver::from_glib_borrow(this).downcast_unchecked())
}

#[cfg(any(feature = "v3_14", feature = "dox"))]
unsafe extern "C" fn notify_location_trampoline<P>(this: *mut ffi::GtkSourceFileSaver, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<FileSaver> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&FileSaver::from_glib_borrow(this).downcast_unchecked())
}

#[cfg(any(feature = "v3_14", feature = "dox"))]
unsafe extern "C" fn notify_newline_type_trampoline<P>(this: *mut ffi::GtkSourceFileSaver, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<FileSaver> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&FileSaver::from_glib_borrow(this).downcast_unchecked())
}
