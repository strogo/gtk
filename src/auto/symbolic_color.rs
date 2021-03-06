// This file was generated by gir (b7f5189) from gir-files (71d73f0)
// DO NOT EDIT

use StyleProperties;
use ffi;
use gdk;
use glib::translate::*;
use std::fmt;

glib_wrapper! {
    pub struct SymbolicColor(Shared<ffi::GtkSymbolicColor>);

    match fn {
        ref => |ptr| ffi::gtk_symbolic_color_ref(ptr),
        unref => |ptr| ffi::gtk_symbolic_color_unref(ptr),
    }
}

impl SymbolicColor {
    pub fn new_alpha(color: &SymbolicColor, factor: f64) -> SymbolicColor {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gtk_symbolic_color_new_alpha(color.to_glib_none().0, factor))
        }
    }

    pub fn new_literal(color: &gdk::RGBA) -> SymbolicColor {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gtk_symbolic_color_new_literal(color.to_glib_none().0))
        }
    }

    pub fn new_mix(color1: &SymbolicColor, color2: &SymbolicColor, factor: f64) -> SymbolicColor {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gtk_symbolic_color_new_mix(color1.to_glib_none().0, color2.to_glib_none().0, factor))
        }
    }

    pub fn new_name(name: &str) -> SymbolicColor {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gtk_symbolic_color_new_name(name.to_glib_none().0))
        }
    }

    pub fn new_shade(color: &SymbolicColor, factor: f64) -> SymbolicColor {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gtk_symbolic_color_new_shade(color.to_glib_none().0, factor))
        }
    }

    pub fn new_win32(theme_class: &str, id: i32) -> SymbolicColor {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gtk_symbolic_color_new_win32(theme_class.to_glib_none().0, id))
        }
    }

    pub fn resolve(&self, props: Option<&StyleProperties>) -> Option<gdk::RGBA> {
        unsafe {
            let mut resolved_color = gdk::RGBA::uninitialized();
            let ret = from_glib(ffi::gtk_symbolic_color_resolve(self.to_glib_none().0, props.to_glib_none().0, resolved_color.to_glib_none_mut().0));
            if ret { Some(resolved_color) } else { None }
        }
    }

    fn to_string(&self) -> String {
        unsafe {
            from_glib_full(ffi::gtk_symbolic_color_to_string(self.to_glib_none().0))
        }
    }
}

impl fmt::Display for SymbolicColor {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
