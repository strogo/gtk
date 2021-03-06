// This file was generated by gir (bf7bd49) from gir-files (71d73f0)
// DO NOT EDIT

use ImageType;
use Menu;
use Orientation;
use Tooltip;
use ffi;
use gdk;
use gdk_ffi;
use gdk_pixbuf;
use glib::Value;
use glib::object::IsA;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use libc;
use std;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct StatusIcon(Object<ffi::GtkStatusIcon>);

    match fn {
        get_type => || ffi::gtk_status_icon_get_type(),
    }
}

impl StatusIcon {
    pub fn new() -> StatusIcon {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gtk_status_icon_new())
        }
    }

    pub fn new_from_file<T: AsRef<std::path::Path>>(filename: T) -> StatusIcon {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gtk_status_icon_new_from_file(filename.as_ref().to_glib_none().0))
        }
    }

    //pub fn new_from_gicon<T: IsA</*Ignored*/gio::Icon>>(icon: &T) -> StatusIcon {
    //    unsafe { TODO: call ffi::gtk_status_icon_new_from_gicon() }
    //}

    pub fn new_from_icon_name(icon_name: &str) -> StatusIcon {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gtk_status_icon_new_from_icon_name(icon_name.to_glib_none().0))
        }
    }

    pub fn new_from_pixbuf(pixbuf: &gdk_pixbuf::Pixbuf) -> StatusIcon {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gtk_status_icon_new_from_pixbuf(pixbuf.to_glib_none().0))
        }
    }

    pub fn new_from_stock(stock_id: &str) -> StatusIcon {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gtk_status_icon_new_from_stock(stock_id.to_glib_none().0))
        }
    }

    pub fn get_geometry(&self) -> Option<(gdk::Screen, gdk::Rectangle, Orientation)> {
        unsafe {
            let mut screen = ptr::null_mut();
            let mut area = gdk::Rectangle::uninitialized();
            let mut orientation = mem::uninitialized();
            let ret = from_glib(ffi::gtk_status_icon_get_geometry(self.to_glib_none().0, &mut screen, area.to_glib_none_mut().0, &mut orientation));
            if ret { Some((from_glib_none(screen), area, from_glib(orientation))) } else { None }
        }
    }

    //pub fn get_gicon(&self) -> /*Ignored*/Option<gio::Icon> {
    //    unsafe { TODO: call ffi::gtk_status_icon_get_gicon() }
    //}

    pub fn get_has_tooltip(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_status_icon_get_has_tooltip(self.to_glib_none().0))
        }
    }

    pub fn get_icon_name(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_status_icon_get_icon_name(self.to_glib_none().0))
        }
    }

    pub fn get_pixbuf(&self) -> Option<gdk_pixbuf::Pixbuf> {
        unsafe {
            from_glib_none(ffi::gtk_status_icon_get_pixbuf(self.to_glib_none().0))
        }
    }

    pub fn get_screen(&self) -> Option<gdk::Screen> {
        unsafe {
            from_glib_none(ffi::gtk_status_icon_get_screen(self.to_glib_none().0))
        }
    }

    pub fn get_size(&self) -> i32 {
        unsafe {
            ffi::gtk_status_icon_get_size(self.to_glib_none().0)
        }
    }

    pub fn get_stock(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_status_icon_get_stock(self.to_glib_none().0))
        }
    }

    pub fn get_storage_type(&self) -> ImageType {
        unsafe {
            from_glib(ffi::gtk_status_icon_get_storage_type(self.to_glib_none().0))
        }
    }

    pub fn get_title(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_status_icon_get_title(self.to_glib_none().0))
        }
    }

    pub fn get_tooltip_markup(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::gtk_status_icon_get_tooltip_markup(self.to_glib_none().0))
        }
    }

    pub fn get_tooltip_text(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::gtk_status_icon_get_tooltip_text(self.to_glib_none().0))
        }
    }

    pub fn get_visible(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_status_icon_get_visible(self.to_glib_none().0))
        }
    }

    pub fn get_x11_window_id(&self) -> u32 {
        unsafe {
            ffi::gtk_status_icon_get_x11_window_id(self.to_glib_none().0)
        }
    }

    pub fn is_embedded(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_status_icon_is_embedded(self.to_glib_none().0))
        }
    }

    pub fn set_from_file<T: AsRef<std::path::Path>>(&self, filename: T) {
        unsafe {
            ffi::gtk_status_icon_set_from_file(self.to_glib_none().0, filename.as_ref().to_glib_none().0);
        }
    }

    //pub fn set_from_gicon<T: IsA</*Ignored*/gio::Icon>>(&self, icon: &T) {
    //    unsafe { TODO: call ffi::gtk_status_icon_set_from_gicon() }
    //}

    pub fn set_from_icon_name(&self, icon_name: &str) {
        unsafe {
            ffi::gtk_status_icon_set_from_icon_name(self.to_glib_none().0, icon_name.to_glib_none().0);
        }
    }

    pub fn set_from_pixbuf(&self, pixbuf: Option<&gdk_pixbuf::Pixbuf>) {
        unsafe {
            ffi::gtk_status_icon_set_from_pixbuf(self.to_glib_none().0, pixbuf.to_glib_none().0);
        }
    }

    pub fn set_from_stock(&self, stock_id: &str) {
        unsafe {
            ffi::gtk_status_icon_set_from_stock(self.to_glib_none().0, stock_id.to_glib_none().0);
        }
    }

    pub fn set_has_tooltip(&self, has_tooltip: bool) {
        unsafe {
            ffi::gtk_status_icon_set_has_tooltip(self.to_glib_none().0, has_tooltip.to_glib());
        }
    }

    pub fn set_name(&self, name: &str) {
        unsafe {
            ffi::gtk_status_icon_set_name(self.to_glib_none().0, name.to_glib_none().0);
        }
    }

    pub fn set_screen(&self, screen: &gdk::Screen) {
        unsafe {
            ffi::gtk_status_icon_set_screen(self.to_glib_none().0, screen.to_glib_none().0);
        }
    }

    pub fn set_title(&self, title: &str) {
        unsafe {
            ffi::gtk_status_icon_set_title(self.to_glib_none().0, title.to_glib_none().0);
        }
    }

    pub fn set_tooltip_markup<'a, T: Into<Option<&'a str>>>(&self, markup: T) {
        unsafe {
            ffi::gtk_status_icon_set_tooltip_markup(self.to_glib_none().0, markup.into().to_glib_none().0);
        }
    }

    pub fn set_tooltip_text(&self, text: &str) {
        unsafe {
            ffi::gtk_status_icon_set_tooltip_text(self.to_glib_none().0, text.to_glib_none().0);
        }
    }

    pub fn set_visible(&self, visible: bool) {
        unsafe {
            ffi::gtk_status_icon_set_visible(self.to_glib_none().0, visible.to_glib());
        }
    }

    pub fn get_property_embedded(&self) -> bool {
        let mut value = Value::from(&false);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "embedded".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    pub fn set_property_file(&self, file: Option<&str>) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "file".to_glib_none().0, Value::from(file).to_glib_none().0);
        }
    }

    //pub fn set_property_gicon<T: IsA</*Ignored*/gio::Icon> + IsA<Object>>(&self, gicon: Option<&T>) {
    //    unsafe {
    //        gobject_ffi::g_object_set_property(self.to_glib_none().0, "gicon".to_glib_none().0, Value::from(gicon).to_glib_none().0);
    //    }
    //}

    pub fn set_property_icon_name(&self, icon_name: Option<&str>) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "icon-name".to_glib_none().0, Value::from(icon_name).to_glib_none().0);
        }
    }

    pub fn get_property_orientation(&self) -> Orientation {
        let mut value = Value::from(&0);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "orientation".to_glib_none().0, value.to_glib_none_mut().0);
            from_glib(transmute(value.get::<i32>().unwrap()))
        }
    }

    pub fn set_property_pixbuf(&self, pixbuf: Option<&gdk_pixbuf::Pixbuf>) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "pixbuf".to_glib_none().0, Value::from(pixbuf).to_glib_none().0);
        }
    }

    pub fn set_property_stock(&self, stock: Option<&str>) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "stock".to_glib_none().0, Value::from(stock).to_glib_none().0);
        }
    }

    pub fn position_menu<T: IsA<Menu>>(menu: &T, x: &mut i32, y: &mut i32, user_data: &StatusIcon) -> bool {
        skip_assert_initialized!();
        unsafe {
            let mut push_in = mem::uninitialized();
            ffi::gtk_status_icon_position_menu(menu.to_glib_none().0, x, y, &mut push_in, user_data.to_glib_none().0);
            from_glib(push_in)
        }
    }

    pub fn connect_activate<F: Fn(&StatusIcon) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&StatusIcon) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "activate",
                transmute(activate_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_button_press_event<F: Fn(&StatusIcon, &gdk::EventButton) -> bool + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&StatusIcon, &gdk::EventButton) -> bool + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "button-press-event",
                transmute(button_press_event_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_button_release_event<F: Fn(&StatusIcon, &gdk::EventButton) -> bool + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&StatusIcon, &gdk::EventButton) -> bool + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "button-release-event",
                transmute(button_release_event_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_popup_menu<F: Fn(&StatusIcon, u32, u32) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&StatusIcon, u32, u32) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "popup-menu",
                transmute(popup_menu_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_query_tooltip<F: Fn(&StatusIcon, i32, i32, bool, &Tooltip) -> bool + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&StatusIcon, i32, i32, bool, &Tooltip) -> bool + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "query-tooltip",
                transmute(query_tooltip_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_scroll_event<F: Fn(&StatusIcon, &gdk::EventScroll) -> bool + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&StatusIcon, &gdk::EventScroll) -> bool + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "scroll-event",
                transmute(scroll_event_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_size_changed<F: Fn(&StatusIcon, i32) -> bool + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&StatusIcon, i32) -> bool + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "size-changed",
                transmute(size_changed_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn activate_trampoline(this: *mut ffi::GtkStatusIcon, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&StatusIcon) + 'static> = transmute(f);
    f(&from_glib_none(this))
}

