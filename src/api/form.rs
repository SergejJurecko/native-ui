use super::Opaque as ApiOpaque;
use ui::UiImpl;
use wrappers::Form as ImplForm;

#[derive(Copy, Clone)]
pub struct Form {
    op: ApiOpaque,
    l: ImplForm,
    // gr: ::EvGroup,
}

impl Form {
    pub fn new(gr: ::EvGroup) -> Form {
        let l = ImplForm::new();
        let id = UiImpl::new_widget(l.op.clone(), gr);
        Form {
            op: ApiOpaque(::WidgetType::Form, id),
            l,
            // gr,
        }
    }

    pub fn append<T: AsRef<ApiOpaque>>(&self, label: &str, o: T, strechy: bool) {
        ::int_opaque(o.as_ref()).map(|o| self.l.append(label, o, strechy));
        UiImpl::push_child(self.op.1, (o.as_ref() as &ApiOpaque).1, false);
    }

    pub fn delete(&self, index: i32) {
        if UiImpl::get_widget(self.op.1).is_none() {
            return;
        }
        self.l.delete(index);
    }

    pub fn padded(&self) -> i32 {
        if UiImpl::get_widget(self.op.1).is_none() {
            return 0;
        }
        self.l.padded()
    }

    pub fn set_padded(&self, padded: i32) {
        if UiImpl::get_widget(self.op.1).is_none() {
            return;
        }
        self.l.set_padded(padded);
    }
}

impl ::std::cmp::PartialEq for Form {
    fn eq(&self, other: &Form) -> bool {
        self.op.1 == other.op.1
    }
}

impl AsRef<ApiOpaque> for Form {
    fn as_ref(&self) -> &ApiOpaque {
        &self.op
    }
}
