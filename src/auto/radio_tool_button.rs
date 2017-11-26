// This file was generated by gir (d933f9a) from gir-files (469db10)
// DO NOT EDIT

use Actionable;
use Bin;
use Buildable;
use Container;
use RadioButton;
use ToggleToolButton;
use ToolButton;
use ToolItem;
use Widget;
use ffi;
use glib::object::Downcast;
use glib::object::IsA;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct RadioToolButton(Object<ffi::GtkRadioToolButton, ffi::GtkRadioToolButtonClass>): ToggleToolButton, ToolButton, ToolItem, Bin, Container, Widget, Buildable, Actionable;

    match fn {
        get_type => || ffi::gtk_radio_tool_button_get_type(),
    }
}

impl RadioToolButton {
    pub fn new_from_widget(group: &RadioToolButton) -> RadioToolButton {
        skip_assert_initialized!();
        unsafe {
            ToolItem::from_glib_none(ffi::gtk_radio_tool_button_new_from_widget(group.to_glib_none().0)).downcast_unchecked()
        }
    }

    pub fn new_with_stock_from_widget(group: &RadioToolButton, stock_id: &str) -> RadioToolButton {
        skip_assert_initialized!();
        unsafe {
            ToolItem::from_glib_none(ffi::gtk_radio_tool_button_new_with_stock_from_widget(group.to_glib_none().0, stock_id.to_glib_none().0)).downcast_unchecked()
        }
    }
}

pub trait RadioToolButtonExt {
    fn get_group(&self) -> Vec<RadioButton>;
}

impl<O: IsA<RadioToolButton>> RadioToolButtonExt for O {
    fn get_group(&self) -> Vec<RadioButton> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::gtk_radio_tool_button_get_group(self.to_glib_none().0))
        }
    }
}
