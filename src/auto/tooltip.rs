// This file was generated by gir (e6cb5d0) from gir-files (11e0e6d)
// DO NOT EDIT

use Rectangle;
use Widget;
use ffi;
use gdk;
use gdk_pixbuf;
use glib::object::IsA;
use glib::translate::*;

glib_wrapper! {
    pub struct Tooltip(Object<ffi::GtkTooltip>);

    match fn {
        get_type => || ffi::gtk_tooltip_get_type(),
    }
}

impl Tooltip {
    pub fn set_custom<T: IsA<Widget>>(&self, custom_widget: Option<&T>) {
        unsafe {
            ffi::gtk_tooltip_set_custom(self.to_glib_none().0, custom_widget.to_glib_none().0);
        }
    }

    pub fn set_icon(&self, pixbuf: Option<&gdk_pixbuf::Pixbuf>) {
        unsafe {
            ffi::gtk_tooltip_set_icon(self.to_glib_none().0, pixbuf.to_glib_none().0);
        }
    }

    //pub fn set_icon_from_gicon<T: IsA</*Ignored*/gio::Icon>>(&self, gicon: Option<&T>, size: i32) {
    //    unsafe { TODO: call ffi::gtk_tooltip_set_icon_from_gicon() }
    //}

    pub fn set_icon_from_icon_name(&self, icon_name: Option<&str>, size: i32) {
        unsafe {
            ffi::gtk_tooltip_set_icon_from_icon_name(self.to_glib_none().0, icon_name.to_glib_none().0, size);
        }
    }

    pub fn set_icon_from_stock(&self, stock_id: Option<&str>, size: i32) {
        unsafe {
            ffi::gtk_tooltip_set_icon_from_stock(self.to_glib_none().0, stock_id.to_glib_none().0, size);
        }
    }

    pub fn set_markup(&self, markup: Option<&str>) {
        unsafe {
            ffi::gtk_tooltip_set_markup(self.to_glib_none().0, markup.to_glib_none().0);
        }
    }

    pub fn set_text(&self, text: Option<&str>) {
        unsafe {
            ffi::gtk_tooltip_set_text(self.to_glib_none().0, text.to_glib_none().0);
        }
    }

    pub fn set_tip_area(&self, rect: &Rectangle) {
        unsafe {
            ffi::gtk_tooltip_set_tip_area(self.to_glib_none().0, rect.to_glib_none().0);
        }
    }

    pub fn trigger_tooltip_query(display: &gdk::Display) {
        assert_initialized_main_thread!();
        unsafe {
            ffi::gtk_tooltip_trigger_tooltip_query(display.to_glib_none().0);
        }
    }
}
