use {ffi, Controller, EvId, Opaque, RegId, Ui, WidgetType};
use std::ffi::CString;
use std::os::raw;

pub struct Button {
    p: *mut ffi::uiButton,
    opaque: Opaque,
}

impl Button {
    pub fn from(o: Opaque) -> Option<Button> {
        if o.0 == ::WidgetType::Button {
            return Some(Button {
                p: o.1 as *mut ffi::uiButton,
                opaque: o,
            });
        }
        None
    }

    pub fn new(name: &str) -> Button {
        let s = CString::new(name).unwrap();
        let p = unsafe { ffi::uiNewButton(s.as_ptr()) };
        Button { p, opaque: Opaque(WidgetType::Button, p as _) }
    }

    pub fn set_text(&self, txt: &str) {
        let s = CString::new(txt).unwrap();
        unsafe {
            ffi::uiButtonSetText(self.p, s.as_ptr());
        }
    }

    pub fn reg_on_click(&self, ctrler: &Controller, evid: &EvId) {
        let id = ::std::boxed::Box::new(RegId {
            wt: WidgetType::Button,
            ctrl: ctrler.id().0,
            ev: evid.0,
        });
        unsafe {
            ffi::uiButtonOnClicked(
                self.p,
                Some(::ui::on_event::<ffi::uiButton>),
                Box::into_raw(id) as *mut raw::c_void,
            );
        }
    }
}

impl AsRef<Opaque> for Button {
    fn as_ref(&self) -> &Opaque {
        &self.opaque
    }
}

// impl Widget for Button {
//     fn opaque(&self) -> Opaque {
//         Opaque(WidgetType::Button, self.p as *mut ::std::os::raw::c_void)
//     }
// }
