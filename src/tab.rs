use {ffi, Opaque, WidgetType};
use std::ffi::CString;

#[derive(Copy, Clone)]
pub struct Tab {
    p: *mut ffi::uiTab,
    opaque: Opaque,
}

impl Tab {
    pub fn from(o: Opaque) -> Option<Tab> {
        if o.0 == ::WidgetType::Tab {
            return Some(Tab {
                p: o.1 as *mut ffi::uiTab,
                opaque: o,
            });
        }
        None
    }

    pub fn new() -> Tab {
        let p = unsafe { ffi::uiNewTab() };
        Tab {
            p,
            opaque: Opaque(WidgetType::Tab, p as _),
        }
    }

    pub fn append<T: AsRef<Opaque>>(&self, name: &str, o: T) {
        let s = CString::new(name).unwrap();
        unsafe {
            ffi::uiTabAppend(self.p, s.as_ptr(), o.as_ref().1 as _);
        }
    }

    pub fn insert(&self, txt: &str, before: i32, w: Opaque) {
        let s = CString::new(txt).unwrap();
        unsafe {
            ffi::uiTabInsertAt(self.p, s.as_ptr(), before, w.1 as _);
        }
    }

    pub fn delete(&self, index: i32) {
        unsafe {
            ffi::uiTabDelete(self.p, index);
        }
    }

    pub fn num_pages(&self) -> i32 {
        unsafe { ffi::uiTabNumPages(self.p) }
    }

    pub fn margined(&self, index: i32) -> i32 {
        unsafe { ffi::uiTabMargined(self.p, index) }
    }

    pub fn set_margined(&self, index: i32, m: i32) {
        unsafe {
            ffi::uiTabSetMargined(self.p, index, m);
        }
    }
}

impl AsRef<Opaque> for Tab {
    fn as_ref(&self) -> &Opaque {
        &self.opaque
    }
}
