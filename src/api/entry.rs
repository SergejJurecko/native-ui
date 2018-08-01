use super::Opaque as ApiOpaque;
use ui::UiImpl;
use {wrappers::Entry as ImplEntry, EvId};

#[derive(Copy, Clone)]
pub struct Entry {
    op: ApiOpaque,
    b: ImplEntry,
    gr: ::EvGroup,
}

impl Entry {
    pub fn new(gr: ::EvGroup) -> Entry {
        let b = ImplEntry::new();
        let id = UiImpl::new_widget(::ImplOpaque(::WidgetType::Entry, b.op.1), gr);
        Entry {
            op: ApiOpaque(::WidgetType::Entry, id),
            b,
            gr,
        }
    }

    pub fn new_password(gr: ::EvGroup) -> Entry {
        let b = ImplEntry::new_password();
        let id = UiImpl::new_widget(::ImplOpaque(::WidgetType::Entry, b.op.1), gr);
        Entry {
            op: ApiOpaque(::WidgetType::Entry, id),
            b,
            gr,
        }
    }

    pub fn new_search(gr: ::EvGroup) -> Entry {
        let b = ImplEntry::new_search();
        let id = UiImpl::new_widget(::ImplOpaque(::WidgetType::Entry, b.op.1), gr);
        Entry {
            op: ApiOpaque(::WidgetType::Entry, id),
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

    pub fn set_read_only(&self, fs: bool) {
        if UiImpl::get_widget(self.op.1).is_none() {
            return;
        }
        self.b.set_read_only(fs);
    }

    pub fn read_only(&self) -> bool {
        if UiImpl::get_widget(self.op.1).is_none() {
            return false;
        }
        self.b.read_only()
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

impl ::std::cmp::PartialEq for Entry {
    fn eq(&self, other: &Entry) -> bool {
        self.op.1 == other.op.1
    }
}

impl AsRef<ApiOpaque> for Entry {
    fn as_ref(&self) -> &ApiOpaque {
        &self.op
    }
}
