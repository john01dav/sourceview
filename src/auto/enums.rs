// This file was generated by gir (e58a8db) from gir-files (469db10)
// DO NOT EDIT

use ffi;
use glib::Type;
use glib::StaticType;
use glib::value::{Value, SetValue, FromValue, FromValueOptional};
use gobject_ffi;
use glib::translate::*;

#[cfg(any(feature = "v3_16", feature = "dox"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum BackgroundPatternType {
    None,
    Grid,
    #[doc(hidden)]
    __Unknown(i32),
}

#[cfg(any(feature = "v3_16", feature = "dox"))]
#[doc(hidden)]
impl ToGlib for BackgroundPatternType {
    type GlibType = ffi::GtkSourceBackgroundPatternType;

    fn to_glib(&self) -> ffi::GtkSourceBackgroundPatternType {
        match *self {
            BackgroundPatternType::None => ffi::GTK_SOURCE_BACKGROUND_PATTERN_TYPE_NONE,
            BackgroundPatternType::Grid => ffi::GTK_SOURCE_BACKGROUND_PATTERN_TYPE_GRID,
            BackgroundPatternType::__Unknown(value) => value
        }
    }
}

#[cfg(any(feature = "v3_16", feature = "dox"))]
#[doc(hidden)]
impl FromGlib<ffi::GtkSourceBackgroundPatternType> for BackgroundPatternType {
    fn from_glib(value: ffi::GtkSourceBackgroundPatternType) -> Self {
        match value {
            0 => BackgroundPatternType::None,
            1 => BackgroundPatternType::Grid,
            value => BackgroundPatternType::__Unknown(value),
        }
    }
}

#[cfg(any(feature = "v3_16", feature = "dox"))]
impl StaticType for BackgroundPatternType {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gtk_source_background_pattern_type_get_type()) }
    }
}

#[cfg(any(feature = "v3_16", feature = "dox"))]
impl<'a> FromValueOptional<'a> for BackgroundPatternType {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

#[cfg(any(feature = "v3_16", feature = "dox"))]
impl<'a> FromValue<'a> for BackgroundPatternType {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

#[cfg(any(feature = "v3_16", feature = "dox"))]
impl SetValue for BackgroundPatternType {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}

#[cfg(any(feature = "v3_12", feature = "dox"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum ChangeCaseType {
    Lower,
    Upper,
    Toggle,
    Title,
    #[doc(hidden)]
    __Unknown(i32),
}

#[cfg(any(feature = "v3_12", feature = "dox"))]
#[doc(hidden)]
impl ToGlib for ChangeCaseType {
    type GlibType = ffi::GtkSourceChangeCaseType;

    fn to_glib(&self) -> ffi::GtkSourceChangeCaseType {
        match *self {
            ChangeCaseType::Lower => ffi::GTK_SOURCE_CHANGE_CASE_LOWER,
            ChangeCaseType::Upper => ffi::GTK_SOURCE_CHANGE_CASE_UPPER,
            ChangeCaseType::Toggle => ffi::GTK_SOURCE_CHANGE_CASE_TOGGLE,
            ChangeCaseType::Title => ffi::GTK_SOURCE_CHANGE_CASE_TITLE,
            ChangeCaseType::__Unknown(value) => value
        }
    }
}

#[cfg(any(feature = "v3_12", feature = "dox"))]
#[doc(hidden)]
impl FromGlib<ffi::GtkSourceChangeCaseType> for ChangeCaseType {
    fn from_glib(value: ffi::GtkSourceChangeCaseType) -> Self {
        match value {
            0 => ChangeCaseType::Lower,
            1 => ChangeCaseType::Upper,
            2 => ChangeCaseType::Toggle,
            3 => ChangeCaseType::Title,
            value => ChangeCaseType::__Unknown(value),
        }
    }
}

#[cfg(any(feature = "v3_12", feature = "dox"))]
impl StaticType for ChangeCaseType {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gtk_source_change_case_type_get_type()) }
    }
}

#[cfg(any(feature = "v3_12", feature = "dox"))]
impl<'a> FromValueOptional<'a> for ChangeCaseType {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

#[cfg(any(feature = "v3_12", feature = "dox"))]
impl<'a> FromValue<'a> for ChangeCaseType {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

#[cfg(any(feature = "v3_12", feature = "dox"))]
impl SetValue for ChangeCaseType {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}

#[cfg(any(feature = "v3_14", feature = "dox"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum CompressionType {
    None,
    Gzip,
    #[doc(hidden)]
    __Unknown(i32),
}

#[cfg(any(feature = "v3_14", feature = "dox"))]
#[doc(hidden)]
impl ToGlib for CompressionType {
    type GlibType = ffi::GtkSourceCompressionType;

    fn to_glib(&self) -> ffi::GtkSourceCompressionType {
        match *self {
            CompressionType::None => ffi::GTK_SOURCE_COMPRESSION_TYPE_NONE,
            CompressionType::Gzip => ffi::GTK_SOURCE_COMPRESSION_TYPE_GZIP,
            CompressionType::__Unknown(value) => value
        }
    }
}

#[cfg(any(feature = "v3_14", feature = "dox"))]
#[doc(hidden)]
impl FromGlib<ffi::GtkSourceCompressionType> for CompressionType {
    fn from_glib(value: ffi::GtkSourceCompressionType) -> Self {
        match value {
            0 => CompressionType::None,
            1 => CompressionType::Gzip,
            value => CompressionType::__Unknown(value),
        }
    }
}

#[cfg(any(feature = "v3_14", feature = "dox"))]
impl StaticType for CompressionType {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gtk_source_compression_type_get_type()) }
    }
}

