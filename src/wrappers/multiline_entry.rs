use std::ffi::{CStr, CString};
use {ffi, EvId, ImplOpaque as Opaque, WidgetType};

#[derive(Copy, Clone)]
pub(crate) struct MultilineEntry {
    pub op: Opaque,
}

impl MultilineEntry {
    pub fn from(o: Opaque) -> Option<MultilineEntry> {
        if o.0 == ::WidgetType::MultilineEntry {
            return Some(MultilineEntry { op: o });
        }
        None
    }

    pub fn new() -> MultilineEntry {
        let p = unsafe { ffi::uiNewMultilineEntry() };
        MultilineEntry {
            op: Opaque(WidgetType::MultilineEntry, p as _),
        }
    }

    pub fn new_non_wrapping() -> MultilineEntry {
        let p = unsafe { ffi::uiNewNonWrappingMultilineEntry() };
        MultilineEntry {
            op: Opaque(WidgetType::MultilineEntry, p as _),
        }
    }

    pub fn append(&self, name: &str) {
        let s = CString::new(name).unwrap();
        unsafe {
            ffi::uiMultilineEntryAppend(self.op.1 as _, s.as_ptr());
        }
    }

    pub fn set_text(&self, txt: &str) {
        let s = CString::new(txt).unwrap();
        unsafe {
            ffi::uiMultilineEntrySetText(self.op.1 as _, s.as_ptr());
        }
    }

    pub fn set_read_only(&self, fs: bool) {
        unsafe {
            ffi::uiMultilineEntrySetReadOnly(self.op.1 as _, fs as i32);
        }
    }

    pub fn read_only(&self) -> bool {
        unsafe {
            if ffi::uiMultilineEntryReadOnly(self.op.1 as _) == 0 {
                false
            } else {
                true
            }
        }
    }

    pub fn text(&self) -> &str {
        unsafe {
            let slice = CStr::from_ptr(ffi::uiMultilineEntryText(self.op.1 as _));
            if let Ok(s) = slice.to_str() {
                s
            } else {
                ""
            }
        }
    }

    pub fn reg_on_changed<T>(&self, p: *mut ::RegId) {
        // let id = ::std::boxed::Box::new(RegId::new(
        //     WidgetType::MultilineEntry,
        //     ctrler.id().0,
        //     evid.0,
        // ));
        // unsafe {
        //     ffi::uiMultilineEntryOnChanged(
        //         self.p,
        //         Some(::ui::on_event::<ffi::uiMultilineEntry>),
        //         Box::into_raw(id) as *mut raw::c_void,
        //     );
        // }
    }
}

impl AsRef<Opaque> for MultilineEntry {
    fn as_ref(&self) -> &Opaque {
        &self.op
    }
}
