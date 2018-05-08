use {ffi, Controller, EvId, Opaque, RegId, WidgetType};
use std::ffi::{CStr, CString};
use std::os::raw;

#[derive(Copy, Clone)]
pub struct EditableCombobox {
    p: *mut ffi::uiEditableCombobox,
    opaque: Opaque,
}

impl EditableCombobox {
    pub fn from(o: Opaque) -> Option<EditableCombobox> {
        if o.0 == ::WidgetType::EditableCombobox {
            return Some(EditableCombobox {
                p: o.1 as *mut ffi::uiEditableCombobox,
                opaque: o,
            });
        }
        None
    }

    pub fn new() -> EditableCombobox {
        let p = unsafe { ffi::uiNewEditableCombobox() };
        EditableCombobox {
            p,
            opaque: Opaque(WidgetType::EditableCombobox, p as _),
        }
    }

    pub fn append(&self, name: &str) {
        let s = CString::new(name).unwrap();
        unsafe {
            ffi::uiEditableComboboxAppend(self.p, s.as_ptr());
        }
    }

    pub fn set_text(&self, txt: &str) {
        let s = CString::new(txt).unwrap();
        unsafe {
            ffi::uiEditableComboboxSetText(self.p, s.as_ptr());
        }
    }

    pub fn text(&self) -> &str {
        unsafe {
            let slice = CStr::from_ptr(ffi::uiEditableComboboxText(self.p));
            if let Ok(s) = slice.to_str() {
                s
            } else {
                ""
            }
        }
    }

    pub fn reg_on_changed<T>(&self, ctrler: &Controller<T>, evid: EvId) {
        let id = ::std::boxed::Box::new(RegId::new(
            WidgetType::EditableCombobox,
            ctrler.id().0,
            evid.0,
        ));
        unsafe {
            ffi::uiEditableComboboxOnChanged(
                self.p,
                Some(::ui::on_event::<ffi::uiEditableCombobox>),
                Box::into_raw(id) as *mut raw::c_void,
            );
        }
    }
}

impl AsRef<Opaque> for EditableCombobox {
    fn as_ref(&self) -> &Opaque {
        &self.opaque
    }
}
