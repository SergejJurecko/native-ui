use std::ffi::{CStr, CString};
use {ffi, ImplOpaque as Opaque, WidgetType};

#[derive(Copy, Clone)]
pub(crate) struct Group {
    pub op: Opaque,
}

impl Group {
    // pub fn from(o: Opaque) -> Option<Group> {
    //     if o.0 == ::WidgetType::Group {
    //         return Some(Group { op: o });
    //     }
    //     None
    // }

    pub fn new(name: &str) -> Group {
        let s = CString::new(name).unwrap();
        let p = unsafe { ffi::uiNewGroup(s.as_ptr()) };
        Group {
            op: Opaque(WidgetType::Group, p as _),
        }
    }

    pub fn set_title(&self, txt: &str) {
        let s = CString::new(txt).unwrap();
        unsafe {
            ffi::uiGroupSetTitle(self.op.1 as _, s.as_ptr());
        }
    }

    pub fn title(&self) -> &str {
        unsafe {
            let slice = CStr::from_ptr(ffi::uiGroupTitle(self.op.1 as _));
            if let Ok(s) = slice.to_str() {
                s
            } else {
                ""
            }
        }
    }

    pub fn set_child(&self, o: Opaque) {
        unsafe {
            ffi::uiGroupSetChild(self.op.1 as _, o.1 as _);
        }
    }

    pub fn set_margined(&self, m: i32) {
        unsafe {
            ffi::uiGroupSetMargined(self.op.1 as _, m);
        }
    }

    pub fn margined(&self) -> i32 {
        unsafe { ffi::uiGroupMargined(self.op.1 as _) }
    }
}

impl AsRef<Opaque> for Group {
    fn as_ref(&self) -> &Opaque {
        &self.op
    }
}
