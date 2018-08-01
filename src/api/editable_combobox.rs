use super::Opaque as ApiOpaque;
use ui::UiImpl;
use {wrappers::EditableCombobox as ImplEditableCombobox, EvId};

#[derive(Copy, Clone)]
pub struct EditableCombobox {
    op: ApiOpaque,
    b: ImplEditableCombobox,
    gr: ::EvGroup,
}

impl EditableCombobox {
    // pub fn from(o: ApiOpaque) -> Option<EditableCombobox> {
    //     if o.0 == ::WidgetType::EditableCombobox {
    //         if let Some(o1) = UiImpl::get_widget(o.1) {
    //             return Some(EditableCombobox {
    //                 op: o,
    //                 b: ImplEditableCombobox::from(o1).unwrap(),
    //             });
    //         }
    //     }
    //     None
    // }

    pub fn new(gr: ::EvGroup) -> EditableCombobox {
        let b = ImplEditableCombobox::new();
        let id = UiImpl::new_widget(::ImplOpaque(::WidgetType::EditableCombobox, b.op.1), gr);
        EditableCombobox {
            op: ApiOpaque(::WidgetType::EditableCombobox, id),
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

    pub fn reg_on_changed<T>(&self) -> EvId {
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

impl ::std::cmp::PartialEq for EditableCombobox {
    fn eq(&self, other: &EditableCombobox) -> bool {
        self.op.1 == other.op.1
    }
}

impl AsRef<ApiOpaque> for EditableCombobox {
    fn as_ref(&self) -> &ApiOpaque {
        &self.op
    }
}
