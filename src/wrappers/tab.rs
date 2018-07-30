use std::ffi::CString;
use {ffi, ImplOpaque as Opaque, WidgetType};

#[derive(Copy, Clone)]
pub(crate) struct Tab {
    pub op: Opaque,
}

impl Tab {
    pub fn from(o: Opaque) -> Option<Tab> {
        if o.0 == ::WidgetType::Tab {
            return Some(Tab { op: o });
        }
        None
    }

    pub fn new() -> Tab {
        let p = unsafe { ffi::uiNewTab() };
        Tab {
            op: Opaque(WidgetType::Tab, p as _),
        }
    }

    pub fn append(&self, name: &str, o: Opaque) {
        let s = CString::new(name).unwrap();
        unsafe {
            ffi::uiTabAppend(self.op.1 as _, s.as_ptr(), o.1 as _);
        }
    }

    pub fn insert(&self, txt: &str, before: i32, w: Opaque) {
        let s = CString::new(txt).unwrap();
        unsafe {
            ffi::uiTabInsertAt(self.op.1 as _, s.as_ptr(), before, w.1 as _);
        }
    }

    pub fn delete(&self, index: i32) {
        unsafe {
            ffi::uiTabDelete(self.op.1 as _, index);
        }
    }

    pub fn num_pages(&self) -> i32 {
        unsafe { ffi::uiTabNumPages(self.op.1 as _) }
    }

    pub fn margined(&self, index: i32) -> i32 {
        unsafe { ffi::uiTabMargined(self.op.1 as _, index) }
    }

    pub fn set_margined(&self, index: i32, m: i32) {
        unsafe {
            ffi::uiTabSetMargined(self.op.1 as _, index, m);
        }
    }
}

impl AsRef<Opaque> for Tab {
    fn as_ref(&self) -> &Opaque {
        &self.op
    }
}
