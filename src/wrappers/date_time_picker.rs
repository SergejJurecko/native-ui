use {ffi, ImplOpaque as Opaque, WidgetType};

#[derive(Copy, Clone)]
pub(crate) struct DateTimePicker {
    pub op: Opaque,
}

impl DateTimePicker {
    pub fn from(o: Opaque) -> Option<DateTimePicker> {
        if o.0 == ::WidgetType::DateTimePicker {
            return Some(DateTimePicker { op: o });
        }
        None
    }

    pub fn new_date_time() -> DateTimePicker {
        let _p = unsafe { ffi::uiNewDateTimePicker() };
        DateTimePicker {
            op: Opaque(WidgetType::DateTimePicker, _p as _),
        }
    }

    pub fn new_date() -> DateTimePicker {
        let _p = unsafe { ffi::uiNewDatePicker() };
        DateTimePicker {
            op: Opaque(WidgetType::DateTimePicker, _p as _),
        }
    }

    pub fn new_time() -> DateTimePicker {
        let _p = unsafe { ffi::uiNewTimePicker() };
        DateTimePicker {
            op: Opaque(WidgetType::DateTimePicker, _p as _),
        }
    }

    pub fn reg_on_changed(&self, p: *mut ::RegId) {
        unsafe {
            ffi::uiDateTimePickerOnChanged(
                self.op.1 as _,
                Some(on_event),
                p as *mut ::std::os::raw::c_void,
            );
        }
    }
}

unsafe extern "C" fn on_event(_: *mut ffi::uiDateTimePicker, reg: *mut ::std::os::raw::c_void) {
    ::ui::on_event(reg);
}

impl AsRef<Opaque> for DateTimePicker {
    fn as_ref(&self) -> &Opaque {
        &self.op
    }
}
