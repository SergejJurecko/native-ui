use {ffi, Controller, EvId, Opaque, RegId, WidgetType};
use std::ffi::{CString};
use std::os::raw;

#[derive(Copy, Clone)]
pub struct Combobox {
    p: *mut ffi::uiCombobox,
    opaque: Opaque,
}

impl Combobox {
    pub fn from(o: Opaque) -> Option<Combobox> {
        if o.0 == ::WidgetType::Combobox {
            return Some(Combobox {
                p: o.1 as *mut ffi::uiCombobox,
                opaque: o,
            });
        }
        None
    }

    pub fn new() -> Combobox {
        let p = unsafe { ffi::uiNewCombobox() };
        Combobox {
            p,
            opaque: Opaque(WidgetType::Combobox, p as _),
        }
    }

    pub fn append(&self, name: &str) {
        let s = CString::new(name).unwrap();
        unsafe {
            ffi::uiComboboxAppend(self.p, s.as_ptr());
        }
    }

    pub fn selected(&self) -> i32 {
        unsafe {
            ffi::uiComboboxSelected(self.p)
        }
    }

    pub fn set_selected(&self, v: i32) {
        unsafe {
            ffi::uiComboboxSetSelected(self.p, v);
        }
    }

    pub fn reg_on_selected<T>(&self, ctrler: &Controller<T>, evid: EvId) {
        let id = ::std::boxed::Box::new(RegId::new(
            WidgetType::Combobox,
            ctrler.id().0,
            evid.0,
        ));
        unsafe {
            ffi::uiComboboxOnSelected(
                self.p,
                Some(::ui::on_event::<ffi::uiCombobox>),
                Box::into_raw(id) as *mut raw::c_void,
            );
        }
    }
}

impl AsRef<Opaque> for Combobox {
    fn as_ref(&self) -> &Opaque {
        &self.opaque
    }
}
