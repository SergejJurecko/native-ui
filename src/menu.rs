use {ffi, Opaque, WidgetType, MenuItem};
use std::ffi::CString;
// use std::os::raw;

#[derive(Copy, Clone)]
pub struct Menu {
    p: *mut ffi::uiMenu,
    opaque: Opaque,
}

impl Menu {
    pub fn from(o: Opaque) -> Option<Menu> {
        if o.0 == ::WidgetType::Menu {
            return Some(Menu {
                p: o.1 as _,
                opaque: o,
            });
        }
        None
    }

    pub fn new(name: &str) -> Menu {
        let s = CString::new(name).unwrap();
        let p = unsafe { ffi::uiNewMenu(s.as_ptr()) };
        Menu {
            p,
            opaque: Opaque(WidgetType::Menu, p as _),
        }
    }

    pub fn append(&self, name: &str) -> MenuItem {
        let s = CString::new(name).unwrap();
        unsafe {
            let p = ffi::uiMenuAppendItem(self.p, s.as_ptr());
            MenuItem {
                p,
                opaque: Opaque(WidgetType::MenuItem, p as _),
            }
        }
    }

    pub fn append_check(&self, name: &str) -> MenuItem {
        let s = CString::new(name).unwrap();
        unsafe {
            let p = ffi::uiMenuAppendCheckItem(self.p, s.as_ptr());
            MenuItem {
                p,
                opaque: Opaque(WidgetType::MenuItem, p as _),
            }
        }
    }

    pub fn append_quit(&self) -> MenuItem {
        unsafe {
            let p = ffi::uiMenuAppendQuitItem(self.p);
            MenuItem {
                p,
                opaque: Opaque(WidgetType::MenuItem, p as _),
            }
        }
    }

    pub fn append_preferences(&self) -> MenuItem {
        unsafe {
            let p = ffi::uiMenuAppendPreferencesItem(self.p);
            MenuItem {
                p,
                opaque: Opaque(WidgetType::MenuItem, p as _),
            }
        }
    }

    pub fn append_about(&self) -> MenuItem {
        unsafe {
            let p = ffi::uiMenuAppendAboutItem(self.p);
            MenuItem {
                p,
                opaque: Opaque(WidgetType::MenuItem, p as _),
            }
        }
    }

    pub fn append_separator(&self) {
        unsafe {
            ffi::uiMenuAppendSeparator(self.p);
        }
    }
}

impl AsRef<Opaque> for Menu {
    fn as_ref(&self) -> &Opaque {
        &self.opaque
    }
}

// impl Widget for Menu {
//     fn opaque(&self) -> Opaque {
//         Opaque(WidgetType::Menu, self.p as *mut ::std::os::raw::c_void)
//     }
// }
