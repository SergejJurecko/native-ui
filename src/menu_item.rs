use {ffi, Controller, EvId, Opaque, RegId, WidgetType};
// use std::ffi::{CStr, CString};
use std::os::raw;

#[derive(Copy, Clone)]
pub struct MenuItem {
    pub(crate) p: *mut ffi::uiMenuItem,
    pub(crate) opaque: Opaque,
}

impl MenuItem {
    pub fn from(o: Opaque) -> Option<MenuItem> {
        if o.0 == ::WidgetType::MenuItem {
            return Some(MenuItem {
                p: o.1 as *mut ffi::uiMenuItem,
                opaque: o,
            });
        }
        None
    }

    pub fn set_checked(&self, v: bool) {
        unsafe {
            ffi::uiMenuItemSetChecked(self.p, v as i32);
        }
    }

    pub fn enable(&self) {
        unsafe {
            ffi::uiMenuItemEnable(self.p);
        }
    }

    pub fn disable(&self) {
        unsafe {
            ffi::uiMenuItemDisable(self.p);
        }
    }

    pub fn checked(&self) -> bool {
        unsafe {
            if ffi::uiMenuItemChecked(self.p) == 0 {
                false
            } else {
                true
            }
        }
    }

    pub fn reg_on_clicked<T>(&self, ctrler: &Controller<T>, evid: &EvId) {
        let id = ::std::boxed::Box::new(RegId::new(
            WidgetType::MenuItem,
            ctrler.id().0,
            evid.0,
        ));
        unsafe {
            ffi::uiMenuItemOnClicked(
                self.p,
                Some(::ui::on_menu_event::<ffi::uiMenuItem>),
                Box::into_raw(id) as *mut raw::c_void,
            );
        }
    }
}

impl AsRef<Opaque> for MenuItem {
    fn as_ref(&self) -> &Opaque {
        &self.opaque
    }
}
