// This file was generated by gir (b7f5189) from gir-files (71d73f0)
// DO NOT EDIT

use Object;
use TreeModel;
use ffi;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use std::boxed::Box as Box_;
use std::mem::transmute;

glib_wrapper! {
    pub struct TreeSortable(Object<ffi::GtkTreeSortable>): TreeModel;

    match fn {
        get_type => || ffi::gtk_tree_sortable_get_type(),
    }
}

pub trait TreeSortableExt {
    fn has_default_sort_func(&self) -> bool;

    //fn set_default_sort_func(&self, sort_func: /*Unknown conversion*//*Unimplemented*/TreeIterCompareFunc, user_data: /*Unimplemented*/Option<Fundamental: Pointer>, destroy: /*Unknown conversion*//*Unimplemented*/DestroyNotify);

    //fn set_sort_func(&self, sort_column_id: i32, sort_func: /*Unknown conversion*//*Unimplemented*/TreeIterCompareFunc, user_data: /*Unimplemented*/Option<Fundamental: Pointer>, destroy: /*Unknown conversion*//*Unimplemented*/DestroyNotify);

    fn sort_column_changed(&self);

    fn connect_sort_column_changed<F: Fn(&Self) + 'static>(&self, f: F) -> u64;
}

impl<O: IsA<TreeSortable> + IsA<Object>> TreeSortableExt for O {
    fn has_default_sort_func(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_tree_sortable_has_default_sort_func(self.to_glib_none().0))
        }
    }

    //fn set_default_sort_func(&self, sort_func: /*Unknown conversion*//*Unimplemented*/TreeIterCompareFunc, user_data: /*Unimplemented*/Option<Fundamental: Pointer>, destroy: /*Unknown conversion*//*Unimplemented*/DestroyNotify) {
    //    unsafe { TODO: call ffi::gtk_tree_sortable_set_default_sort_func() }
    //}

    //fn set_sort_func(&self, sort_column_id: i32, sort_func: /*Unknown conversion*//*Unimplemented*/TreeIterCompareFunc, user_data: /*Unimplemented*/Option<Fundamental: Pointer>, destroy: /*Unknown conversion*//*Unimplemented*/DestroyNotify) {
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

unsafe extern "C" fn sort_column_changed_trampoline<T>(this: *mut ffi::GtkTreeSortable, f: glib_ffi::gpointer)
where T: IsA<TreeSortable> {
    callback_guard!();
    let f: &Box_<Fn(&T) + 'static> = transmute(f);
    f(&TreeSortable::from_glib_none(this).downcast_unchecked())
}
