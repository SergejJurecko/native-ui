use {ffi, Opaque, WidgetType};
// use std::ffi::CString;
// use std::os::raw;

pub struct Layout {
    p: *mut ffi::uiBox,
    opaque: Opaque,
}

impl Layout {
    pub fn from(o: Opaque) -> Option<Layout> {
        if o.0 == ::WidgetType::Layout {
            return Some(Layout {
                p: o.1 as _,
                opaque: o,
            });
        }
        None
    }

    pub fn new_vertical() -> Layout {
        let p = unsafe { ffi::uiNewVerticalBox() };
        Layout { p, opaque: Opaque(WidgetType::Layout, p as _) }
    }

    pub fn new_horizontal() -> Layout {
        let p = unsafe { ffi::uiNewHorizontalBox() };
        Layout { p, opaque: Opaque(WidgetType::Layout, p as _) }
    }

    pub fn append<T: AsRef<Opaque>>(&self, o: T, strechy: bool) {
        unsafe {
            ffi::uiBoxAppend(self.p, o.as_ref().1 as _, strechy as _);
        }
    }

    pub fn delete(&self, index: i32) {
        unsafe {
            ffi::uiBoxDelete(self.p, index);
        }
    }

    pub fn padded(&self) -> i32 {
        unsafe {
            ffi::uiBoxPadded(self.p)
        }
    }

    pub fn set_padded(&self, padded: i32) {
        unsafe {
            ffi::uiBoxSetPadded(self.p, padded);
        }
    }
}

impl AsRef<Opaque> for Layout {
    fn as_ref(&self) -> &Opaque {
        &self.opaque
    }
}

// impl Widget for Layout {
//     fn opaque(&self) -> Opaque {
//         Opaque(WidgetType::Layout, self.p as *mut ::std::os::raw::c_void)
//     }
// }
