use super::Opaque as ApiOpaque;
use ui::UiImpl;
use wrappers::Group as ImplGroup;

#[derive(Copy, Clone)]
pub struct Group {
    op: ApiOpaque,
    b: ImplGroup,
}

impl Group {
    pub fn from(o: ApiOpaque) -> Option<Group> {
        if o.0 == ::WidgetType::Group {
            if let Some(o1) = UiImpl::get_widget(o.1) {
                return Some(Group {
                    op: o,
                    b: ImplGroup::from(o1).unwrap(),
                });
            }
        }
        None
    }

    pub fn new(name: &str) -> Group {
        let b = ImplGroup::new(name);
        let id = UiImpl::new_widget(::ImplOpaque(::WidgetType::Group, b.op.1));
        Group {
            op: ApiOpaque(::WidgetType::Group, id),
            b,
        }
    }

    pub fn set_title(&self, txt: &str) {
        if UiImpl::get_widget(self.op.1).is_none() {
            return;
        }
        self.b.set_title(txt);
    }

    pub fn title(&self) -> &str {
        if UiImpl::get_widget(self.op.1).is_none() {
            return "";
        }
        self.b.title()
    }

    pub fn set_child<T: AsRef<ApiOpaque>>(&self, o: T) {
        ::int_opaque(o.as_ref()).map(|o| self.b.set_child(o));
        UiImpl::push_child(self.op.1, (o.as_ref() as &ApiOpaque).1);
    }

    pub fn set_margined(&self, m: i32) {
        if UiImpl::get_widget(self.op.1).is_none() {
            return;
        }
        self.b.set_margined(m);
    }

    pub fn margined(&self) -> i32 {
        if UiImpl::get_widget(self.op.1).is_none() {
            return 0;
        }
        self.b.margined()
    }
}

impl ::std::cmp::PartialEq for Group {
    fn eq(&self, other: &Group) -> bool {
        self.op.1 == other.op.1
    }
}

impl AsRef<ApiOpaque> for Group {
    fn as_ref(&self) -> &ApiOpaque {
        &self.op
    }
}
