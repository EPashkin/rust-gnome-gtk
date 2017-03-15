// This file was generated by gir (b7f5189) from gir-files (71d73f0)
// DO NOT EDIT

use Container;
use Orientable;
use Orientation;
use ScrollType;
use Widget;
use ffi;
use gdk;
use glib::Value;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::boxed::Box as Box_;
use std::mem::transmute;

glib_wrapper! {
    pub struct Paned(Object<ffi::GtkPaned>): Container, Widget, Orientable;

    match fn {
        get_type => || ffi::gtk_paned_get_type(),
    }
}

impl Paned {
    pub fn new(orientation: Orientation) -> Paned {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_paned_new(orientation.to_glib())).downcast_unchecked()
        }
    }

    pub fn add1<T: IsA<Widget>>(&self, child: &T) {
        unsafe {
            ffi::gtk_paned_add1(self.to_glib_none().0, child.to_glib_none().0);
        }
    }

    pub fn add2<T: IsA<Widget>>(&self, child: &T) {
        unsafe {
            ffi::gtk_paned_add2(self.to_glib_none().0, child.to_glib_none().0);
        }
    }

    pub fn get_child1(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_paned_get_child1(self.to_glib_none().0))
        }
    }

    pub fn get_child2(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_paned_get_child2(self.to_glib_none().0))
        }
    }

    pub fn get_handle_window(&self) -> Option<gdk::Window> {
        unsafe {
            from_glib_none(ffi::gtk_paned_get_handle_window(self.to_glib_none().0))
        }
    }

    pub fn get_position(&self) -> i32 {
        unsafe {
            ffi::gtk_paned_get_position(self.to_glib_none().0)
        }
    }

    #[cfg(feature = "v3_16")]
    pub fn get_wide_handle(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_paned_get_wide_handle(self.to_glib_none().0))
        }
    }

    pub fn pack1<T: IsA<Widget>>(&self, child: &T, resize: bool, shrink: bool) {
        unsafe {
            ffi::gtk_paned_pack1(self.to_glib_none().0, child.to_glib_none().0, resize.to_glib(), shrink.to_glib());
        }
    }

    pub fn pack2<T: IsA<Widget>>(&self, child: &T, resize: bool, shrink: bool) {
        unsafe {
            ffi::gtk_paned_pack2(self.to_glib_none().0, child.to_glib_none().0, resize.to_glib(), shrink.to_glib());
        }
    }

    pub fn set_position(&self, position: i32) {
        unsafe {
            ffi::gtk_paned_set_position(self.to_glib_none().0, position);
        }
    }

    #[cfg(feature = "v3_16")]
    pub fn set_wide_handle(&self, wide: bool) {
        unsafe {
            ffi::gtk_paned_set_wide_handle(self.to_glib_none().0, wide.to_glib());
        }
    }

    pub fn get_property_max_position(&self) -> i32 {
        let mut value = Value::from(&0);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "max-position".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    pub fn get_property_min_position(&self) -> i32 {
        let mut value = Value::from(&0);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "min-position".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    pub fn get_property_position_set(&self) -> bool {
        let mut value = Value::from(&false);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "position-set".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    pub fn set_property_position_set(&self, position_set: bool) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "position-set".to_glib_none().0, Value::from(&position_set).to_glib_none().0);
        }
    }

    pub fn get_child_resize<T: IsA<Widget>>(&self, item: &T) -> bool {
        let mut value = Value::from(&false);
        unsafe {
            ffi::gtk_container_child_get_property(self.to_glib_none().0, item.to_glib_none().0, "resize".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    pub fn set_child_resize<T: IsA<Widget>>(&self, item: &T, resize: bool) {
        unsafe {
            ffi::gtk_container_child_set_property(self.to_glib_none().0, item.to_glib_none().0, "resize".to_glib_none().0, Value::from(&resize).to_glib_none().0);
        }
    }

    pub fn get_child_shrink<T: IsA<Widget>>(&self, item: &T) -> bool {
        let mut value = Value::from(&false);
        unsafe {
            ffi::gtk_container_child_get_property(self.to_glib_none().0, item.to_glib_none().0, "shrink".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    pub fn set_child_shrink<T: IsA<Widget>>(&self, item: &T, shrink: bool) {
        unsafe {
            ffi::gtk_container_child_set_property(self.to_glib_none().0, item.to_glib_none().0, "shrink".to_glib_none().0, Value::from(&shrink).to_glib_none().0);
        }
    }

    pub fn connect_accept_position<F: Fn(&Paned) -> bool + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Paned) -> bool + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "accept-position",
                transmute(accept_position_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_cancel_position<F: Fn(&Paned) -> bool + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Paned) -> bool + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "cancel-position",
                transmute(cancel_position_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_cycle_child_focus<F: Fn(&Paned, bool) -> bool + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Paned, bool) -> bool + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "cycle-child-focus",
                transmute(cycle_child_focus_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_cycle_handle_focus<F: Fn(&Paned, bool) -> bool + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Paned, bool) -> bool + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "cycle-handle-focus",
                transmute(cycle_handle_focus_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_move_handle<F: Fn(&Paned, ScrollType) -> bool + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Paned, ScrollType) -> bool + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "move-handle",
                transmute(move_handle_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_toggle_handle_focus<F: Fn(&Paned) -> bool + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Paned) -> bool + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "toggle-handle-focus",
                transmute(toggle_handle_focus_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn accept_position_trampoline(this: *mut ffi::GtkPaned, f: glib_ffi::gpointer) -> glib_ffi::gboolean {
    callback_guard!();
    let f: &Box_<Fn(&Paned) -> bool + 'static> = transmute(f);
    f(&from_glib_none(this)).to_glib()
}

unsafe extern "C" fn cancel_position_trampoline(this: *mut ffi::GtkPaned, f: glib_ffi::gpointer) -> glib_ffi::gboolean {
    callback_guard!();
    let f: &Box_<Fn(&Paned) -> bool + 'static> = transmute(f);
    f(&from_glib_none(this)).to_glib()
}

unsafe extern "C" fn cycle_child_focus_trampoline(this: *mut ffi::GtkPaned, reversed: glib_ffi::gboolean, f: glib_ffi::gpointer) -> glib_ffi::gboolean {
    callback_guard!();
    let f: &Box_<Fn(&Paned, bool) -> bool + 'static> = transmute(f);
    f(&from_glib_none(this), from_glib(reversed)).to_glib()
}

unsafe extern "C" fn cycle_handle_focus_trampoline(this: *mut ffi::GtkPaned, reversed: glib_ffi::gboolean, f: glib_ffi::gpointer) -> glib_ffi::gboolean {
    callback_guard!();
    let f: &Box_<Fn(&Paned, bool) -> bool + 'static> = transmute(f);
    f(&from_glib_none(this), from_glib(reversed)).to_glib()
}

unsafe extern "C" fn move_handle_trampoline(this: *mut ffi::GtkPaned, scroll_type: ffi::GtkScrollType, f: glib_ffi::gpointer) -> glib_ffi::gboolean {
    callback_guard!();
    let f: &Box_<Fn(&Paned, ScrollType) -> bool + 'static> = transmute(f);
    f(&from_glib_none(this), from_glib(scroll_type)).to_glib()
}

unsafe extern "C" fn toggle_handle_focus_trampoline(this: *mut ffi::GtkPaned, f: glib_ffi::gpointer) -> glib_ffi::gboolean {
    callback_guard!();
    let f: &Box_<Fn(&Paned) -> bool + 'static> = transmute(f);
    f(&from_glib_none(this)).to_glib()
}
