// This file was generated by gir (b7f5189) from gir-files (71d73f0)
// DO NOT EDIT

use Bin;
use Container;
use Frame;
use Widget;
use ffi;
use glib::Value;
use glib::object::Downcast;
use glib::translate::*;
use gobject_ffi;

glib_wrapper! {
    pub struct AspectFrame(Object<ffi::GtkAspectFrame>): Frame, Bin, Container, Widget;

    match fn {
        get_type => || ffi::gtk_aspect_frame_get_type(),
    }
}

impl AspectFrame {
    pub fn new<'a, T: Into<Option<&'a str>>>(label: T, xalign: f32, yalign: f32, ratio: f32, obey_child: bool) -> AspectFrame {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_aspect_frame_new(label.into().to_glib_none().0, xalign, yalign, ratio, obey_child.to_glib())).downcast_unchecked()
        }
    }

    pub fn set(&self, xalign: f32, yalign: f32, ratio: f32, obey_child: bool) {
        unsafe {
            ffi::gtk_aspect_frame_set(self.to_glib_none().0, xalign, yalign, ratio, obey_child.to_glib());
        }
    }

    pub fn get_property_obey_child(&self) -> bool {
        let mut value = Value::from(&false);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "obey-child".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    pub fn set_property_obey_child(&self, obey_child: bool) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "obey-child".to_glib_none().0, Value::from(&obey_child).to_glib_none().0);
        }
    }

    pub fn get_property_ratio(&self) -> f32 {
        let mut value = Value::from(&0f32);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "ratio".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    pub fn set_property_ratio(&self, ratio: f32) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "ratio".to_glib_none().0, Value::from(&ratio).to_glib_none().0);
        }
    }

    pub fn get_property_xalign(&self) -> f32 {
        let mut value = Value::from(&0f32);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "xalign".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    pub fn set_property_xalign(&self, xalign: f32) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "xalign".to_glib_none().0, Value::from(&xalign).to_glib_none().0);
        }
    }

    pub fn get_property_yalign(&self) -> f32 {
        let mut value = Value::from(&0f32);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "yalign".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    pub fn set_property_yalign(&self, yalign: f32) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "yalign".to_glib_none().0, Value::from(&yalign).to_glib_none().0);
        }
    }
}
