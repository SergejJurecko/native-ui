use {ffi, Controller, EvId, Opaque, RegId, Ui, WidgetType};
use std::ffi::{CStr, CString};
use std::os::raw;

#[derive(Copy, Clone)]
pub struct Checkbox {
    p: *mut ffi::uiCheckbox,
    opaque: Opaque,
}

impl Checkbox {
    pub fn from(o: Opaque) -> Option<Checkbox> {
        if o.0 == ::WidgetType::Checkbox {
            return Some(Checkbox {
                p: o.1 as *mut ffi::uiCheckbox,
                opaque: o,
            });
        }
        None
    }

    pub fn new(name: &str) -> Checkbox {
        let s = CString::new(name).unwrap();
        let p = unsafe { ffi::uiNewCheckbox(s.as_ptr()) };
        Checkbox {
            p,
            opaque: Opaque(WidgetType::Checkbox, p as _),
        }
    }

    pub fn set_text(&self, txt: &str) {
        let s = CString::new(txt).unwrap();
        unsafe {
            ffi::uiCheckboxSetText(self.p, s.as_ptr());
        }
    }

    pub fn text(&self) -> &str {
        // let s = CString::new(txt).unwrap();
        unsafe {
            let slice = CStr::from_ptr(ffi::uiCheckboxText(self.p));
            if let Ok(s) = slice.to_str() {
                s
            } else {
                ""
            }
        }
    }

    pub fn set_checked(&self, v: bool) {
        unsafe {
            ffi::uiCheckboxSetChecked(self.p, v as i32);
        }
    }

    pub fn checked(&self) -> bool {
        unsafe {
            if ffi::uiCheckboxChecked(self.p) == 0 {
                false
            } else {
                true
            }
        }
    }

    pub fn reg_on_toggled<T>(&self, ctrler: &Controller<T>, evid: &EvId) {
        let id = ::std::boxed::Box::new(RegId {
            wt: WidgetType::Checkbox,
            ctrl: ctrler.id().0,
            ev: evid.0,
        });
        unsafe {
            ffi::uiCheckboxOnToggled(
                self.p,
                Some(::ui::on_event::<ffi::uiCheckbox>),
                Box::into_raw(id) as *mut raw::c_void,
            );
        }
    }
}

impl AsRef<Opaque> for Checkbox {
    fn as_ref(&self) -> &Opaque {
        &self.opaque
    }
}
