use super::Opaque as ApiOpaque;
use ui::UiImpl;
use {wrappers::Button as ImplButton, EvId};

#[derive(Copy, Clone)]
pub struct Button {
    op: ApiOpaque,
    b: ImplButton,
    gr: ::EvGroup,
}

impl Button {
    // pub fn from(o: ApiOpaque) -> Option<Button> {
    //     if o.0 == ::WidgetType::Button {
    //         if let Some(o1) = UiImpl::get_widget(o.1) {
    //             return Some(Button {
    //                 op: o,
    //                 b: ImplButton::from(o1).unwrap(),
    //             });
    //         }
    //     }
    //     None
    // }

    pub fn new(name: &str, gr: ::EvGroup) -> Button {
        let b = ImplButton::new(name);
        let id = UiImpl::new_widget(::ImplOpaque(::WidgetType::Button, b.op.1), gr);
        Button {
            op: ApiOpaque(::WidgetType::Button, id),
            b,
            gr,
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

    pub fn reg_on_click(&self) -> EvId {
        let evid = ::EventLoop::ev_id(self.gr);
        if UiImpl::get_widget(self.op.1).is_none() {
            return evid;
        }
        let id = Box::into_raw(Box::new(::RegId::new(self.op, evid)));
        self.b.reg_on_click(id);
        UiImpl::add_ev(self.op, id);
        evid
    }
}

impl ::std::cmp::PartialEq for Button {
    fn eq(&self, other: &Button) -> bool {
        self.op.1 == other.op.1
    }
}

impl AsRef<ApiOpaque> for Button {
    fn as_ref(&self) -> &ApiOpaque {
        &self.op
    }
}
