use {ffi, Opaque, WidgetType};
use std::ffi::{CStr, CString};

#[derive(Copy, Clone)]
pub struct Label {
    p: *mut ffi::uiLabel,
    opaque: Opaque,
}

impl Label {
    pub fn from(o: Opaque) -> Option<Label> {
        if o.0 == ::WidgetType::Label {
            return Some(Label {
                p: o.1 as *mut ffi::uiLabel,
                opaque: o,
            });
        }
        None
    }

    pub fn new(name: &str) -> Label {
        let s = CString::new(name).unwrap();
        let p = unsafe { ffi::uiNewLabel(s.as_ptr()) };
        Label {
            p,
            opaque: Opaque(WidgetType::Label, p as _),
        }
    }

    pub fn set_text(&self, txt: &str) {
        let s = CString::new(txt).unwrap();
        unsafe {
            ffi::uiLabelSetText(self.p, s.as_ptr());
        }
    }

    pub fn text(&self) -> &str {
        unsafe {
            let slice = CStr::from_ptr(ffi::uiLabelText(self.p));
            if let Ok(s) = slice.to_str() {
                s
            } else {
                ""
            }
        }
    }
}

impl AsRef<Opaque> for Label {
    fn as_ref(&self) -> &Opaque {
        &self.opaque
    }
}
