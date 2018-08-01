use super::Opaque as ApiOpaque;
use ui::UiImpl;
use {wrappers::DateTimePicker as ImplDateTimePicker, EvId};

#[derive(Copy, Clone)]
pub struct DateTimePicker {
    op: ApiOpaque,
    b: ImplDateTimePicker,
    gr: ::EvGroup,
}

impl DateTimePicker {
    // pub fn from(o: ApiOpaque) -> Option<DateTimePicker> {
    //     if o.0 == ::WidgetType::DateTimePicker {
    //         if let Some(o1) = UiImpl::get_widget(o.1) {
    //             return Some(DateTimePicker {
    //                 op: o,
    //                 b: ::wrappers::DateTimePicker::from(o1).unwrap(),
    //             });
    //         }
    //     }
    //     None
    // }

    pub fn new_date_time(gr: ::EvGroup) -> DateTimePicker {
        let b = ::wrappers::DateTimePicker::new_date_time();
        let id = UiImpl::new_widget(::ImplOpaque(::WidgetType::DateTimePicker, b.op.1), gr);
        DateTimePicker {
            op: ApiOpaque(::WidgetType::DateTimePicker, id),
            b,
            gr,
        }
    }

    pub fn new_date(gr: ::EvGroup) -> DateTimePicker {
        let b = ::wrappers::DateTimePicker::new_date();
        let id = UiImpl::new_widget(::ImplOpaque(::WidgetType::DateTimePicker, b.op.1), gr);
        DateTimePicker {
            op: ApiOpaque(::WidgetType::DateTimePicker, id),
            b,
            gr,
        }
    }

    pub fn new_time(gr: ::EvGroup) -> DateTimePicker {
        let b = ::wrappers::DateTimePicker::new_time();
        let id = UiImpl::new_widget(::ImplOpaque(::WidgetType::DateTimePicker, b.op.1), gr);
        DateTimePicker {
            op: ApiOpaque(::WidgetType::DateTimePicker, id),
            b,
            gr,
        }
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

impl ::std::cmp::PartialEq for DateTimePicker {
    fn eq(&self, other: &DateTimePicker) -> bool {
        self.op.1 == other.op.1
    }
}

impl AsRef<ApiOpaque> for DateTimePicker {
    fn as_ref(&self) -> &ApiOpaque {
        &self.op
    }
}
