use super::Opaque as ApiOpaque;
use ui::UiImpl;
use {wrappers::MultilineEntry as ImplMultilineEntry, EvId};

#[derive(Copy, Clone)]
pub struct MultilineEntry {
    op: ApiOpaque,
    b: ImplMultilineEntry,
    gr: ::EvGroup,
}

impl MultilineEntry {
    pub fn new(gr: ::EvGroup) -> MultilineEntry {
        let b = ImplMultilineEntry::new();
        Self::new_impl(b, gr)
    }

    fn new_impl(b: ImplMultilineEntry, gr: ::EvGroup) -> MultilineEntry {
        let id = UiImpl::new_widget(::ImplOpaque(::WidgetType::MultilineEntry, b.op.1), gr);
        MultilineEntry {
            op: ApiOpaque(::WidgetType::MultilineEntry, id),
            b,
            gr,
        }
    }

    pub fn new_non_wrapping(gr: ::EvGroup) -> MultilineEntry {
        let b = ImplMultilineEntry::new_non_wrapping();
        Self::new_impl(b, gr)
    }

    pub fn set_text(&self, txt: &str) {
        if UiImpl::get_widget(self.op.1).is_none() {
            return;
        }
        self.b.set_text(txt);
    }

    pub fn append(&self, txt: &str) {
        if UiImpl::get_widget(self.op.1).is_none() {
            return;
        }
        self.b.append(txt);
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

impl ::std::cmp::PartialEq for MultilineEntry {
    fn eq(&self, other: &MultilineEntry) -> bool {
        self.op.1 == other.op.1
    }
}

impl AsRef<ApiOpaque> for MultilineEntry {
    fn as_ref(&self) -> &ApiOpaque {
        &self.op
    }
}
