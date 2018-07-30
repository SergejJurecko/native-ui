use super::Opaque as ApiOpaque;
use ui::UiImpl;
use {wrappers::EditableCombobox as ImplEditableCombobox, Controller, EvId};

#[derive(Copy, Clone)]
pub struct EditableCombobox {
    op: ApiOpaque,
    b: ImplEditableCombobox,
}

impl EditableCombobox {
    pub fn from(o: ApiOpaque) -> Option<EditableCombobox> {
        if o.0 == ::WidgetType::EditableCombobox {
            if let Some(o1) = UiImpl::get_widget(o.1) {
                return Some(EditableCombobox {
                    op: o,
                    b: ImplEditableCombobox::from(o1).unwrap(),
                });
            }
        }
        None
    }

    pub fn new() -> EditableCombobox {
        let b = ImplEditableCombobox::new();
        let id = UiImpl::new_widget(::ImplOpaque(::WidgetType::EditableCombobox, b.op.1));
        EditableCombobox {
            op: ApiOpaque(::WidgetType::EditableCombobox, id),
            b,
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

    pub fn reg_on_changed<T>(&self, ctrler: &Controller<T>, evid: EvId) {
        if UiImpl::get_widget(self.op.1).is_none() {
            return;
        }
        let id = Box::into_raw(Box::new(::RegId::new(self.op, ctrler.id().0, evid.0)));
        self.b.reg_on_changed(id);
        UiImpl::add_ev(self.op, id);
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
