use super::Opaque as ApiOpaque;
use ui::UiImpl;
use wrappers::Label as ImplLabel;

#[derive(Copy, Clone)]
pub struct Label {
    op: ApiOpaque,
    b: ImplLabel,
    // gr: ::EvGroup,
}

impl Label {
    pub fn new(name: &str, gr: ::EvGroup) -> Label {
        let b = ImplLabel::new(name);
        let id = UiImpl::new_widget(::ImplOpaque(::WidgetType::Label, b.op.1), gr);
        Label {
            op: ApiOpaque(::WidgetType::Label, id),
            b,
            // gr,
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
}

impl ::std::cmp::PartialEq for Label {
    fn eq(&self, other: &Label) -> bool {
        self.op.1 == other.op.1
    }
}

impl AsRef<ApiOpaque> for Label {
    fn as_ref(&self) -> &ApiOpaque {
        &self.op
    }
}
