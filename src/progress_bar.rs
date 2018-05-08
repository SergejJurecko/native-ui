use {ffi, Opaque, WidgetType};

#[derive(Copy, Clone)]
pub struct ProgressBar {
    p: *mut ffi::uiProgressBar,
    opaque: Opaque,
}

impl ProgressBar {
    pub fn from(o: Opaque) -> Option<ProgressBar> {
        if o.0 == ::WidgetType::ProgressBar {
            return Some(ProgressBar {
                p: o.1 as *mut ffi::uiProgressBar,
                opaque: o,
            });
        }
        None
    }

    pub fn new() -> ProgressBar {
        let p = unsafe { ffi::uiNewProgressBar() };
        ProgressBar {
            p,
            opaque: Opaque(WidgetType::ProgressBar, p as _),
        }
    }

    pub fn set_value(&self, v: i32) {
        unsafe {
            ffi::uiProgressBarSetValue(self.p, v);
        }
    }

    pub fn value(&self) -> i32 {
        unsafe {
            ffi::uiProgressBarValue(self.p)
        }
    }
}

impl AsRef<Opaque> for ProgressBar {
    fn as_ref(&self) -> &Opaque {
        &self.opaque
    }
}
