use super::Opaque as ApiOpaque;
use ui::UiImpl;
use {wrappers::Combobox as ImplCombobox, EvId};

#[derive(Copy, Clone)]
pub struct Combobox {
    op: ApiOpaque,
    b: ImplCombobox,
    gr: ::EvGroup,
}

impl Combobox {
    // pub fn from(o: ApiOpaque) -> Option<Combobox> {
    //     if o.0 == ::WidgetType::Combobox {
    //         if let Some(o1) = UiImpl::get_widget(o.1) {
    //             return Some(Combobox {
    //                 op: o,
    //                 b: ImplCombobox::from(o1).unwrap(),
    //             });
    //         }
    //     }
    //     None
    // }

    pub fn new(gr: ::EvGroup) -> Combobox {
        let b = ImplCombobox::new();
        let id = UiImpl::new_widget(::ImplOpaque(::WidgetType::Combobox, b.op.1), gr);
        Combobox {
            op: ApiOpaque(::WidgetType::Combobox, id),
            b,
            gr,
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

    pub fn reg_on_selected(&self) -> EvId {
        let evid = ::EventLoop::ev_id(self.gr);
        if UiImpl::get_widget(self.op.1).is_none() {
            return evid;
        }
        let id = Box::into_raw(Box::new(::RegId::new(self.op, evid)));
        self.b.reg_on_selected(id);
        UiImpl::add_ev(self.op, id);
        evid
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
