use {ffi, Opaque, WidgetType};

#[derive(Copy, Clone)]
pub struct DateTimePicker {
    _p: *mut ffi::uiDateTimePicker,
    opaque: Opaque,
}

impl DateTimePicker {
    pub fn from(o: Opaque) -> Option<DateTimePicker> {
        if o.0 == ::WidgetType::DateTimePicker {
            return Some(DateTimePicker {
                _p: o.1 as _,
                opaque: o,
            });
        }
        None
    }

    pub fn new_date_time() -> DateTimePicker {
        let _p = unsafe { ffi::uiNewDateTimePicker() };
        DateTimePicker {
            _p,
            opaque: Opaque(WidgetType::DateTimePicker, _p as _),
        }
    }

    pub fn new_date() -> DateTimePicker {
        let _p = unsafe { ffi::uiNewDatePicker() };
        DateTimePicker {
            _p,
            opaque: Opaque(WidgetType::DateTimePicker, _p as _),
        }
    }

    pub fn new_time() -> DateTimePicker {
        let _p = unsafe { ffi::uiNewTimePicker() };
        DateTimePicker {
            _p,
            opaque: Opaque(WidgetType::DateTimePicker, _p as _),
        }
    }
}

impl AsRef<Opaque> for DateTimePicker {
    fn as_ref(&self) -> &Opaque {
        &self.opaque
    }
}

// impl Widget for DateTimePicker {
//     fn opaque(&self) -> Opaque {
//         Opaque(WidgetType::DateTimePicker, self.p as *mut ::std::os::raw::c_void)
//     }
// }
