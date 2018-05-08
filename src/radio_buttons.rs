use {ffi, Controller, EvId, Opaque, RegId, WidgetType};
use std::ffi::{CString};
use std::os::raw;

#[derive(Copy, Clone)]
pub struct RadioButtons {
    p: *mut ffi::uiRadioButtons,
    opaque: Opaque,
}

impl RadioButtons {
    pub fn from(o: Opaque) -> Option<RadioButtons> {
        if o.0 == ::WidgetType::RadioButtons {
            return Some(RadioButtons {
                p: o.1 as *mut ffi::uiRadioButtons,
                opaque: o,
            });
        }
        None
    }

    pub fn new() -> RadioButtons {
        let p = unsafe { ffi::uiNewRadioButtons() };
        RadioButtons {
            p,
            opaque: Opaque(WidgetType::RadioButtons, p as _),
        }
    }

    pub fn append(&self, name: &str) {
        let s = CString::new(name).unwrap();
        unsafe {
            ffi::uiRadioButtonsAppend(self.p, s.as_ptr());
        }
    }

    pub fn selected(&self) -> i32 {
        unsafe {
            ffi::uiRadioButtonsSelected(self.p)
        }
    }

    pub fn set_selected(&self, v: i32) {
        unsafe {
            ffi::uiRadioButtonsSetSelected(self.p, v);
        }
    }

    pub fn reg_on_selected<T>(&self, ctrler: &Controller<T>, evid: EvId) {
        let id = ::std::boxed::Box::new(RegId::new(
            WidgetType::RadioButtons,
            ctrler.id().0,
            evid.0,
        ));
        unsafe {
            ffi::uiRadioButtonsOnSelected(
                self.p,
                Some(::ui::on_event::<ffi::uiRadioButtons>),
                Box::into_raw(id) as *mut raw::c_void,
            );
        }
    }
}

impl AsRef<Opaque> for RadioButtons {
    fn as_ref(&self) -> &Opaque {
        &self.opaque
    }
}
