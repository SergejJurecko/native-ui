use {ffi, ImplOpaque as Opaque, WidgetType};

#[derive(Copy, Clone)]
pub(crate) struct Separator {
    // p: *mut ffi::uiSeparator,
    pub op: Opaque,
}

impl Separator {
    pub fn from(o: Opaque) -> Option<Separator> {
        if o.0 == ::WidgetType::Separator {
            return Some(Separator { op: o });
        }
        None
    }

    pub fn new_horizontal() -> Separator {
        let p = unsafe { ffi::uiNewHorizontalSeparator() };
        Separator {
            // p,
            op: Opaque(WidgetType::Separator, p as _),
        }
    }

    pub fn new_vertical() -> Separator {
        let p = unsafe { ffi::uiNewVerticalSeparator() };
        Separator {
            // p,
            op: Opaque(WidgetType::Separator, p as _),
        }
    }
}
impl AsRef<Opaque> for Separator {
    fn as_ref(&self) -> &Opaque {
        &self.op
    }
}
