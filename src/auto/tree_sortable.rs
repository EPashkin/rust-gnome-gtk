// This file was generated by gir (127a851) from gir-files (0bcaef9)
// DO NOT EDIT

use TreeModel;
use ffi;
use glib;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct TreeSortable(Object<ffi::GtkTreeSortable>): TreeModel;

    match fn {
        get_type => || ffi::gtk_tree_sortable_get_type(),
    }
}

pub trait TreeSortableExt {
    fn has_default_sort_func(&self) -> bool;

    //fn set_default_sort_func<'a, P: Into<Option</*Unimplemented*/Fundamental: Pointer>>, Q: Into<Option<&'a /*Ignored*/glib::DestroyNotify>>>(&self, sort_func: /*Unknown conversion*//*Unimplemented*/TreeIterCompareFunc, user_data: P, destroy: Q);

    //fn set_sort_func<'a, P: Into<Option</*Unimplemented*/Fundamental: Pointer>>, Q: Into<Option<&'a /*Ignored*/glib::DestroyNotify>>>(&self, sort_column_id: i32, sort_func: /*Unknown conversion*//*Unimplemented*/TreeIterCompareFunc, user_data: P, destroy: Q);

    fn sort_column_changed(&self);

    fn connect_sort_column_changed<F: Fn(&Self) + 'static>(&self, f: F) -> u64;
}

impl<O: IsA<TreeSortable> + IsA<glib::object::Object>> TreeSortableExt for O {
    fn has_default_sort_func(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_tree_sortable_has_default_sort_func(self.to_glib_none().0))
        }
    }

    //fn set_default_sort_func<'a, P: Into<Option</*Unimplemented*/Fundamental: Pointer>>, Q: Into<Option<&'a /*Ignored*/glib::DestroyNotify>>>(&self, sort_func: /*Unknown conversion*//*Unimplemented*/TreeIterCompareFunc, user_data: P, destroy: Q) {
    //    unsafe { TODO: call ffi::gtk_tree_sortable_set_default_sort_func() }
    //}

    //fn set_sort_func<'a, P: Into<Option</*Unimplemented*/Fundamental: Pointer>>, Q: Into<Option<&'a /*Ignored*/glib::DestroyNotify>>>(&self, sort_column_id: i32, sort_func: /*Unknown conversion*//*Unimplemented*/TreeIterCompareFunc, user_data: P, destroy: Q) {
    //    unsafe { TODO: call ffi::gtk_tree_sortable_set_sort_func() }
    //}

    fn sort_column_changed(&self) {
        unsafe {
            ffi::gtk_tree_sortable_sort_column_changed(self.to_glib_none().0);
        }
    }

    fn connect_sort_column_changed<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "sort-column-changed",
                transmute(sort_column_changed_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn sort_column_changed_trampoline<P>(this: *mut ffi::GtkTreeSortable, f: glib_ffi::gpointer)
where P: IsA<TreeSortable> {
    callback_guard!();
    let f: &Box_<Fn(&P) + 'static> = transmute(f);
    f(&TreeSortable::from_glib_none(this).downcast_unchecked())
}
