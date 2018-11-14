use super::Opaque as ApiOpaque;
use ui::UiImpl;
use wrappers::Grid as ImplGrid;

#[derive(Copy, Clone)]
pub struct Grid {
    op: ApiOpaque,
    l: ImplGrid,
}

impl Grid {
    pub fn new(gr: ::EvGroup) -> Grid {
        let l = ImplGrid::new();
        let id = UiImpl::new_widget(l.op.clone(), gr);
        Grid {
            op: ApiOpaque(::WidgetType::Grid, id),
            l,
        }
    }

    pub fn append<T: AsRef<ApiOpaque>>(
        &self,
        o: T,
        left: i32,
        top: i32,
        xspan: i32,
        yspan: i32,
        hexpand: bool,
        halign: ::Align,
        vexpand: bool,
        valign: ::Align,
    ) {
        ::int_opaque(o.as_ref()).map(|o| {
            self.l
                .append(o, left, top, xspan, yspan, hexpand, halign, vexpand, valign)
        });
        UiImpl::push_child(self.op.1, (o.as_ref() as &ApiOpaque).1, false);
    }

    pub fn insert_at<T: AsRef<ApiOpaque>>(
        &self,
        o: T,
        existing: T,
        at: ::At,
        xspan: i32,
        yspan: i32,
        hexpand: i32,
        halign: ::Align,
        vexpand: i32,
        valign: ::Align,
    ) {
        ::int_opaque(o.as_ref()).map(|o| {
            ::int_opaque(existing.as_ref()).map(|ex| {
                self.l
                    .insert_at(o, ex, at, xspan, yspan, hexpand, halign, vexpand, valign);
            });
        });
        UiImpl::remove_child(self.op.1, (existing.as_ref() as &ApiOpaque).1);
        UiImpl::push_child(self.op.1, (o.as_ref() as &ApiOpaque).1, false);
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

impl ::std::cmp::PartialEq for Grid {
    fn eq(&self, other: &Grid) -> bool {
        self.op.1 == other.op.1
    }
}

impl AsRef<ApiOpaque> for Grid {
    fn as_ref(&self) -> &ApiOpaque {
        &self.op
    }
}
