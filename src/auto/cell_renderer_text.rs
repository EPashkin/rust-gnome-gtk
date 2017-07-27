// This file was generated by gir (127a851) from gir-files (0bcaef9)
// DO NOT EDIT

use CellRenderer;
use TreePath;
use ffi;
use gdk;
use glib;
use glib::Value;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use libc;
use pango;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct CellRendererText(Object<ffi::GtkCellRendererText>): CellRenderer;

    match fn {
        get_type => || ffi::gtk_cell_renderer_text_get_type(),
    }
}

impl CellRendererText {
    pub fn new() -> CellRendererText {
        assert_initialized_main_thread!();
        unsafe {
            CellRenderer::from_glib_none(ffi::gtk_cell_renderer_text_new()).downcast_unchecked()
        }
    }
}

pub trait CellRendererTextExt {
    fn set_fixed_height_from_font(&self, number_of_rows: i32);

    fn get_property_align_set(&self) -> bool;

    fn set_property_align_set(&self, align_set: bool);

    fn set_property_background(&self, background: Option<&str>);

    fn get_property_background_rgba(&self) -> Option<gdk::RGBA>;

    fn set_property_background_rgba(&self, background_rgba: Option<&gdk::RGBA>);

    fn get_property_background_set(&self) -> bool;

    fn set_property_background_set(&self, background_set: bool);

    fn get_property_editable(&self) -> bool;

    fn set_property_editable(&self, editable: bool);

    fn get_property_editable_set(&self) -> bool;

    fn set_property_editable_set(&self, editable_set: bool);

    fn get_property_ellipsize(&self) -> pango::EllipsizeMode;

    fn set_property_ellipsize(&self, ellipsize: pango::EllipsizeMode);

    fn get_property_ellipsize_set(&self) -> bool;

    fn set_property_ellipsize_set(&self, ellipsize_set: bool);

    fn get_property_family(&self) -> Option<String>;

    fn set_property_family(&self, family: Option<&str>);

    fn get_property_family_set(&self) -> bool;

    fn set_property_family_set(&self, family_set: bool);

    fn get_property_font(&self) -> Option<String>;

    fn set_property_font(&self, font: Option<&str>);

    fn set_property_foreground(&self, foreground: Option<&str>);

    fn get_property_foreground_rgba(&self) -> Option<gdk::RGBA>;

    fn set_property_foreground_rgba(&self, foreground_rgba: Option<&gdk::RGBA>);

    fn get_property_foreground_set(&self) -> bool;

    fn set_property_foreground_set(&self, foreground_set: bool);

    fn get_property_language(&self) -> Option<String>;

    fn set_property_language(&self, language: Option<&str>);

    fn get_property_language_set(&self) -> bool;

    fn set_property_language_set(&self, language_set: bool);

    fn set_property_markup(&self, markup: Option<&str>);

    fn get_property_max_width_chars(&self) -> i32;

    fn set_property_max_width_chars(&self, max_width_chars: i32);

    fn get_property_placeholder_text(&self) -> Option<String>;

    fn set_property_placeholder_text(&self, placeholder_text: Option<&str>);

    fn get_property_rise(&self) -> i32;

    fn set_property_rise(&self, rise: i32);

    fn get_property_rise_set(&self) -> bool;

    fn set_property_rise_set(&self, rise_set: bool);

    fn get_property_scale(&self) -> f64;

    fn set_property_scale(&self, scale: f64);

    fn get_property_scale_set(&self) -> bool;

    fn set_property_scale_set(&self, scale_set: bool);

    fn get_property_single_paragraph_mode(&self) -> bool;

    fn set_property_single_paragraph_mode(&self, single_paragraph_mode: bool);

    fn get_property_size(&self) -> i32;

    fn set_property_size(&self, size: i32);

    fn get_property_size_points(&self) -> f64;

    fn set_property_size_points(&self, size_points: f64);

    fn get_property_size_set(&self) -> bool;

    fn set_property_size_set(&self, size_set: bool);

