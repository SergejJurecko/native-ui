use std::ffi::{CStr, CString};
use {ffi, ImplOpaque as Opaque, WidgetType};

#[derive(Copy, Clone)]
pub(crate) struct Label {
    pub op: Opaque,
}

impl Label {
    // pub fn from(o: Opaque) -> Option<Label> {
    //     if o.0 == ::WidgetType::Label {
    //         return Some(Label { op: o });
    //     }
    //     None
    // }

    pub fn new(name: &str) -> Label {
        let s = CString::new(name).unwrap();
        let p = unsafe { ffi::uiNewLabel(s.as_ptr()) };
        Label {
            op: Opaque(WidgetType::Label, p as _),
        }
    }

    pub fn set_text(&self, txt: &str) {
        let s = CString::new(txt).unwrap();
        unsafe {
            ffi::uiLabelSetText(self.op.1 as _, s.as_ptr());
        }
    }

    pub fn text(&self) -> &str {
        unsafe {
            let slice = CStr::from_ptr(ffi::uiLabelText(self.op.1 as _));
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
        &self.op
    }
}
