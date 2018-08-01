use super::Opaque as ApiOpaque;
use ui::UiImpl;
use {wrappers::Entry as ImplEntry, EvId};

#[derive(Copy, Clone)]
pub struct Entry {
    op: ApiOpaque,
    b: ImplEntry,
}

impl Entry {
    pub fn from(o: ApiOpaque) -> Option<Entry> {
        if o.0 == ::WidgetType::Entry {
            if let Some(o1) = UiImpl::get_widget(o.1) {
                return Some(Entry {
                    op: o,
                    b: ImplEntry::from(o1).unwrap(),
                });
            }
        }
        None
    }

    pub fn new() -> Entry {
        let b = ImplEntry::new();
        let id = UiImpl::new_widget(::ImplOpaque(::WidgetType::Entry, b.op.1));
        Entry {
            op: ApiOpaque(::WidgetType::Entry, id),
            b,
        }
    }

    pub fn new_password() -> Entry {
        let b = ImplEntry::new_password();
        let id = UiImpl::new_widget(::ImplOpaque(::WidgetType::Entry, b.op.1));
        Entry {
            op: ApiOpaque(::WidgetType::Entry, id),
            b,
        }
    }

    pub fn new_search() -> Entry {
        let b = ImplEntry::new_search();
        let id = UiImpl::new_widget(::ImplOpaque(::WidgetType::Entry, b.op.1));
        Entry {
            op: ApiOpaque(::WidgetType::Entry, id),
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

    pub fn reg_on_changed(&self, evid: EvId) {
        if UiImpl::get_widget(self.op.1).is_none() {
            return;
        }
        let id = Box::into_raw(Box::new(::RegId::new(self.op, evid)));
        self.b.reg_on_changed(id);
        UiImpl::add_ev(self.op, id);
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
