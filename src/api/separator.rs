use super::Opaque as ApiOpaque;
use ui::UiImpl;
use wrappers::Separator as ImplSeparator;

#[derive(Copy, Clone)]
pub struct Separator {
    op: ApiOpaque,
    b: ImplSeparator,
    gr: ::EvGroup,
}

impl Separator {
    pub fn new_horizontal(gr: ::EvGroup) -> Separator {
        let b = ImplSeparator::new_horizontal();
        Self::new_int(b, gr)
    }

    pub fn new_vertical(gr: ::EvGroup) -> Separator {
        let b = ImplSeparator::new_vertical();
        Self::new_int(b, gr)
    }

    fn new_int(b: ImplSeparator, gr: ::EvGroup) -> Separator {
        let id = UiImpl::new_widget(::ImplOpaque(::WidgetType::Separator, b.op.1), gr);
        Separator {
            op: ApiOpaque(::WidgetType::Separator, id),
            b,
            gr,
        }
    }
}

impl ::std::cmp::PartialEq for Separator {
    fn eq(&self, other: &Separator) -> bool {
        self.op.1 == other.op.1
    }
}

impl AsRef<ApiOpaque> for Separator {
    fn as_ref(&self) -> &ApiOpaque {
        &self.op
    }
}
