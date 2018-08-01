use std::ffi::{CStr, CString};
use {ffi, ImplOpaque as Opaque, WidgetType};

#[derive(Copy, Clone)]
pub(crate) struct Entry {
    pub op: Opaque,
}

impl Entry {
    pub fn from(o: Opaque) -> Option<Entry> {
        if o.0 == ::WidgetType::Entry {
            return Some(Entry { op: o });
        }
        None
    }

    pub fn new() -> Entry {
        let p = unsafe { ffi::uiNewEntry() };
        Entry {
            op: Opaque(WidgetType::Entry, p as _),
        }
    }

    pub fn new_password() -> Entry {
        let p = unsafe { ffi::uiNewPasswordEntry() };
        Entry {
            op: Opaque(WidgetType::Entry, p as _),
        }
    }

    pub fn new_search() -> Entry {
        let p = unsafe { ffi::uiNewSearchEntry() };
        Entry {
            op: Opaque(WidgetType::Entry, p as _),
        }
    }

    pub fn set_text(&self, txt: &str) {
        let s = CString::new(txt).unwrap();
        unsafe {
            ffi::uiEntrySetText(self.op.1 as _, s.as_ptr());
        }
    }

    pub fn set_read_only(&self, fs: bool) {
        unsafe {
            ffi::uiEntrySetReadOnly(self.op.1 as _, fs as i32);
        }
    }

    pub fn read_only(&self) -> bool {
        unsafe {
            if ffi::uiEntryReadOnly(self.op.1 as _) == 0 {
                false
            } else {
                true
            }
        }
    }

    pub fn text(&self) -> &str {
        unsafe {
            let slice = CStr::from_ptr(ffi::uiEntryText(self.op.1 as _));
            if let Ok(s) = slice.to_str() {
                s
            } else {
                ""
            }
        }
    }

    pub fn reg_on_changed(&self, p: *mut ::RegId) {
        unsafe {
            ffi::uiEntryOnChanged(
                self.op.1 as _,
                Some(on_event),
                p as *mut ::std::os::raw::c_void,
            );
        }
    }
}

unsafe extern "C" fn on_event(_: *mut ffi::uiEntry, reg: *mut ::std::os::raw::c_void) {
    ::ui::on_event(reg);
}

impl AsRef<Opaque> for Entry {
    fn as_ref(&self) -> &Opaque {
        &self.op
    }
}
