use std::ffi::CString;
use {ffi, ImplOpaque as Opaque, WidgetType};

#[derive(Copy, Clone)]
pub(crate) struct Form {
    pub op: Opaque,
}

impl Form {
    pub fn new() -> Form {
        let p = unsafe { ffi::uiNewForm() };
        Form {
            op: Opaque(WidgetType::Form, p as _),
        }
    }

    pub fn append(&self, label: &str, o: Opaque, strechy: bool) {
        let s = CString::new(label).unwrap();
        unsafe {
            ffi::uiFormAppend(self.op.1 as _, s.as_ptr(), o.1 as _, strechy as _);
        }
    }

    pub fn delete(&self, index: i32) {
        unsafe {
            ffi::uiFormDelete(self.op.1 as _, index);
        }
    }

    pub fn padded(&self) -> i32 {
        unsafe { ffi::uiFormPadded(self.op.1 as _) }
    }

    pub fn set_padded(&self, padded: i32) {
        unsafe {
            ffi::uiFormSetPadded(self.op.1 as _, padded);
        }
    }
}

impl AsRef<Opaque> for Form {
    fn as_ref(&self) -> &Opaque {
        &self.op
    }
}
