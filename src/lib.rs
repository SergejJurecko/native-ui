extern crate fnv;
extern crate native_ui_sys as ffi;
#[cfg(target_os = "macos")]
#[macro_use]
extern crate objc;

mod api;
mod os;
mod ui;
mod wrappers;
pub use api::Opaque;
pub use api::*;
pub use ui::*;

/// Event ID so different events can be distinguished when sent to a single controller.
#[derive(PartialEq, Copy, Clone)]
pub struct EvId(usize);

pub(crate) fn int_opaque(o: &api::Opaque) -> Option<ImplOpaque> {
    if let Some(op) = ui::UiImpl::get_widget(o.1) {
        return Some(op.clone());
    }
    None
}

// use std::os::raw;
struct RegId {
    widget: api::Opaque,
    // ctrl: usize,
    ev: EvId,
}
impl RegId {
    fn new(widget: api::Opaque, ev: EvId) -> RegId {
        RegId { widget, ev }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum WidgetType {
    Window,
    Button,
    Layout,
    Checkbox,
    Entry,
    Label,
    Tab,
    Group,
    Spinbox,
    Slider,
    ProgressBar,
    Separator,
    Combobox,
    EditableCombobox,
    RadioButtons,
    DateTimePicker,
    MultilineEntry,
    MenuItem,
    Menu,
    Null,
    Tray,
}

#[derive(Clone, Copy, PartialEq, Hash)]
struct ImplOpaque(WidgetType, *mut ::std::os::raw::c_void);