    fn get_property_stretch(&self) -> pango::Stretch;

    fn set_property_stretch(&self, stretch: pango::Stretch);

    fn get_property_stretch_set(&self) -> bool;

    fn set_property_stretch_set(&self, stretch_set: bool);

    fn get_property_strikethrough(&self) -> bool;

    fn set_property_strikethrough(&self, strikethrough: bool);

    fn get_property_strikethrough_set(&self) -> bool;

    fn set_property_strikethrough_set(&self, strikethrough_set: bool);

    fn get_property_style(&self) -> pango::Style;

    fn set_property_style(&self, style: pango::Style);

    fn get_property_style_set(&self) -> bool;

    fn set_property_style_set(&self, style_set: bool);

    fn get_property_text(&self) -> Option<String>;

    fn set_property_text(&self, text: Option<&str>);

    fn get_property_underline(&self) -> pango::Underline;

    fn set_property_underline(&self, underline: pango::Underline);

    fn get_property_underline_set(&self) -> bool;

    fn set_property_underline_set(&self, underline_set: bool);

    fn get_property_variant(&self) -> pango::Variant;

    fn set_property_variant(&self, variant: pango::Variant);

    fn get_property_variant_set(&self) -> bool;

    fn set_property_variant_set(&self, variant_set: bool);

    fn get_property_weight(&self) -> i32;

    fn set_property_weight(&self, weight: i32);

    fn get_property_weight_set(&self) -> bool;

    fn set_property_weight_set(&self, weight_set: bool);

    fn get_property_width_chars(&self) -> i32;

    fn set_property_width_chars(&self, width_chars: i32);

    fn get_property_wrap_mode(&self) -> pango::WrapMode;

    fn set_property_wrap_mode(&self, wrap_mode: pango::WrapMode);

    fn get_property_wrap_width(&self) -> i32;

    fn set_property_wrap_width(&self, wrap_width: i32);

    fn connect_edited<F: Fn(&Self, TreePath, &str) + 'static>(&self, f: F) -> u64;
}

impl<O: IsA<CellRendererText> + IsA<glib::object::Object>> CellRendererTextExt for O {
    fn set_fixed_height_from_font(&self, number_of_rows: i32) {
        unsafe {
            ffi::gtk_cell_renderer_text_set_fixed_height_from_font(self.to_glib_none().0, number_of_rows);
        }
    }

