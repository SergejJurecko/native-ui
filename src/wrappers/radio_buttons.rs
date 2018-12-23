use std::ffi::CString;
use std::os::raw;
use {ffi, ImplOpaque as Opaque, WidgetType};

#[derive(Copy, Clone)]
pub(crate) struct RadioButtons {
    pub op: Opaque,
}

impl RadioButtons {
    pub fn from(o: Opaque) -> Option<RadioButtons> {
        if o.0 == ::WidgetType::RadioButtons {
            return Some(RadioButtons { op: o });
        }
        None
    }

    pub fn new() -> RadioButtons {
        let p = unsafe { ffi::uiNewRadioButtons() };
        RadioButtons {
            op: Opaque(WidgetType::RadioButtons, p as _),
        }
    }

    pub fn append(&self, name: &str) {
        let s = CString::new(name).unwrap();
        unsafe {
            ffi::uiRadioButtonsAppend(self.op.1 as _, s.as_ptr());
        }
    }

    pub fn selected(&self) -> i32 {
        unsafe { ffi::uiRadioButtonsSelected(self.op.1 as _) }
    }

    pub fn set_selected(&self, v: i32) {
        unsafe {
            ffi::uiRadioButtonsSetSelected(self.op.1 as _, v);
        }
    }

    pub fn reg_on_selected(&self, p: *mut ::RegId) {
        unsafe {
            ffi::uiRadioButtonsOnSelected(
                self.op.1 as _,
                Some(on_event),
                p as *mut ::std::os::raw::c_void,
            );
        }
    }
}

unsafe extern "C" fn on_event(_: *mut ffi::uiRadioButtons, reg: *mut ::std::os::raw::c_void) {
    ::ui::on_event(reg);
}

impl AsRef<Opaque> for RadioButtons {
    fn as_ref(&self) -> &Opaque {
        &self.op
    }
}
