use {ffi, Opaque, WidgetType};
use std::ffi::{CStr, CString};

#[derive(Copy, Clone)]
pub struct Group {
    p: *mut ffi::uiGroup,
    opaque: Opaque,
}

impl Group {
    pub fn from(o: Opaque) -> Option<Group> {
        if o.0 == ::WidgetType::Group {
            return Some(Group {
                p: o.1 as *mut ffi::uiGroup,
                opaque: o,
            });
        }
        None
    }

    pub fn new(name: &str) -> Group {
        let s = CString::new(name).unwrap();
        let p = unsafe { ffi::uiNewGroup(s.as_ptr()) };
        Group {
            p,
            opaque: Opaque(WidgetType::Group, p as _),
        }
    }

    pub fn set_title(&self, txt: &str) {
        let s = CString::new(txt).unwrap();
        unsafe {
            ffi::uiGroupSetTitle(self.p, s.as_ptr());
        }
    }

    pub fn title(&self) -> &str {
        unsafe {
            let slice = CStr::from_ptr(ffi::uiGroupTitle(self.p));
            if let Ok(s) = slice.to_str() {
                s
            } else {
                ""
            }
        }
    }

    pub fn set_child<T: AsRef<Opaque>>(&self, o: T) {
        unsafe {
            ffi::uiGroupSetChild(self.p, o.as_ref().1 as _);
        }
    }

    pub fn set_margined(&self, m: i32) {
        unsafe {
            ffi::uiGroupSetMargined(self.p, m);
        }
    }

    pub fn margined(&self) -> i32 {
        unsafe { ffi::uiGroupMargined(self.p) }
    }
}

impl AsRef<Opaque> for Group {
    fn as_ref(&self) -> &Opaque {
        &self.opaque
    }
}
