use std::ffi::{CStr, CString};
use {ffi, Controller, EvId, ImplOpaque as Opaque};

#[derive(Copy, Clone)]
pub(crate) struct Checkbox {
    pub op: Opaque,
}

impl Checkbox {
    pub fn from(o: Opaque) -> Option<Checkbox> {
        if o.0 == ::WidgetType::Checkbox {
            return Some(Checkbox { op: o });
        }
        None
    }

    pub fn new(name: &str) -> Checkbox {
        let s = CString::new(name).unwrap();
        let p = unsafe { ffi::uiNewCheckbox(s.as_ptr()) };
        Checkbox {
            op: Opaque(::WidgetType::Checkbox, p as _),
        }
    }

    pub fn set_text(&self, txt: &str) {
        let s = CString::new(txt).unwrap();
        unsafe {
            ffi::uiCheckboxSetText(self.op.1 as _, s.as_ptr());
        }
    }

    pub fn text(&self) -> &str {
        // let s = CString::new(txt).unwrap();
        unsafe {
            let slice = CStr::from_ptr(ffi::uiCheckboxText(self.op.1 as _));
            if let Ok(s) = slice.to_str() {
                s
            } else {
                ""
            }
        }
    }

    pub fn set_checked(&self, v: bool) {
        unsafe {
            ffi::uiCheckboxSetChecked(self.op.1 as _, v as i32);
        }
    }

    pub fn checked(&self) -> bool {
        unsafe {
            if ffi::uiCheckboxChecked(self.op.1 as _) == 0 {
                false
            } else {
                true
            }
        }
    }

    pub fn reg_on_toggled(&self, p: *mut ::RegId) {
        unsafe {
            ffi::uiCheckboxOnToggled(
                self.op.1 as _,
                Some(::ui::on_event::<ffi::uiCheckbox>),
                p as *mut ::std::os::raw::c_void,
            );
        }
    }
}

impl AsRef<Opaque> for Checkbox {
    fn as_ref(&self) -> &Opaque {
        &self.op
    }
}
