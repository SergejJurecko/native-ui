use {ffi, Opaque, WidgetType};

#[derive(Copy, Clone)]
pub struct Separator {
    // p: *mut ffi::uiSeparator,
    opaque: Opaque,
}

impl Separator {
    pub fn from(o: Opaque) -> Option<Separator> {
        if o.0 == ::WidgetType::Separator {
            return Some(Separator {
                // p: o.1 as *mut ffi::uiSeparator,
                opaque: o,
            });
        }
        None
    }

    pub fn new_horizontal() -> Separator {
        let p = unsafe { ffi::uiNewHorizontalSeparator() };
        Separator {
            // p,
            opaque: Opaque(WidgetType::Separator, p as _),
        }
    }

    pub fn new_vertical() -> Separator {
        let p = unsafe { ffi::uiNewVerticalSeparator() };
        Separator {
            // p,
            opaque: Opaque(WidgetType::Separator, p as _),
        }
    }
}

impl AsRef<Opaque> for Separator {
    fn as_ref(&self) -> &Opaque {
        &self.opaque
    }
}