unsafe extern "C" fn button_press_event_trampoline(this: *mut ffi::GtkStatusIcon, event: *mut gdk_ffi::GdkEventButton, f: glib_ffi::gpointer) -> glib_ffi::gboolean {
    callback_guard!();
    let f: &Box_<Fn(&StatusIcon, &gdk::EventButton) -> bool + 'static> = transmute(f);
    f(&from_glib_none(this), &from_glib_none(event)).to_glib()
}

unsafe extern "C" fn button_release_event_trampoline(this: *mut ffi::GtkStatusIcon, event: *mut gdk_ffi::GdkEventButton, f: glib_ffi::gpointer) -> glib_ffi::gboolean {
    callback_guard!();
    let f: &Box_<Fn(&StatusIcon, &gdk::EventButton) -> bool + 'static> = transmute(f);
    f(&from_glib_none(this), &from_glib_none(event)).to_glib()
}

unsafe extern "C" fn popup_menu_trampoline(this: *mut ffi::GtkStatusIcon, button: libc::c_uint, activate_time: libc::c_uint, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&StatusIcon, u32, u32) + 'static> = transmute(f);
    f(&from_glib_none(this), button, activate_time)
}

unsafe extern "C" fn query_tooltip_trampoline(this: *mut ffi::GtkStatusIcon, x: libc::c_int, y: libc::c_int, keyboard_mode: glib_ffi::gboolean, tooltip: *mut ffi::GtkTooltip, f: glib_ffi::gpointer) -> glib_ffi::gboolean {
    callback_guard!();
    let f: &Box_<Fn(&StatusIcon, i32, i32, bool, &Tooltip) -> bool + 'static> = transmute(f);
    f(&from_glib_none(this), x, y, from_glib(keyboard_mode), &from_glib_none(tooltip)).to_glib()
}

unsafe extern "C" fn scroll_event_trampoline(this: *mut ffi::GtkStatusIcon, event: *mut gdk_ffi::GdkEventScroll, f: glib_ffi::gpointer) -> glib_ffi::gboolean {
    callback_guard!();
    let f: &Box_<Fn(&StatusIcon, &gdk::EventScroll) -> bool + 'static> = transmute(f);
    f(&from_glib_none(this), &from_glib_none(event)).to_glib()
}

unsafe extern "C" fn size_changed_trampoline(this: *mut ffi::GtkStatusIcon, size: libc::c_int, f: glib_ffi::gpointer) -> glib_ffi::gboolean {
    callback_guard!();
    let f: &Box_<Fn(&StatusIcon, i32) -> bool + 'static> = transmute(f);
    f(&from_glib_none(this), size).to_glib()
}
