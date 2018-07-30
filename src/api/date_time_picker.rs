use super::Opaque as ApiOpaque;
use ui::UiImpl;
use {wrappers::DateTimePicker as ImplDateTimePicker, Controller, EvId};

#[derive(Copy, Clone)]
pub struct DateTimePicker {
    op: ApiOpaque,
    b: ImplDateTimePicker,
}

impl DateTimePicker {
    pub fn from(o: ApiOpaque) -> Option<DateTimePicker> {
        if o.0 == ::WidgetType::DateTimePicker {
            if let Some(o1) = UiImpl::get_widget(o.1) {
                return Some(DateTimePicker {
                    op: o,
                    b: ::wrappers::DateTimePicker::from(o1).unwrap(),
                });
            }
        }
        None
    }

    pub fn new_date_time() -> DateTimePicker {
        let b = ::wrappers::DateTimePicker::new_date_time();
        let id = UiImpl::new_widget(::ImplOpaque(::WidgetType::DateTimePicker, b.op.1));
        DateTimePicker {
            op: ApiOpaque(::WidgetType::DateTimePicker, id),
            b,
        }
    }

    pub fn new_date() -> DateTimePicker {
        let b = ::wrappers::DateTimePicker::new_date();
        let id = UiImpl::new_widget(::ImplOpaque(::WidgetType::DateTimePicker, b.op.1));
        DateTimePicker {
            op: ApiOpaque(::WidgetType::DateTimePicker, id),
            b,
        }
    }

    pub fn new_time() -> DateTimePicker {
        let b = ::wrappers::DateTimePicker::new_time();
        let id = UiImpl::new_widget(::ImplOpaque(::WidgetType::DateTimePicker, b.op.1));
        DateTimePicker {
            op: ApiOpaque(::WidgetType::DateTimePicker, id),
            b,
        }
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
