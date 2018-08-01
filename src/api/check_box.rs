use super::Opaque as ApiOpaque;
use ui::UiImpl;
use {wrappers::Checkbox as ImplCheckbox, EvId};

#[derive(Copy, Clone)]
pub struct Checkbox {
    op: ApiOpaque,
    b: ImplCheckbox,
    gr: ::EvGroup,
}

impl Checkbox {
    pub fn new(name: &str, gr: ::EvGroup) -> Checkbox {
        let b = ImplCheckbox::new(name);
        let id = UiImpl::new_widget(::ImplOpaque(::WidgetType::Checkbox, b.op.1), gr);
        Checkbox {
            op: ApiOpaque(::WidgetType::Checkbox, id),
            b,
            gr,
        }
    }

    pub fn set_text(&self, txt: &str) {
        if UiImpl::get_widget(self.op.1).is_none() {
            return;
        }
        self.b.set_text(txt);
    }

    pub fn text(&self) -> &str {
        if UiImpl::get_widget(self.op.1).is_none() {
            return "";
        }
        self.b.text()
    }

    pub fn set_checked(&self, v: bool) {
        if UiImpl::get_widget(self.op.1).is_none() {
            return;
        }
        self.b.set_checked(v);
    }

    pub fn checked(&self) -> bool {
        if UiImpl::get_widget(self.op.1).is_none() {
            return false;
        }
        self.b.checked()
    }

    pub fn reg_on_toggled(&self) -> EvId {
        let evid = ::EventLoop::ev_id(self.gr);
        if UiImpl::get_widget(self.op.1).is_none() {
            return evid;
        }
        let id = Box::into_raw(Box::new(::RegId::new(self.op, evid)));
        self.b.reg_on_toggled(id);
        UiImpl::add_ev(self.op, id);
        evid
    }
}

impl ::std::cmp::PartialEq for Checkbox {
    fn eq(&self, other: &Checkbox) -> bool {
        self.op.1 == other.op.1
    }
}

impl AsRef<ApiOpaque> for Checkbox {
    fn as_ref(&self) -> &ApiOpaque {
        &self.op
    }
}
