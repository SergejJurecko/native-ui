use {ffi, Controller, EvId, Opaque, RegId, WidgetType};
use std::ffi::{CStr, CString};
use std::os::raw;

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

    pub fn set_child<T: AsRef<Opaque>>(&self, o: T) {
        unsafe {
            ffi::uiWindowSetChild(self.p, o.as_ref().1 as *mut ffi::uiControl);
        }
    }

    pub fn title(&self) -> &str {
        unsafe {
            let slice = CStr::from_ptr(ffi::uiWindowTitle(self.p));
            if let Ok(s) = slice.to_str() {
                s
            } else {
                ""
            }
        }
    }

    pub fn set_title(&self, txt: &str) {
        let s = CString::new(txt).unwrap();
        unsafe {
            ffi::uiWindowSetTitle(self.p, s.as_ptr());
        }
    }

    pub fn reg_on_resize<T>(&self, ctrler: &Controller<T>, evid: EvId) {
        let id = ::std::boxed::Box::new(RegId {
            wt: WidgetType::Window,
            ctrl: ctrler.id().0,
            ev: evid.0,
        });
        unsafe {
            ffi::uiWindowOnContentSizeChanged(
                self.p,
                Some(::ui::on_event::<ffi::uiWindow>),
                Box::into_raw(id) as *mut raw::c_void,
            );
        }
    }

    pub fn reg_on_closing<T>(&self, ctrler: &Controller<T>, evid: EvId) {
        let id = ::std::boxed::Box::new(RegId {
            wt: WidgetType::Window,
            ctrl: ctrler.id().0,
            ev: evid.0,
        });
        unsafe {
            ffi::uiWindowOnClosing(
                self.p,
                Some(::ui::on_close_event::<ffi::uiWindow>),
                Box::into_raw(id) as *mut raw::c_void,
            );
        }
    }

    pub fn content_size(&self) -> (i32, i32) {
        unsafe {
            let mut a = 0;
            let mut b = 0;
            ffi::uiWindowContentSize(self.p, &mut a as _, &mut b as _);
            (a, b)
        }
    }

    pub fn set_content_size(&self, width: i32, height: i32) {
        unsafe {
            ffi::uiWindowSetContentSize(self.p, width, height);
        }
    }

    pub fn set_fullscreen(&self, fs: bool) {
        unsafe {
            ffi::uiWindowSetFullscreen(self.p, fs as i32);
        }
    }

    pub fn fullscreen(&self) -> bool {
        unsafe {
            if ffi::uiWindowFullscreen(self.p) == 0 {
                false
            } else {
                true
            }
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
