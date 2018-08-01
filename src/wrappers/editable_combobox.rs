use std::ffi::{CStr, CString};
use {ffi, ImplOpaque as Opaque, WidgetType};

#[derive(Copy, Clone)]
pub(crate) struct EditableCombobox {
    pub(crate) op: Opaque,
}

impl EditableCombobox {
    pub fn from(o: Opaque) -> Option<EditableCombobox> {
        if o.0 == ::WidgetType::EditableCombobox {
            return Some(EditableCombobox { op: o });
        }
        None
    }

    pub fn new() -> EditableCombobox {
        let p = unsafe { ffi::uiNewEditableCombobox() };
        EditableCombobox {
            op: Opaque(WidgetType::EditableCombobox, p as _),
        }
    }

    pub fn append(&self, name: &str) {
        let s = CString::new(name).unwrap();
        unsafe {
            ffi::uiEditableComboboxAppend(self.op.1 as _, s.as_ptr());
        }
    }

    pub fn set_text(&self, txt: &str) {
        let s = CString::new(txt).unwrap();
        unsafe {
            ffi::uiEditableComboboxSetText(self.op.1 as _, s.as_ptr());
        }
    }

    pub fn text(&self) -> &str {
        unsafe {
            let slice = CStr::from_ptr(ffi::uiEditableComboboxText(self.op.1 as _));
            if let Ok(s) = slice.to_str() {
                s
            } else {
                ""
            }
        }
    }

    pub fn reg_on_changed(&self, p: *mut ::RegId) {
        unsafe {
            ffi::uiEditableComboboxOnChanged(
                self.op.1 as _,
                Some(on_event),
                p as *mut ::std::os::raw::c_void,
            );
        }
    }
}

unsafe extern "C" fn on_event(_: *mut ffi::uiEditableCombobox, reg: *mut ::std::os::raw::c_void) {
    ::ui::on_event(reg);
}

impl AsRef<Opaque> for EditableCombobox {
    fn as_ref(&self) -> &Opaque {
        &self.op
    }
}