#[cfg(any(feature = "v3_14", feature = "dox"))]
impl<'a> FromValueOptional<'a> for CompressionType {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

#[cfg(any(feature = "v3_14", feature = "dox"))]
impl<'a> FromValue<'a> for CompressionType {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

#[cfg(any(feature = "v3_14", feature = "dox"))]
impl SetValue for CompressionType {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum GutterRendererAlignmentMode {
    Cell,
    First,
    Last,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl ToGlib for GutterRendererAlignmentMode {
    type GlibType = ffi::GtkSourceGutterRendererAlignmentMode;

    fn to_glib(&self) -> ffi::GtkSourceGutterRendererAlignmentMode {
        match *self {
            GutterRendererAlignmentMode::Cell => ffi::GTK_SOURCE_GUTTER_RENDERER_ALIGNMENT_MODE_CELL,
            GutterRendererAlignmentMode::First => ffi::GTK_SOURCE_GUTTER_RENDERER_ALIGNMENT_MODE_FIRST,
            GutterRendererAlignmentMode::Last => ffi::GTK_SOURCE_GUTTER_RENDERER_ALIGNMENT_MODE_LAST,
            GutterRendererAlignmentMode::__Unknown(value) => value
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GtkSourceGutterRendererAlignmentMode> for GutterRendererAlignmentMode {
    fn from_glib(value: ffi::GtkSourceGutterRendererAlignmentMode) -> Self {
        match value {
            0 => GutterRendererAlignmentMode::Cell,
            1 => GutterRendererAlignmentMode::First,
            2 => GutterRendererAlignmentMode::Last,
            value => GutterRendererAlignmentMode::__Unknown(value),
        }
    }
}

impl StaticType for GutterRendererAlignmentMode {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gtk_source_gutter_renderer_alignment_mode_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for GutterRendererAlignmentMode {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for GutterRendererAlignmentMode {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl SetValue for GutterRendererAlignmentMode {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}

#[cfg(any(feature = "v3_14", feature = "dox"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum NewlineType {
    Lf,
    Cr,
    CrLf,
    #[doc(hidden)]
    __Unknown(i32),
}

#[cfg(any(feature = "v3_14", feature = "dox"))]
#[doc(hidden)]
impl ToGlib for NewlineType {
    type GlibType = ffi::GtkSourceNewlineType;

    fn to_glib(&self) -> ffi::GtkSourceNewlineType {
        match *self {
            NewlineType::Lf => ffi::GTK_SOURCE_NEWLINE_TYPE_LF,
            NewlineType::Cr => ffi::GTK_SOURCE_NEWLINE_TYPE_CR,
            NewlineType::CrLf => ffi::GTK_SOURCE_NEWLINE_TYPE_CR_LF,
            NewlineType::__Unknown(value) => value
        }
    }
}

#[cfg(any(feature = "v3_14", feature = "dox"))]
#[doc(hidden)]
impl FromGlib<ffi::GtkSourceNewlineType> for NewlineType {
    fn from_glib(value: ffi::GtkSourceNewlineType) -> Self {
        match value {
            0 => NewlineType::Lf,
            1 => NewlineType::Cr,
            2 => NewlineType::CrLf,
            value => NewlineType::__Unknown(value),
        }
    }
}

#[cfg(any(feature = "v3_14", feature = "dox"))]
impl StaticType for NewlineType {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gtk_source_newline_type_get_type()) }
    }
}

#[cfg(any(feature = "v3_14", feature = "dox"))]
impl<'a> FromValueOptional<'a> for NewlineType {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

#[cfg(any(feature = "v3_14", feature = "dox"))]
impl<'a> FromValue<'a> for NewlineType {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

#[cfg(any(feature = "v3_14", feature = "dox"))]
impl SetValue for NewlineType {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum SmartHomeEndType {
    Disabled,
    Before,
    After,
    Always,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl ToGlib for SmartHomeEndType {
    type GlibType = ffi::GtkSourceSmartHomeEndType;

    fn to_glib(&self) -> ffi::GtkSourceSmartHomeEndType {
        match *self {
            SmartHomeEndType::Disabled => ffi::GTK_SOURCE_SMART_HOME_END_DISABLED,
            SmartHomeEndType::Before => ffi::GTK_SOURCE_SMART_HOME_END_BEFORE,
            SmartHomeEndType::After => ffi::GTK_SOURCE_SMART_HOME_END_AFTER,
            SmartHomeEndType::Always => ffi::GTK_SOURCE_SMART_HOME_END_ALWAYS,
            SmartHomeEndType::__Unknown(value) => value
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GtkSourceSmartHomeEndType> for SmartHomeEndType {
    fn from_glib(value: ffi::GtkSourceSmartHomeEndType) -> Self {
        match value {
            0 => SmartHomeEndType::Disabled,
            1 => SmartHomeEndType::Before,
            2 => SmartHomeEndType::After,
            3 => SmartHomeEndType::Always,
            value => SmartHomeEndType::__Unknown(value),
        }
    }
}

impl StaticType for SmartHomeEndType {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gtk_source_smart_home_end_type_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for SmartHomeEndType {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for SmartHomeEndType {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl SetValue for SmartHomeEndType {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}

