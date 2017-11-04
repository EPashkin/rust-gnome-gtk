// This file was generated by gir (94e079d) from gir-files (469db10)
// DO NOT EDIT

use IconSet;
use ffi;
use glib::object::IsA;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct IconFactory(Object<ffi::GtkIconFactory>);

    match fn {
        get_type => || ffi::gtk_icon_factory_get_type(),
    }
}

impl IconFactory {
    pub fn new() -> IconFactory {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gtk_icon_factory_new())
        }
    }

    pub fn lookup_default(stock_id: &str) -> Option<IconSet> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ffi::gtk_icon_factory_lookup_default(stock_id.to_glib_none().0))
        }
    }
}

impl Default for IconFactory {
    fn default() -> Self {
        Self::new()
    }
}

pub trait IconFactoryExt {
    fn add(&self, stock_id: &str, icon_set: &IconSet);

    fn add_default(&self);

    fn lookup(&self, stock_id: &str) -> Option<IconSet>;

    fn remove_default(&self);
}

impl<O: IsA<IconFactory>> IconFactoryExt for O {
    fn add(&self, stock_id: &str, icon_set: &IconSet) {
        unsafe {
            ffi::gtk_icon_factory_add(self.to_glib_none().0, stock_id.to_glib_none().0, icon_set.to_glib_none().0);
        }
    }

    fn add_default(&self) {
        unsafe {
            ffi::gtk_icon_factory_add_default(self.to_glib_none().0);
        }
    }

    fn lookup(&self, stock_id: &str) -> Option<IconSet> {
        unsafe {
            from_glib_none(ffi::gtk_icon_factory_lookup(self.to_glib_none().0, stock_id.to_glib_none().0))
        }
    }

    fn remove_default(&self) {
        unsafe {
            ffi::gtk_icon_factory_remove_default(self.to_glib_none().0);
        }
    }
}
