use super::Opaque as ApiOpaque;
use ui::UiImpl;
use {wrappers::Slider as ImplSlider, EvId};

#[derive(Copy, Clone)]
pub struct Slider {
    op: ApiOpaque,
    b: ImplSlider,
    gr: ::EvGroup,
}

impl Slider {
    pub fn new(min: i32, max: i32, gr: ::EvGroup) -> Slider {
        let b = ImplSlider::new(min, max);
        let id = UiImpl::new_widget(::ImplOpaque(::WidgetType::Slider, b.op.1), gr);
        Slider {
            op: ApiOpaque(::WidgetType::Slider, id),
            b,
            gr,
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
            return 0;
        }
        self.b.value()
    }

    pub fn reg_on_changed(&self) -> EvId {
        let evid = ::EventLoop::ev_id(self.gr);
        if UiImpl::get_widget(self.op.1).is_none() {
            return evid;
        }
        let id = Box::into_raw(Box::new(::RegId::new(self.op, evid)));
        self.b.reg_on_changed(id);
        UiImpl::add_ev(self.op, id);
        evid
    }
}

impl ::std::cmp::PartialEq for Slider {
    fn eq(&self, other: &Slider) -> bool {
        self.op.1 == other.op.1
    }
}

impl AsRef<ApiOpaque> for Slider {
    fn as_ref(&self) -> &ApiOpaque {
        &self.op
    }
}
