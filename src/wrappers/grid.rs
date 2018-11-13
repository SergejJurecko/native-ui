use std::ffi::CString;
use {ffi, ImplOpaque as Opaque, WidgetType};

#[derive(Copy, Clone)]
pub(crate) struct Grid {
    pub op: Opaque,
}

impl Grid {
    pub fn new() -> Grid {
        let p = unsafe { ffi::uiNewGrid() };
        Grid {
            op: Opaque(WidgetType::Grid, p as _),
        }
    }

    pub fn insert_at(
        &self,
        o: Opaque,
        existing: Opaque,
        at: ::At,
        xspan: i32,
        yspan: i32,
        hexpand: i32,
        halign: ::Align,
        vexpand: i32,
        valign: ::Align,
    ) {
        unsafe {
            ffi::uiGridInsertAt(
                self.op.1 as _,
                o.1 as _,
                existing.1 as _,
                at as u32,
                xspan,
                yspan,
                hexpand,
                halign as u32,
                vexpand,
                valign as u32,
            );
        }
    }

    pub fn append(
        &self,
        o: Opaque,
        left: i32,
        top: i32,
        xspan: i32,
        yspan: i32,
        hexpand: i32,
        halign: ::Align,
        vexpand: i32,
        valign: ::Align,
    ) {
        unsafe {
            ffi::uiGridAppend(
                self.op.1 as _,
                o.1 as _,
                left,
                top,
                xspan,
                yspan,
                hexpand,
                halign as u32,
                vexpand,
                valign as u32,
            );
        }
    }

    pub fn padded(&self) -> i32 {
        unsafe { ffi::uiGridPadded(self.op.1 as _) }
    }

    pub fn set_padded(&self, padded: i32) {
        unsafe {
            ffi::uiGridSetPadded(self.op.1 as _, padded);
        }
    }
}

impl AsRef<Opaque> for Grid {
    fn as_ref(&self) -> &Opaque {
        &self.op
    }
}
