use {ffi, Controller, EvId, ImplOpaque as Opaque, RegId, WidgetType};

#[derive(Copy, Clone)]
pub(crate) struct Spinbox {
    pub op: Opaque,
}

impl Spinbox {
    pub fn from(o: Opaque) -> Option<Spinbox> {
        if o.0 == ::WidgetType::Spinbox {
            return Some(Spinbox { op: o });
        }
        None
    }

    pub fn new(min: i32, max: i32) -> Spinbox {
        let p = unsafe { ffi::uiNewSpinbox(min, max) };
        Spinbox {
            op: Opaque(WidgetType::Spinbox, p as _),
        }
    }

    pub fn set_value(&self, v: i32) {
        unsafe {
            ffi::uiSpinboxSetValue(self.op.1 as _, v);
        }
    }

    pub fn value(&self) -> i32 {
        unsafe { ffi::uiSpinboxValue(self.op.1 as _) }
    }

    pub fn reg_on_changed<T>(&self, ctrler: &Controller<T>, evid: EvId) {
        // let id = ::std::boxed::Box::new(RegId::new(
        //     WidgetType::Spinbox,
        //     ctrler.id().0,
        //     evid.0,
        // ));
        // unsafe {
        //     ffi::uiSpinboxOnChanged(
        //         self.p,
        //         Some(::ui::on_event::<ffi::uiSpinbox>),
        //         Box::into_raw(id) as *mut raw::c_void,
        //     );
        // }
    }
}

impl AsRef<Opaque> for Spinbox {
    fn as_ref(&self) -> &Opaque {
        &self.op
    }
}
