use {ffi, ImplOpaque as Opaque, WidgetType};

#[derive(Copy, Clone)]
pub(crate) struct Slider {
    pub op: Opaque,
}

impl Slider {
    pub fn from(o: Opaque) -> Option<Slider> {
        if o.0 == ::WidgetType::Slider {
            return Some(Slider { op: o });
        }
        None
    }

    pub fn new(min: i32, max: i32) -> Slider {
        let p = unsafe { ffi::uiNewSlider(min, max) };
        Slider {
            op: Opaque(WidgetType::Slider, p as _),
        }
    }

    pub fn set_value(&self, v: i32) {
        unsafe {
            ffi::uiSliderSetValue(self.op.1 as _, v);
        }
    }

    pub fn value(&self) -> i32 {
        unsafe { ffi::uiSliderValue(self.op.1 as _) }
    }

    pub fn reg_on_changed(&self, p: *mut ::RegId) {
        unsafe {
            ffi::uiSliderOnChanged(
                self.op.1 as _,
                Some(on_event),
                p as *mut ::std::os::raw::c_void,
            );
        }
    }
}

unsafe extern "C" fn on_event(_: *mut ffi::uiSlider, reg: *mut ::std::os::raw::c_void) {
    ::ui::on_event(reg);
}

impl AsRef<Opaque> for Slider {
    fn as_ref(&self) -> &Opaque {
        &self.op
    }
}
