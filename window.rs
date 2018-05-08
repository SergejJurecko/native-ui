use {ffi, Controller, EvId, Opaque, RegId, WidgetType};
use std::ffi::{CStr, CString};
use std::os::raw;
use std::boxed::Box;
struct WinInternal {
    p: *mut ffi::uiWindow,
    close: bool,
}

#[derive(Copy, Clone)]
pub struct Window {
    p: *mut WinInternal,
    opaque: Opaque,
}

impl Window {
    pub fn from(o: Opaque) -> Option<Window> {
        if o.0 == ::WidgetType::Window {
            return Some(Window {
                p: o.1 as *mut WinInternal,
                opaque: o,
            });
        }
        None
    }

    pub(crate) fn internal(&self) -> *mut raw::c_void {
        unsafe { (*self.p).p as _ }
    }

    pub fn new(title: &str, width: i32, height: i32, has_menu: bool) -> Window {
        let s = CString::new(title).unwrap();
        let p = unsafe { ffi::uiNewWindow(s.as_ptr(), width, height, has_menu as i32) };
        let p = Box::into_raw(Box::new(WinInternal { p, close: true }));
        Window {
            p,
            opaque: Opaque(WidgetType::Window, p as _),
        }
    }

    pub fn set_child<T: AsRef<Opaque>>(&self, o: T) {
        unsafe {
            ffi::uiWindowSetChild((*self.p).p, o.as_ref().1 as *mut ffi::uiControl);
        }
    }

    // If set to true, it will prevent close on the next on_closing event.
    pub fn prevent_close(&mut self, b: bool) {
        unsafe {
            println!("PREVENT {}", b);
            (*self.p).close = !b;
        }
    }

    pub(crate) fn burn_close(&mut self) -> bool {
        unsafe {
            println!("BURN {}", (*self.p).close);
            if !(*self.p).close {
                (*self.p).close = true;
                return false;
            }
        }
        true
    }

    pub fn title(&self) -> &str {
        unsafe {
            let slice = CStr::from_ptr(ffi::uiWindowTitle((*self.p).p));
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
            ffi::uiWindowSetTitle((*self.p).p, s.as_ptr());
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
                (*self.p).p,
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
                (*self.p).p,
                Some(::ui::on_close_event::<ffi::uiWindow>),
                Box::into_raw(id) as *mut raw::c_void,
            );
        }
    }

    pub fn content_size(&self) -> (i32, i32) {
        unsafe {
            let mut a = 0;
            let mut b = 0;
            ffi::uiWindowContentSize((*self.p).p, &mut a as _, &mut b as _);
            (a, b)
        }
    }

    pub fn set_content_size(&self, width: i32, height: i32) {
        unsafe {
            ffi::uiWindowSetContentSize((*self.p).p, width, height);
        }
    }

    pub fn set_fullscreen(&self, fs: bool) {
        unsafe {
            ffi::uiWindowSetFullscreen((*self.p).p, fs as i32);
        }
    }

    pub fn fullscreen(&self) -> bool {
        unsafe {
            if ffi::uiWindowFullscreen((*self.p).p) == 0 {
                false
            } else {
                true
            }
        }
    }

    pub fn set_margined(&self, m: i32) {
        unsafe {
            ffi::uiWindowSetMargined((*self.p).p, m);
        }
    }

    pub fn margined(&self) -> i32 {
        unsafe { ffi::uiWindowMargined((*self.p).p) }
    }

    pub fn set_borderless(&self, b: i32) {
        unsafe {
            ffi::uiWindowSetBorderless((*self.p).p, b);
        }
    }

    pub fn borderless(&self) -> i32 {
        unsafe { ffi::uiWindowBorderless((*self.p).p) }
    }
}

impl AsRef<Opaque> for Window {
    fn as_ref(&self) -> &Opaque {
        &self.opaque
    }
}
