use std::ffi::CString;
use {ffi, Controller, EvId, ImplOpaque as Opaque, RegId, WidgetType};

#[derive(Copy, Clone)]
pub(crate) struct Combobox {
    pub(crate) op: Opaque,
}

impl Combobox {
    pub fn from(o: Opaque) -> Option<Combobox> {
        if o.0 == ::WidgetType::Combobox {
            return Some(Combobox { op: o });
        }
        None
    }

    pub fn new() -> Combobox {
        let p = unsafe { ffi::uiNewCombobox() };
        Combobox {
            op: Opaque(WidgetType::Combobox, p as _),
        }
    }

    pub fn append(&self, name: &str) {
        let s = CString::new(name).unwrap();
        unsafe {
            ffi::uiComboboxAppend(self.op.1 as _, s.as_ptr());
        }
    }

    pub fn selected(&self) -> i32 {
        unsafe { ffi::uiComboboxSelected(self.op.1 as _) }
    }

    pub fn set_selected(&self, v: i32) {
        unsafe {
            ffi::uiComboboxSetSelected(self.op.1 as _, v);
        }
    }

    pub fn reg_on_selected(&self, p: *mut ::RegId) {
        unsafe {
            ffi::uiComboboxOnSelected(
                self.op.1 as _,
                Some(::ui::on_event::<ffi::uiCombobox>),
                p as *mut ::std::os::raw::c_void,
            );
        }
    }
}

impl AsRef<Opaque> for Combobox {
    fn as_ref(&self) -> &Opaque {
        &self.op
    }
}
