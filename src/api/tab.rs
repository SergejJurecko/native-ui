use super::Opaque as ApiOpaque;
use ui::UiImpl;
use wrappers::Tab as ImplTab;

#[derive(Copy, Clone)]
pub struct Tab {
    op: ApiOpaque,
    b: ImplTab,
    // gr: ::EvGroup,
}

impl Tab {
    pub fn new(gr: ::EvGroup) -> Tab {
        let b = ImplTab::new();
        let id = UiImpl::new_widget(::ImplOpaque(::WidgetType::Tab, b.op.1), gr);
        Tab {
            op: ApiOpaque(::WidgetType::Button, id),
            b,
            // gr,
        }
    }

    pub fn append<T: AsRef<ApiOpaque>>(&self, name: &str, o: T) {
        ::int_opaque(o.as_ref()).map(|o| self.b.append(name, o));
        UiImpl::push_child(self.op.1, (o.as_ref() as &ApiOpaque).1);
    }

    pub fn insert<T: AsRef<ApiOpaque>>(&self, name: &str, before: i32, o: T) {
        ::int_opaque(o.as_ref()).map(|o| self.b.insert(name, before, o));
        UiImpl::push_child(self.op.1, (o.as_ref() as &ApiOpaque).1);
    }

    pub fn delete(&self, index: i32) {
        if UiImpl::get_widget(self.op.1).is_none() {
            return;
        }
        self.b.delete(index);
    }

    pub fn num_pages(&self) -> usize {
        if UiImpl::get_widget(self.op.1).is_none() {
            return 0;
        }
        self.b.num_pages() as _
    }

    pub fn margined(&self, index: usize) -> bool {
        if UiImpl::get_widget(self.op.1).is_none() {
            return false;
        }
        self.b.margined(index as _) > 0
    }

    pub fn set_margined(&self, index: usize, m: bool) {
        if UiImpl::get_widget(self.op.1).is_none() {
            return;
        }
        self.b.set_margined(index as _, if m { 1 } else { 0 });
    }
}

impl ::std::cmp::PartialEq for Tab {
    fn eq(&self, other: &Tab) -> bool {
        self.op.1 == other.op.1
    }
}

impl AsRef<ApiOpaque> for Tab {
    fn as_ref(&self) -> &ApiOpaque {
        &self.op
    }
}
