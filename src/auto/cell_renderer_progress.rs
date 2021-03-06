// This file was generated by gir (b7f5189) from gir-files (71d73f0)
// DO NOT EDIT

use CellRenderer;
use Orientable;
use ffi;
use glib::Value;
use glib::object::Downcast;
use glib::translate::*;
use gobject_ffi;

glib_wrapper! {
    pub struct CellRendererProgress(Object<ffi::GtkCellRendererProgress>): CellRenderer, Orientable;

    match fn {
        get_type => || ffi::gtk_cell_renderer_progress_get_type(),
    }
}

impl CellRendererProgress {
    pub fn new() -> CellRendererProgress {
        assert_initialized_main_thread!();
        unsafe {
            CellRenderer::from_glib_none(ffi::gtk_cell_renderer_progress_new()).downcast_unchecked()
        }
    }

    pub fn get_property_inverted(&self) -> bool {
        let mut value = Value::from(&false);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "inverted".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    pub fn set_property_inverted(&self, inverted: bool) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "inverted".to_glib_none().0, Value::from(&inverted).to_glib_none().0);
        }
    }

    pub fn get_property_pulse(&self) -> i32 {
        let mut value = Value::from(&0);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "pulse".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    pub fn set_property_pulse(&self, pulse: i32) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "pulse".to_glib_none().0, Value::from(&pulse).to_glib_none().0);
        }
    }

    pub fn get_property_text(&self) -> Option<String> {
        let mut value = Value::from(None::<&str>);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "text".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get()
    }

    pub fn set_property_text(&self, text: Option<&str>) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "text".to_glib_none().0, Value::from(text).to_glib_none().0);
        }
    }

    pub fn get_property_text_xalign(&self) -> f32 {
        let mut value = Value::from(&0f32);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "text-xalign".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    pub fn set_property_text_xalign(&self, text_xalign: f32) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "text-xalign".to_glib_none().0, Value::from(&text_xalign).to_glib_none().0);
        }
    }

    pub fn get_property_text_yalign(&self) -> f32 {
        let mut value = Value::from(&0f32);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "text-yalign".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    pub fn set_property_text_yalign(&self, text_yalign: f32) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "text-yalign".to_glib_none().0, Value::from(&text_yalign).to_glib_none().0);
        }
    }

    pub fn get_property_value(&self) -> i32 {
        let mut value = Value::from(&0);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "value".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    pub fn set_property_value(&self, value: i32) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "value".to_glib_none().0, Value::from(&value).to_glib_none().0);
        }
    }
}
