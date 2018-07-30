use {ffi, ImplOpaque as Opaque, WidgetType};

#[derive(Copy, Clone)]
pub(crate) struct ProgressBar {
    pub op: Opaque,
}

impl ProgressBar {
    pub fn from(o: Opaque) -> Option<ProgressBar> {
        if o.0 == ::WidgetType::ProgressBar {
            return Some(ProgressBar { op: o });
        }
        None
    }

    pub fn new() -> ProgressBar {
        let p = unsafe { ffi::uiNewProgressBar() };
        ProgressBar {
            op: Opaque(WidgetType::ProgressBar, p as _),
        }
    }

    pub fn set_value(&self, v: i32) {
        unsafe {
            ffi::uiProgressBarSetValue(self.op.1 as _, v);
        }
    }

    pub fn value(&self) -> i32 {
        unsafe { ffi::uiProgressBarValue(self.op.1 as _) }
    }
}

impl AsRef<Opaque> for ProgressBar {
    fn as_ref(&self) -> &Opaque {
        &self.op
    }
}
