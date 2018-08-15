use {ffi, EvId, ImplOpaque as Opaque, RegId, WidgetType};
// use std::ffi::{CStr, CString};

#[derive(Copy, Clone)]
pub(crate) struct MenuItem {
    pub op: Opaque,
}

impl MenuItem {
    // pub fn from(o: Opaque) -> Option<MenuItem> {
    //     if o.0 == ::WidgetType::MenuItem {
    //         return Some(MenuItem { op: o });
    //     }
    //     None
    // }

    pub fn set_checked(&self, v: bool) {
        unsafe {
            ffi::uiMenuItemSetChecked(self.op.1 as _, v as i32);
        }
    }

    pub fn enable(&self) {
        unsafe {
            ffi::uiMenuItemEnable(self.op.1 as _);
        }
    }

    pub fn disable(&self) {
        unsafe {
            ffi::uiMenuItemDisable(self.op.1 as _);
        }
    }

    pub fn checked(&self) -> bool {
        unsafe {
            if ffi::uiMenuItemChecked(self.op.1 as _) == 0 {
                false
            } else {
                true
            }
        }
    }

    pub fn reg_on_clicked<T>(&self, p: *mut ::RegId) {
        // let id = ::std::boxed::Box::new(RegId::new(
        //     WidgetType::MenuItem,
        //     ctrler.id().0,
        //     evid.0,
        // ));
        // unsafe {
        //     ffi::uiMenuItemOnClicked(
        //         self.p,
        //         Some(::ui::on_menu_event::<ffi::uiMenuItem>),
        //         Box::into_raw(id) as *mut raw::c_void,
        //     );
        // }
    }
}

impl AsRef<Opaque> for MenuItem {
    fn as_ref(&self) -> &Opaque {
        &self.op
    }
}
