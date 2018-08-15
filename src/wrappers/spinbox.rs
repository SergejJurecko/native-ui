use {ffi, ImplOpaque as Opaque, WidgetType};

#[derive(Copy, Clone)]
pub(crate) struct Spinbox {
    pub op: Opaque,
}

impl Spinbox {
    // pub fn from(o: Opaque) -> Option<Spinbox> {
    //     if o.0 == ::WidgetType::Spinbox {
    //         return Some(Spinbox { op: o });
    //     }
    //     None
    // }

    pub fn new(min: i32, max: i32) -> Spinbox {
        let p = unsafe { ffi::uiNewSpinbox(min, max) };
        Spinbox {
            op: Opaque(WidgetType::Spinbox, p as _),
        }
    }

    pub fn set_value(&self, v: i32) {
        unsafe {
            ffi::uiSpinboxSetValue(self.op.1 as _, v);
        }
    }

    pub fn value(&self) -> i32 {
        unsafe { ffi::uiSpinboxValue(self.op.1 as _) }
    }

    pub fn reg_on_changed(&self, p: *mut ::RegId) {
        unsafe {
            ffi::uiSpinboxOnChanged(
                self.op.1 as _,
                Some(on_event),
                p as *mut ::std::os::raw::c_void,
            );
        }
    }
}

unsafe extern "C" fn on_event(_: *mut ffi::uiSpinbox, reg: *mut ::std::os::raw::c_void) {
    ::ui::on_event(reg);
}

impl AsRef<Opaque> for Spinbox {
    fn as_ref(&self) -> &Opaque {
        &self.op
    }
}
