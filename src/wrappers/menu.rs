use super::MenuItem;
use std::ffi::CString;
use {ffi, ImplOpaque as Opaque, WidgetType};
// use std::os::raw;

#[derive(Copy, Clone)]
pub(crate) struct Menu {
    pub op: Opaque,
}

impl Menu {
    pub fn from(o: Opaque) -> Option<Menu> {
        if o.0 == ::WidgetType::Menu {
            return Some(Menu { op: o });
        }
        None
    }

    pub fn new(name: &str) -> Menu {
        let s = CString::new(name).unwrap();
        let p = unsafe { ffi::uiNewMenu(s.as_ptr()) };
        Menu {
            op: Opaque(WidgetType::Menu, p as _),
        }
    }

    pub fn append(&self, name: &str) -> MenuItem {
        let s = CString::new(name).unwrap();
        unsafe {
            let p = ffi::uiMenuAppendItem(self.op.1 as _, s.as_ptr());
            MenuItem {
                op: Opaque(WidgetType::MenuItem, p as _),
            }
        }
    }

    pub fn append_check(&self, name: &str) -> MenuItem {
        let s = CString::new(name).unwrap();
        unsafe {
            let p = ffi::uiMenuAppendCheckItem(self.op.1 as _, s.as_ptr());
            MenuItem {
                op: Opaque(WidgetType::MenuItem, p as _),
            }
        }
    }

    pub fn append_quit(&self) -> MenuItem {
        unsafe {
            let p = ffi::uiMenuAppendQuitItem(self.op.1 as _);
            MenuItem {
                op: Opaque(WidgetType::MenuItem, p as _),
            }
        }
    }

    pub fn append_preferences(&self) -> MenuItem {
        unsafe {
            let p = ffi::uiMenuAppendPreferencesItem(self.op.1 as _);
            MenuItem {
                op: Opaque(WidgetType::MenuItem, p as _),
            }
        }
    }

    pub fn append_about(&self) -> MenuItem {
        unsafe {
            let p = ffi::uiMenuAppendAboutItem(self.op.1 as _);
            MenuItem {
                op: Opaque(WidgetType::MenuItem, p as _),
            }
        }
    }

    pub fn append_separator(&self) {
        unsafe {
            ffi::uiMenuAppendSeparator(self.op.1 as _);
        }
    }
}

impl AsRef<Opaque> for Menu {
    fn as_ref(&self) -> &Opaque {
        &self.op
    }
}

// impl Widget for Menu {
//     fn opaque(&self) -> Opaque {
//         Opaque(WidgetType::Menu, self.p as *mut ::std::os::raw::c_void)
//     }
// }
