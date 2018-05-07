use {ffi, Opaque, WidgetType};
use std::ffi::CString;

#[derive(Copy, Clone)]
pub struct Window {
    p: *mut ffi::uiWindow,
    opaque: Opaque,
}

impl Window {
    pub fn from(o: Opaque) -> Option<Window> {
        if o.0 == ::WidgetType::Window {
            return Some(Window {
                p: o.1 as *mut ffi::uiWindow,
                opaque: o,
            });
        }
        None
    }
    pub fn new(title: &str, width: i32, height: i32, has_menu: bool) -> Window {
        let s = CString::new(title).unwrap();
        let p = unsafe { ffi::uiNewWindow(s.as_ptr(), width, height, has_menu as i32) };
        Window {
            p,
            opaque: Opaque(WidgetType::Window, p as _),
        }
    }

    pub fn set_margined(&self, m: i32) {
        unsafe {
            ffi::uiWindowSetMargined(self.p, m);
        }
    }

    pub fn margined(&self) -> i32 {
        unsafe { ffi::uiWindowMargined(self.p) }
    }

    pub fn set_child<T: AsRef<Opaque>>(&self, o: T) {
        unsafe {
            ffi::uiWindowSetChild(self.p, o.as_ref().1 as *mut ffi::uiControl);
        }
    }

    pub fn set_borderless(&self, b: i32) {
        unsafe {
            ffi::uiWindowSetBorderless(self.p, b);
        }
    }

    pub fn borderless(&self) -> i32 {
        unsafe { ffi::uiWindowBorderless(self.p) }
    }
}

impl AsRef<Opaque> for Window {
    fn as_ref(&self) -> &Opaque {
        &self.opaque
    }
}

// impl Widget for Window {
//     fn opaque(&self) -> Opaque {
//         Opaque(WidgetType::Window, self.p as *mut ::std::os::raw::c_void)
//     }
// }
