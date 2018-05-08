use {ffi, Controller, EvId, Opaque, RegId, WidgetType};
use std::ffi::{CStr, CString};
use std::os::raw;

#[derive(Copy, Clone)]
pub struct Entry {
    p: *mut ffi::uiEntry,
    opaque: Opaque,
}

impl Entry {
    pub fn from(o: Opaque) -> Option<Entry> {
        if o.0 == ::WidgetType::Entry {
            return Some(Entry {
                p: o.1 as *mut ffi::uiEntry,
                opaque: o,
            });
        }
        None
    }

    pub fn new() -> Entry {
        let p = unsafe { ffi::uiNewEntry() };
        Entry {
            p,
            opaque: Opaque(WidgetType::Entry, p as _),
        }
    }

    pub fn new_password() -> Entry {
        let p = unsafe { ffi::uiNewPasswordEntry() };
        Entry {
            p,
            opaque: Opaque(WidgetType::Entry, p as _),
        }
    }

    pub fn new_search() -> Entry {
        let p = unsafe { ffi::uiNewSearchEntry() };
        Entry {
            p,
            opaque: Opaque(WidgetType::Entry, p as _),
        }
    }

    pub fn set_text(&self, txt: &str) {
        let s = CString::new(txt).unwrap();
        unsafe {
            ffi::uiEntrySetText(self.p, s.as_ptr());
        }
    }

    pub fn set_read_only(&self, fs: bool) {
        unsafe {
            ffi::uiEntrySetReadOnly(self.p, fs as i32);
        }
    }

    pub fn read_only(&self) -> bool {
        unsafe {
            if ffi::uiEntryReadOnly(self.p) == 0 {
                false
            } else {
                true
            }
        }
    }

    pub fn text(&self) -> &str {
        unsafe {
            let slice = CStr::from_ptr(ffi::uiEntryText(self.p));
            if let Ok(s) = slice.to_str() {
                s
            } else {
                ""
            }
        }
    }

    pub fn reg_on_changed<T>(&self, ctrler: &Controller<T>, evid: EvId) {
        let id = ::std::boxed::Box::new(RegId::new(
            WidgetType::Entry,
            ctrler.id().0,
            evid.0,
        ));
        unsafe {
            ffi::uiEntryOnChanged(
                self.p,
                Some(::ui::on_event::<ffi::uiEntry>),
                Box::into_raw(id) as *mut raw::c_void,
            );
        }
    }
}

impl AsRef<Opaque> for Entry {
    fn as_ref(&self) -> &Opaque {
        &self.opaque
    }
}
