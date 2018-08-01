use super::Opaque as ApiOpaque;
use ui::UiImpl;
use wrappers::Layout as ImplLayout;

#[derive(Copy, Clone)]
pub struct Layout {
    op: ApiOpaque,
    l: ImplLayout,
    // gr: ::EvGroup,
}

impl Layout {
    pub fn new_vertical(gr: ::EvGroup) -> Layout {
        let l = ImplLayout::new_vertical();
        let id = UiImpl::new_widget(l.op.clone(), gr);
        Layout {
            op: ApiOpaque(::WidgetType::Layout, id),
            l,
            // gr,
        }
    }

    pub fn new_horizontal(gr: ::EvGroup) -> Layout {
        let l = ImplLayout::new_horizontal();
        let id = UiImpl::new_widget(l.op.clone(), gr);
        Layout {
            op: ApiOpaque(::WidgetType::Layout, id),
            l,
            // gr,
        }
    }

    pub fn append<T: AsRef<ApiOpaque>>(&self, o: T, strechy: bool) {
        ::int_opaque(o.as_ref()).map(|o| self.l.append(o, strechy));
        UiImpl::push_child(self.op.1, (o.as_ref() as &ApiOpaque).1);
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

impl ::std::cmp::PartialEq for Layout {
    fn eq(&self, other: &Layout) -> bool {
        self.op.1 == other.op.1
    }
}

impl AsRef<ApiOpaque> for Layout {
    fn as_ref(&self) -> &ApiOpaque {
        &self.op
    }
}
