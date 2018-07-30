use super::Opaque as ApiOpaque;
use ui::UiImpl;
use {wrappers::Combobox as ImplCombobox, Controller, EvId};

#[derive(Copy, Clone)]
pub struct Combobox {
    op: ApiOpaque,
    b: ImplCombobox,
}

impl Combobox {
    pub fn from(o: ApiOpaque) -> Option<Combobox> {
        if o.0 == ::WidgetType::Combobox {
            if let Some(o1) = UiImpl::get_widget(o.1) {
                return Some(Combobox {
                    op: o,
                    b: ImplCombobox::from(o1).unwrap(),
                });
            }
        }
        None
    }

    pub fn new() -> Combobox {
        let b = ImplCombobox::new();
        let id = UiImpl::new_widget(::ImplOpaque(::WidgetType::Combobox, b.op.1));
        Combobox {
            op: ApiOpaque(::WidgetType::Combobox, id),
            b,
        }
    }

    pub fn append(&self, name: &str) {
        if UiImpl::get_widget(self.op.1).is_none() {
            return;
        }
        self.b.append(name);
    }

    pub fn selected(&self) -> i32 {
        if UiImpl::get_widget(self.op.1).is_none() {
            return 0;
        }
        self.b.selected()
    }

    pub fn set_selected(&self, v: i32) {
        if UiImpl::get_widget(self.op.1).is_none() {
            return;
        }
        self.b.set_selected(v)
    }

    pub fn reg_on_selected<T>(&self, ctrler: &Controller<T>, evid: EvId) {
        if UiImpl::get_widget(self.op.1).is_none() {
            return;
        }
        let id = Box::into_raw(Box::new(::RegId::new(self.op, ctrler.id().0, evid.0)));
        self.b.reg_on_selected(id);
        UiImpl::add_ev(self.op, id);
    }
}

impl ::std::cmp::PartialEq for Combobox {
    fn eq(&self, other: &Combobox) -> bool {
        self.op.1 == other.op.1
    }
}

impl AsRef<ApiOpaque> for Combobox {
    fn as_ref(&self) -> &ApiOpaque {
        &self.op
    }
}