    fn get_property_align_set(&self) -> bool {
        let mut value = Value::from(&false);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "align-set".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn set_property_align_set(&self, align_set: bool) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "align-set".to_glib_none().0, Value::from(&align_set).to_glib_none().0);
        }
    }

    fn set_property_background(&self, background: Option<&str>) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "background".to_glib_none().0, Value::from(background).to_glib_none().0);
        }
    }

    fn get_property_background_rgba(&self) -> Option<gdk::RGBA> {
        let mut value = Value::from(None::<&gdk::RGBA>);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "background-rgba".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get()
    }

    fn set_property_background_rgba(&self, background_rgba: Option<&gdk::RGBA>) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "background-rgba".to_glib_none().0, Value::from(background_rgba).to_glib_none().0);
        }
    }

    fn get_property_background_set(&self) -> bool {
        let mut value = Value::from(&false);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "background-set".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn set_property_background_set(&self, background_set: bool) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "background-set".to_glib_none().0, Value::from(&background_set).to_glib_none().0);
        }
    }

    fn get_property_editable(&self) -> bool {
        let mut value = Value::from(&false);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "editable".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn set_property_editable(&self, editable: bool) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "editable".to_glib_none().0, Value::from(&editable).to_glib_none().0);
        }
    }

    fn get_property_editable_set(&self) -> bool {
        let mut value = Value::from(&false);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "editable-set".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn set_property_editable_set(&self, editable_set: bool) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "editable-set".to_glib_none().0, Value::from(&editable_set).to_glib_none().0);
        }
    }

    fn get_property_ellipsize(&self) -> pango::EllipsizeMode {
        let mut value = Value::from(&0);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "ellipsize".to_glib_none().0, value.to_glib_none_mut().0);
            from_glib(transmute(value.get::<i32>().unwrap()))
        }
    }

    fn set_property_ellipsize(&self, ellipsize: pango::EllipsizeMode) {
        let ellipsize = ellipsize.to_glib() as i32;
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "ellipsize".to_glib_none().0, Value::from(&ellipsize).to_glib_none().0);
        }
    }

    fn get_property_ellipsize_set(&self) -> bool {
        let mut value = Value::from(&false);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "ellipsize-set".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn set_property_ellipsize_set(&self, ellipsize_set: bool) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "ellipsize-set".to_glib_none().0, Value::from(&ellipsize_set).to_glib_none().0);
        }
    }

    fn get_property_family(&self) -> Option<String> {
        let mut value = Value::from(None::<&str>);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "family".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get()
    }

    fn set_property_family(&self, family: Option<&str>) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "family".to_glib_none().0, Value::from(family).to_glib_none().0);
        }
    }

    fn get_property_family_set(&self) -> bool {
        let mut value = Value::from(&false);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "family-set".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn set_property_family_set(&self, family_set: bool) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "family-set".to_glib_none().0, Value::from(&family_set).to_glib_none().0);
        }
    }

    fn get_property_font(&self) -> Option<String> {
        let mut value = Value::from(None::<&str>);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "font".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get()
    }

    fn set_property_font(&self, font: Option<&str>) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "font".to_glib_none().0, Value::from(font).to_glib_none().0);
        }
    }

    fn set_property_foreground(&self, foreground: Option<&str>) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "foreground".to_glib_none().0, Value::from(foreground).to_glib_none().0);
        }
    }

    fn get_property_foreground_rgba(&self) -> Option<gdk::RGBA> {
        let mut value = Value::from(None::<&gdk::RGBA>);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "foreground-rgba".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get()
    }

    fn set_property_foreground_rgba(&self, foreground_rgba: Option<&gdk::RGBA>) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "foreground-rgba".to_glib_none().0, Value::from(foreground_rgba).to_glib_none().0);
        }
    }

    fn get_property_foreground_set(&self) -> bool {
        let mut value = Value::from(&false);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "foreground-set".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn set_property_foreground_set(&self, foreground_set: bool) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "foreground-set".to_glib_none().0, Value::from(&foreground_set).to_glib_none().0);
        }
    }

    fn get_property_language(&self) -> Option<String> {
        let mut value = Value::from(None::<&str>);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "language".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get()
    }

    fn set_property_language(&self, language: Option<&str>) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "language".to_glib_none().0, Value::from(language).to_glib_none().0);
        }
    }

    fn get_property_language_set(&self) -> bool {
        let mut value = Value::from(&false);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "language-set".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn set_property_language_set(&self, language_set: bool) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "language-set".to_glib_none().0, Value::from(&language_set).to_glib_none().0);
        }
    }

    fn set_property_markup(&self, markup: Option<&str>) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "markup".to_glib_none().0, Value::from(markup).to_glib_none().0);
        }
    }

    fn get_property_max_width_chars(&self) -> i32 {
        let mut value = Value::from(&0);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "max-width-chars".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn set_property_max_width_chars(&self, max_width_chars: i32) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "max-width-chars".to_glib_none().0, Value::from(&max_width_chars).to_glib_none().0);
        }
    }

    fn get_property_placeholder_text(&self) -> Option<String> {
        let mut value = Value::from(None::<&str>);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "placeholder-text".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get()
    }

    fn set_property_placeholder_text(&self, placeholder_text: Option<&str>) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "placeholder-text".to_glib_none().0, Value::from(placeholder_text).to_glib_none().0);
        }
    }

    fn get_property_rise(&self) -> i32 {
        let mut value = Value::from(&0);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "rise".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn set_property_rise(&self, rise: i32) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "rise".to_glib_none().0, Value::from(&rise).to_glib_none().0);
        }
    }

    fn get_property_rise_set(&self) -> bool {
        let mut value = Value::from(&false);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "rise-set".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn set_property_rise_set(&self, rise_set: bool) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "rise-set".to_glib_none().0, Value::from(&rise_set).to_glib_none().0);
        }
    }

    fn get_property_scale(&self) -> f64 {
        let mut value = Value::from(&0f64);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "scale".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn set_property_scale(&self, scale: f64) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "scale".to_glib_none().0, Value::from(&scale).to_glib_none().0);
        }
    }

    fn get_property_scale_set(&self) -> bool {
        let mut value = Value::from(&false);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "scale-set".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn set_property_scale_set(&self, scale_set: bool) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "scale-set".to_glib_none().0, Value::from(&scale_set).to_glib_none().0);
        }
    }

    fn get_property_single_paragraph_mode(&self) -> bool {
        let mut value = Value::from(&false);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "single-paragraph-mode".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn set_property_single_paragraph_mode(&self, single_paragraph_mode: bool) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "single-paragraph-mode".to_glib_none().0, Value::from(&single_paragraph_mode).to_glib_none().0);
        }
    }

    fn get_property_size(&self) -> i32 {
        let mut value = Value::from(&0);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "size".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn set_property_size(&self, size: i32) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "size".to_glib_none().0, Value::from(&size).to_glib_none().0);
        }
    }

    fn get_property_size_points(&self) -> f64 {
        let mut value = Value::from(&0f64);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "size-points".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn set_property_size_points(&self, size_points: f64) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "size-points".to_glib_none().0, Value::from(&size_points).to_glib_none().0);
        }
    }

    fn get_property_size_set(&self) -> bool {
        let mut value = Value::from(&false);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "size-set".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn set_property_size_set(&self, size_set: bool) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "size-set".to_glib_none().0, Value::from(&size_set).to_glib_none().0);
        }
    }

    fn get_property_stretch(&self) -> pango::Stretch {
        let mut value = Value::from(&0);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "stretch".to_glib_none().0, value.to_glib_none_mut().0);
            from_glib(transmute(value.get::<i32>().unwrap()))
        }
    }

    fn set_property_stretch(&self, stretch: pango::Stretch) {
        let stretch = stretch.to_glib() as i32;
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "stretch".to_glib_none().0, Value::from(&stretch).to_glib_none().0);
        }
    }

    fn get_property_stretch_set(&self) -> bool {
        let mut value = Value::from(&false);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "stretch-set".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn set_property_stretch_set(&self, stretch_set: bool) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "stretch-set".to_glib_none().0, Value::from(&stretch_set).to_glib_none().0);
        }
    }

    fn get_property_strikethrough(&self) -> bool {
        let mut value = Value::from(&false);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "strikethrough".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn set_property_strikethrough(&self, strikethrough: bool) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "strikethrough".to_glib_none().0, Value::from(&strikethrough).to_glib_none().0);
        }
    }

    fn get_property_strikethrough_set(&self) -> bool {
        let mut value = Value::from(&false);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "strikethrough-set".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn set_property_strikethrough_set(&self, strikethrough_set: bool) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "strikethrough-set".to_glib_none().0, Value::from(&strikethrough_set).to_glib_none().0);
        }
    }

    fn get_property_style(&self) -> pango::Style {
        let mut value = Value::from(&0);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "style".to_glib_none().0, value.to_glib_none_mut().0);
            from_glib(transmute(value.get::<i32>().unwrap()))
        }
    }

    fn set_property_style(&self, style: pango::Style) {
        let style = style.to_glib() as i32;
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "style".to_glib_none().0, Value::from(&style).to_glib_none().0);
        }
    }

    fn get_property_style_set(&self) -> bool {
        let mut value = Value::from(&false);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "style-set".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn set_property_style_set(&self, style_set: bool) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "style-set".to_glib_none().0, Value::from(&style_set).to_glib_none().0);
        }
    }

    fn get_property_text(&self) -> Option<String> {
        let mut value = Value::from(None::<&str>);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "text".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get()
    }

    fn set_property_text(&self, text: Option<&str>) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "text".to_glib_none().0, Value::from(text).to_glib_none().0);
        }
    }

    fn get_property_underline(&self) -> pango::Underline {
        let mut value = Value::from(&0);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "underline".to_glib_none().0, value.to_glib_none_mut().0);
            from_glib(transmute(value.get::<i32>().unwrap()))
        }
    }

    fn set_property_underline(&self, underline: pango::Underline) {
        let underline = underline.to_glib() as i32;
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "underline".to_glib_none().0, Value::from(&underline).to_glib_none().0);
        }
    }

    fn get_property_underline_set(&self) -> bool {
        let mut value = Value::from(&false);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "underline-set".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn set_property_underline_set(&self, underline_set: bool) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "underline-set".to_glib_none().0, Value::from(&underline_set).to_glib_none().0);
        }
    }

    fn get_property_variant(&self) -> pango::Variant {
        let mut value = Value::from(&0);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "variant".to_glib_none().0, value.to_glib_none_mut().0);
            from_glib(transmute(value.get::<i32>().unwrap()))
        }
    }

    fn set_property_variant(&self, variant: pango::Variant) {
        let variant = variant.to_glib() as i32;
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "variant".to_glib_none().0, Value::from(&variant).to_glib_none().0);
        }
    }

    fn get_property_variant_set(&self) -> bool {
        let mut value = Value::from(&false);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "variant-set".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn set_property_variant_set(&self, variant_set: bool) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "variant-set".to_glib_none().0, Value::from(&variant_set).to_glib_none().0);
        }
    }

    fn get_property_weight(&self) -> i32 {
        let mut value = Value::from(&0);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "weight".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn set_property_weight(&self, weight: i32) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "weight".to_glib_none().0, Value::from(&weight).to_glib_none().0);
        }
    }

    fn get_property_weight_set(&self) -> bool {
        let mut value = Value::from(&false);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "weight-set".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn set_property_weight_set(&self, weight_set: bool) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "weight-set".to_glib_none().0, Value::from(&weight_set).to_glib_none().0);
        }
    }

    fn get_property_width_chars(&self) -> i32 {
        let mut value = Value::from(&0);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "width-chars".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn set_property_width_chars(&self, width_chars: i32) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "width-chars".to_glib_none().0, Value::from(&width_chars).to_glib_none().0);
        }
    }

    fn get_property_wrap_mode(&self) -> pango::WrapMode {
        let mut value = Value::from(&0);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "wrap-mode".to_glib_none().0, value.to_glib_none_mut().0);
            from_glib(transmute(value.get::<i32>().unwrap()))
        }
    }

    fn set_property_wrap_mode(&self, wrap_mode: pango::WrapMode) {
        let wrap_mode = wrap_mode.to_glib() as i32;
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "wrap-mode".to_glib_none().0, Value::from(&wrap_mode).to_glib_none().0);
        }
    }

    fn get_property_wrap_width(&self) -> i32 {
        let mut value = Value::from(&0);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "wrap-width".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn set_property_wrap_width(&self, wrap_width: i32) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "wrap-width".to_glib_none().0, Value::from(&wrap_width).to_glib_none().0);
        }
    }

    fn connect_edited<F: Fn(&Self, TreePath, &str) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self, TreePath, &str) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "edited",
                transmute(edited_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn edited_trampoline<P>(this: *mut ffi::GtkCellRendererText, path: *mut libc::c_char, new_text: *mut libc::c_char, f: glib_ffi::gpointer)
where P: IsA<CellRendererText> {
    callback_guard!();
    let f: &Box_<Fn(&P, TreePath, &str) + 'static> = transmute(f);
    let path = from_glib_full(ffi::gtk_tree_path_new_from_string(path));
    f(&CellRendererText::from_glib_none(this).downcast_unchecked(), path, &String::from_glib_none(new_text))
}
