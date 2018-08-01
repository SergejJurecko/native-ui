use std::ffi::{CStr, CString};
use std::os::raw;
use {ffi, ImplOpaque as Opaque, WidgetType};

#[derive(Copy, Clone)]
pub(crate) struct Button {
    pub op: Opaque,
}

impl Button {
    pub fn from(o: Opaque) -> Option<Button> {
        if o.0 == ::WidgetType::Button {
            return Some(Button { op: o });
        }
        None
    }

    pub fn new(name: &str) -> Button {
        let s = CString::new(name).unwrap();
        let p = unsafe { ffi::uiNewButton(s.as_ptr()) };
        Button {
            op: Opaque(WidgetType::Button, p as _),
        }
    }

    pub fn set_text(&self, txt: &str) {
        let s = CString::new(txt).unwrap();
        unsafe {
            ffi::uiButtonSetText(self.op.1 as _, s.as_ptr());
        }
    }

    pub fn text(&self) -> &str {
        unsafe {
            let slice = CStr::from_ptr(ffi::uiButtonText(self.op.1 as _));
            if let Ok(s) = slice.to_str() {
                s
            } else {
                ""
            }
        }
    }

    pub fn reg_on_click(&self, p: *mut ::RegId) {
        unsafe {
            ffi::uiButtonOnClicked(self.op.1 as _, Some(on_event), p as *mut raw::c_void);
        }
    }
}

unsafe extern "C" fn on_event(_: *mut ffi::uiButton, reg: *mut raw::c_void) {
    ::ui::on_event::<*mut ffi::uiButton>(reg);
}

impl AsRef<Opaque> for Button {
    fn as_ref(&self) -> &Opaque {
        &self.op
    }
}
