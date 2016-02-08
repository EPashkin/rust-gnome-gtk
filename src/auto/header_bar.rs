// This file was generated by gir (c9185c9) from gir-files (11e0e6d)
// DO NOT EDIT

use Buildable;
use Container;
use Widget;
use ffi;
use glib::object::Downcast;
use glib::object::IsA;
use glib::translate::*;

glib_wrapper! {
    pub struct HeaderBar(Object<ffi::GtkHeaderBar>): Widget, Container, Buildable;

    match fn {
        get_type => || ffi::gtk_header_bar_get_type(),
    }
}

impl HeaderBar {
    #[cfg(gtk_3_10)]
    pub fn new() -> HeaderBar {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_header_bar_new()).downcast_unchecked()
        }
    }

    #[cfg(gtk_3_10)]
    pub fn get_custom_title(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_header_bar_get_custom_title(self.to_glib_none().0))
        }
    }

    #[cfg(gtk_3_12)]
    pub fn get_decoration_layout(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_header_bar_get_decoration_layout(self.to_glib_none().0))
        }
    }

    #[cfg(gtk_3_12)]
    pub fn get_has_subtitle(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_header_bar_get_has_subtitle(self.to_glib_none().0))
        }
    }

    #[cfg(gtk_3_10)]
    pub fn get_show_close_button(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_header_bar_get_show_close_button(self.to_glib_none().0))
        }
    }

    #[cfg(gtk_3_10)]
    pub fn get_subtitle(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_header_bar_get_subtitle(self.to_glib_none().0))
        }
    }

    #[cfg(gtk_3_10)]
    pub fn get_title(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_header_bar_get_title(self.to_glib_none().0))
        }
    }

    #[cfg(gtk_3_10)]
    pub fn pack_end<T: IsA<Widget>>(&self, child: &T) {
        unsafe {
            ffi::gtk_header_bar_pack_end(self.to_glib_none().0, child.to_glib_none().0);
        }
    }

    #[cfg(gtk_3_10)]
    pub fn pack_start<T: IsA<Widget>>(&self, child: &T) {
        unsafe {
            ffi::gtk_header_bar_pack_start(self.to_glib_none().0, child.to_glib_none().0);
        }
    }

    #[cfg(gtk_3_10)]
    pub fn set_custom_title<T: IsA<Widget>>(&self, title_widget: Option<&T>) {
        unsafe {
            ffi::gtk_header_bar_set_custom_title(self.to_glib_none().0, title_widget.to_glib_none().0);
        }
    }

    #[cfg(gtk_3_12)]
    pub fn set_decoration_layout(&self, layout: Option<&str>) {
        unsafe {
            ffi::gtk_header_bar_set_decoration_layout(self.to_glib_none().0, layout.to_glib_none().0);
        }
    }

    #[cfg(gtk_3_12)]
    pub fn set_has_subtitle(&self, setting: bool) {
        unsafe {
            ffi::gtk_header_bar_set_has_subtitle(self.to_glib_none().0, setting.to_glib());
        }
    }

    #[cfg(gtk_3_10)]
    pub fn set_show_close_button(&self, setting: bool) {
        unsafe {
            ffi::gtk_header_bar_set_show_close_button(self.to_glib_none().0, setting.to_glib());
        }
    }

    #[cfg(gtk_3_10)]
    pub fn set_subtitle(&self, subtitle: Option<&str>) {
        unsafe {
            ffi::gtk_header_bar_set_subtitle(self.to_glib_none().0, subtitle.to_glib_none().0);
        }
    }

    #[cfg(gtk_3_10)]
    pub fn set_title(&self, title: Option<&str>) {
        unsafe {
            ffi::gtk_header_bar_set_title(self.to_glib_none().0, title.to_glib_none().0);
        }
    }
}
