use {ffi, Controller, EvId, Opaque, RegId, WidgetType};
use std::os::raw;

#[derive(Copy, Clone)]
pub struct Slider {
    p: *mut ffi::uiSlider,
    opaque: Opaque,
}

impl Slider {
    pub fn from(o: Opaque) -> Option<Slider> {
        if o.0 == ::WidgetType::Slider {
            return Some(Slider {
                p: o.1 as *mut ffi::uiSlider,
                opaque: o,
            });
        }
        None
    }

    pub fn new(min: i32, max: i32) -> Slider {
        let p = unsafe { ffi::uiNewSlider(min, max) };
        Slider {
            p,
            opaque: Opaque(WidgetType::Slider, p as _),
        }
    }

    pub fn set_value(&self, v: i32) {
        unsafe {
            ffi::uiSliderSetValue(self.p, v);
        }
    }

    pub fn value(&self) -> i32 {
        unsafe {
            ffi::uiSliderValue(self.p)
        }
    }

    pub fn reg_on_changed<T>(&self, ctrler: &Controller<T>, evid: EvId) {
        let id = ::std::boxed::Box::new(RegId {
            wt: WidgetType::Slider,
            ctrl: ctrler.id().0,
            ev: evid.0,
        });
        unsafe {
            ffi::uiSliderOnChanged(
                self.p,
                Some(::ui::on_event::<ffi::uiSlider>),
                Box::into_raw(id) as *mut raw::c_void,
            );
        }
    }
}

impl AsRef<Opaque> for Slider {
    fn as_ref(&self) -> &Opaque {
        &self.opaque
    }
}
