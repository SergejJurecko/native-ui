use super::Opaque as ApiOpaque;
use ui::UiImpl;
use wrappers::ProgressBar as ImplProgBar;

#[derive(Copy, Clone)]
pub struct ProgressBar {
    op: ApiOpaque,
    b: ImplProgBar,
}

impl ProgressBar {
    pub fn new(gr: ::EvGroup) -> ProgressBar {
        let b = ImplProgBar::new();
        let id = UiImpl::new_widget(::ImplOpaque(::WidgetType::ProgressBar, b.op.1), gr);
        ProgressBar {
            op: ApiOpaque(::WidgetType::ProgressBar, id),
            b,
            // gr,
        }
    }

    pub fn set_value(&self, v: i32) {
        if UiImpl::get_widget(self.op.1).is_none() {
            return;
        }
        self.b.set_value(v);
    }

    pub fn value(&self) -> i32 {
        if UiImpl::get_widget(self.op.1).is_none() {
            return -1;
        }
        self.b.value()
    }
}

impl ::std::cmp::PartialEq for ProgressBar {
    fn eq(&self, other: &ProgressBar) -> bool {
        self.op.1 == other.op.1
    }
}

impl AsRef<ApiOpaque> for ProgressBar {
    fn as_ref(&self) -> &ApiOpaque {
        &self.op
    }
}
