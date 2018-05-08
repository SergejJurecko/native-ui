extern crate fnv;
extern crate native_ui_sys as ffi;

mod window;
mod button;
mod layout;
mod ui;
mod check_box;
mod entry;
mod label;
mod tab;
mod group;
mod spinbox;
mod slider;
mod progress_bar;
mod separator;
mod combobox;
mod editable_combobox;
mod radio_buttons;
mod date_time_picker;
mod multiline_entry;
pub use ui::*;
pub use window::*;
pub use button::*;
pub use layout::*;
pub use check_box::*;
pub use entry::*;
pub use label::*;
pub use tab::*;
pub use group::*;
pub use spinbox::*;
pub use slider::*;
pub use progress_bar::*;
pub use separator::*;
pub use combobox::*;
pub use editable_combobox::*;
pub use radio_buttons::*;
pub use date_time_picker::*;
pub use multiline_entry::*;
// pub trait Widget {
//     fn opaque(&self) -> Opaque;
// }

pub trait Controller<T> {
    /// GUI triggered events
    fn event(&mut self, ev: EvId, obj: Opaque);
    /// Window close
    fn close_event(&mut self, ev: EvId, obj: Opaque) -> bool;
    /// Inter-controller messages
    fn msg(&mut self, msg: T);
    /// Created with ui::ctrl_id.
    /// Uniquelly identifies a controller.
    fn id(&self) -> CtrlId;
}

#[derive(PartialEq, Copy, Clone)]
pub struct EvId(usize);
#[derive(PartialEq, Eq, Hash, Copy, Clone)]
pub struct CtrlId(usize);

#[derive(Hash, PartialEq, Eq)]
struct RegId {
    wt: WidgetType,
    ctrl: usize,
    ev: usize,
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
}

#[derive(Clone, Copy, PartialEq, Hash)]
pub struct Opaque(WidgetType, *mut ::std::os::raw::c_void);
