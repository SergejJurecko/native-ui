use {ffi, ImplOpaque as Opaque, WidgetType};
// use std::ffi::CString;
// use std::os::raw;

#[derive(Copy, Clone)]
pub(crate) struct Layout {
    pub op: Opaque,
}

impl Layout {
    // pub fn from(op: Opaque) -> Option<Layout> {
    //     if op.0 == ::WidgetType::Layout {
    //         return Some(Layout { op });
    //     }
    //     None
    // }

    pub fn new_vertical() -> Layout {
        let p = unsafe { ffi::uiNewVerticalBox() };
        Layout {
            op: Opaque(WidgetType::Layout, p as _),
        }
    }

    pub fn new_horizontal() -> Layout {
        let p = unsafe { ffi::uiNewHorizontalBox() };
        Layout {
            op: Opaque(WidgetType::Layout, p as _),
        }
    }

    pub fn append(&self, o: Opaque, strechy: bool) {
        unsafe {
            ffi::uiBoxAppend(self.op.1 as _, o.1 as _, strechy as _);
        }
    }

    pub fn delete(&self, index: i32) {
        unsafe {
            ffi::uiBoxDelete(self.op.1 as _, index);
        }
    }

    pub fn padded(&self) -> bool {
        unsafe { ffi::uiBoxPadded(self.op.1 as _) != 0 }
    }

    pub fn set_padded(&self, padded: bool) {
        unsafe {
            ffi::uiBoxSetPadded(self.op.1 as _, padded as _);
        }
    }
}

impl AsRef<Opaque> for Layout {
    fn as_ref(&self) -> &Opaque {
        &self.op
    }
}

// impl Widget for Layout {
//     fn opaque(&self) -> Opaque {
//         Opaque(WidgetType::Layout, self.p as *mut ::std::os::raw::c_void)
//     }
// }
