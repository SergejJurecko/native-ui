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
}

impl AsRef<Opaque> for DateTimePicker {
    fn as_ref(&self) -> &Opaque {
        &self.op
    }
}
