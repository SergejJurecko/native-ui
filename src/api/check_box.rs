use super::Opaque as ApiOpaque;
use ui::UiImpl;
use {wrappers::Checkbox as ImplCheckbox, EvId};

#[derive(Copy, Clone)]
pub struct Checkbox {
    op: ApiOpaque,
    b: ImplCheckbox,
}

impl Checkbox {
    pub fn from(o: ApiOpaque) -> Option<Checkbox> {
        if o.0 == ::WidgetType::Checkbox {
            if let Some(o1) = UiImpl::get_widget(o.1) {
                return Some(Checkbox {
                    op: o,
                    b: ImplCheckbox::from(o1).unwrap(),
                });
            }
        }
        None
    }

    pub fn new(name: &str) -> Checkbox {
        let b = ImplCheckbox::new(name);
        let id = UiImpl::new_widget(::ImplOpaque(::WidgetType::Checkbox, b.op.1));
        Checkbox {
            op: ApiOpaque(::WidgetType::Checkbox, id),
            b,
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

    pub fn reg_on_toggled(&self, evid: EvId) {
        if UiImpl::get_widget(self.op.1).is_none() {
            return;
        }
        let id = Box::into_raw(Box::new(::RegId::new(self.op, evid)));
        self.b.reg_on_toggled(id);
        UiImpl::add_ev(self.op, id);
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
