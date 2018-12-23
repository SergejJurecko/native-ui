use super::Opaque as ApiOpaque;
use ui::UiImpl;
use {wrappers::RadioButtons as ImplRadBtns, EvId};

#[derive(Copy, Clone)]
pub struct RadioButtons {
    op: ApiOpaque,
    b: ImplRadBtns,
    gr: ::EvGroup,
}

impl RadioButtons {
    pub fn new(gr: ::EvGroup) -> RadioButtons {
        let b = ImplRadBtns::new();
        Self::new_impl(b, gr)
    }

    fn new_impl(b: ImplRadBtns, gr: ::EvGroup) -> RadioButtons {
        let id = UiImpl::new_widget(::ImplOpaque(::WidgetType::RadioButtons, b.op.1), gr);
        RadioButtons {
            op: ApiOpaque(::WidgetType::RadioButtons, id),
            b,
            gr,
        }
    }

    pub fn append(&self, name: &str) {
        self.b.append(name);
        // ::int_opaque(o.as_ref()).map(|o| self.b.append(name));
        // UiImpl::push_child(self.op.1, (o.as_ref() as &ApiOpaque).1, false);
    }

    pub fn set_selected(&self, v: i32) {
        if UiImpl::get_widget(self.op.1).is_none() {
            return;
        }
        self.b.set_selected(v);
    }

    pub fn selected(&self) -> i32 {
        if UiImpl::get_widget(self.op.1).is_none() {
            return -1;
        }
        self.b.selected()
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

impl ::std::cmp::PartialEq for RadioButtons {
    fn eq(&self, other: &RadioButtons) -> bool {
        self.op.1 == other.op.1
    }
}

impl AsRef<ApiOpaque> for RadioButtons {
    fn as_ref(&self) -> &ApiOpaque {
        &self.op
    }
}
