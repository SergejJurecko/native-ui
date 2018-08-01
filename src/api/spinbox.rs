use super::Opaque as ApiOpaque;
use ui::UiImpl;
use {wrappers::Spinbox as ImplSpinbox, EvId};

#[derive(Copy, Clone)]
pub struct Spinbox {
    op: ApiOpaque,
    b: ImplSpinbox,
    gr: ::EvGroup,
}

impl Spinbox {
    pub fn new(min: i32, max: i32, gr: ::EvGroup) -> Spinbox {
        let b = ImplSpinbox::new(min, max);
        let id = UiImpl::new_widget(::ImplOpaque(::WidgetType::Spinbox, b.op.1), gr);
        Spinbox {
            op: ApiOpaque(::WidgetType::Button, id),
            b,
            gr,
        }
    }

    pub fn set_value(&self, v: i32) {
        if UiImpl::get_widget(self.op.1).is_none() {
            return;
        }
        self.b.set_value(v);
    }

    pub fn value(&self) -> i32 {
        if UiImpl::get_widget(self.op.1).is_none() {
            return 0;
        }
        self.b.value()
    }

    pub fn reg_on_changed(&self) -> EvId {
        let evid = ::EventLoop::ev_id(self.gr);
        if UiImpl::get_widget(self.op.1).is_none() {
            return evid;
        }
        let id = Box::into_raw(Box::new(::RegId::new(self.op, evid)));
        self.b.reg_on_changed(id);
        UiImpl::add_ev(self.op, id);
        evid
    }
}

impl ::std::cmp::PartialEq for Spinbox {
    fn eq(&self, other: &Spinbox) -> bool {
        self.op.1 == other.op.1
    }
}

impl AsRef<ApiOpaque> for Spinbox {
    fn as_ref(&self) -> &ApiOpaque {
        &self.op
    }
}
