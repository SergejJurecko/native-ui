use std::ffi::{CStr, CString};
use std::os::raw;
use {ffi, Controller, EvId, ImplOpaque as Opaque, WidgetType};

/// Create windows as well as open message boxes and open/save dialogs. Once created and child is set open with Ui::show.
#[derive(Copy, Clone)]
pub(crate) struct Window {
    pub op: Opaque,
}

impl Window {
    pub fn from(o: Opaque) -> Option<Window> {
        if o.0 == ::WidgetType::Window {
            return Some(Window {
                op: o,
                // on_closing: ::std::ptr::null_mut(),
            });
        }
        None
    }

    pub fn new(title: &str, width: i32, height: i32, has_menu: bool) -> Window {
        let s = CString::new(title).unwrap();
        let p = unsafe { ffi::uiNewWindow(s.as_ptr(), width, height, has_menu as i32) };
        Window {
            op: Opaque(WidgetType::Window, p as _),
        }
    }

    /// And a container child like a layout, group or tab.
    pub fn set_child(&self, o: Opaque) {
        unsafe {
            ffi::uiWindowSetChild(self.op.1 as _, o.1 as *mut ffi::uiControl);
        }
    }

    pub fn title(&self) -> &str {
        unsafe {
            let slice = CStr::from_ptr(ffi::uiWindowTitle(self.op.1 as _));
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
            ffi::uiWindowSetTitle(self.op.1 as _, s.as_ptr());
        }
    }

    pub fn reg_on_resize(&self, p: *mut ::RegId) {
        unsafe {
            ffi::uiWindowOnContentSizeChanged(
                self.op.1 as _,
                Some(on_event),
                p as *mut raw::c_void,
            );
        }
    }

    pub fn reg_on_closing(&self, p: *mut ::RegId) {
        unsafe {
            ffi::uiWindowOnClosing(
                self.op.1 as _,
                Some(::ui::on_close_event::<ffi::uiWindow>),
                p as *mut raw::c_void,
            );
        }
    }

    pub fn open_file(&self) -> &str {
        unsafe {
            let slice = CStr::from_ptr(ffi::uiOpenFile(self.op.1 as _));
            if let Ok(s) = slice.to_str() {
                s
            } else {
                ""
            }
        }
    }

    pub fn save_file(&self) -> &str {
        unsafe {
            let slice = CStr::from_ptr(ffi::uiSaveFile(self.op.1 as _));
            if let Ok(s) = slice.to_str() {
                s
            } else {
                ""
            }
        }
    }

    pub fn msg_box(&self, title: &str, desc: &str) {
        let title = CString::new(title).unwrap();
        let desc = CString::new(desc).unwrap();
        unsafe {
            ffi::uiMsgBox(self.op.1 as _, title.as_ptr(), desc.as_ptr());
        }
    }

    pub fn msg_box_error(&self, title: &str, desc: &str) {
        let title = CString::new(title).unwrap();
        let desc = CString::new(desc).unwrap();
        unsafe {
            ffi::uiMsgBoxError(self.op.1 as _, title.as_ptr(), desc.as_ptr());
        }
    }

    pub fn content_size(&self) -> (i32, i32) {
        unsafe {
            let mut a = 0;
            let mut b = 0;
            ffi::uiWindowContentSize(self.op.1 as _, &mut a as _, &mut b as _);
            (a, b)
        }
    }

    pub fn set_content_size(&self, width: i32, height: i32) {
        unsafe {
            ffi::uiWindowSetContentSize(self.op.1 as _, width, height);
        }
    }

    pub fn set_fullscreen(&self, fs: bool) {
        unsafe {
            ffi::uiWindowSetFullscreen(self.op.1 as _, fs as i32);
        }
    }

    pub fn fullscreen(&self) -> bool {
        unsafe {
            if ffi::uiWindowFullscreen(self.op.1 as _) == 0 {
                false
            } else {
                true
            }
        }
    }

    pub fn set_margined(&self, m: i32) {
        unsafe {
            ffi::uiWindowSetMargined(self.op.1 as _, m);
        }
    }

    pub fn margined(&self) -> i32 {
        unsafe { ffi::uiWindowMargined(self.op.1 as _) }
    }

    pub fn set_borderless(&self, b: i32) {
        unsafe {
            ffi::uiWindowSetBorderless(self.op.1 as _, b);
        }
    }

    pub fn borderless(&self) -> i32 {
        unsafe { ffi::uiWindowBorderless(self.op.1 as _) }
    }
}

unsafe extern "C" fn on_event(_: *mut ffi::uiWindow, reg: *mut ::std::os::raw::c_void) {
    ::ui::on_event::<*mut ffi::uiWindow>(reg);
}

impl AsRef<Opaque> for Window {
    fn as_ref(&self) -> &Opaque {
        &self.op
    }
}
