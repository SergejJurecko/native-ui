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
mod menu_item;
mod menu;
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
pub use menu_item::*;
pub use menu::*;
// pub trait Widget {
//     fn opaque(&self) -> Opaque;
// }


/// Controller trait that receives events from GUI widgets. 
/// First it must be registered with a widget event, then it must be given to Ui::reg_ctrler.
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

/// Event id so different events can be distinguished.
#[derive(PartialEq, Copy, Clone)]
pub struct EvId(usize);
/// Event of controller that handles events. There can be any number of controllers or 
/// one large one that handles all events.
#[derive(PartialEq, Eq, Hash, Copy, Clone)]
pub struct CtrlId(usize);

use std::os::raw;
#[derive(Hash, PartialEq, Eq)]
struct RegId {
    wt: WidgetType,
    ctrl: usize,
    ev: usize,
    evdata: *mut raw::c_void,
}
impl RegId {
    fn new(wt: WidgetType, ctrl: usize, ev: usize) -> RegId {
        RegId {
            wt, ctrl, ev, evdata: ::std::ptr::null_mut(),
        }
    }

    // fn new_data(wt: WidgetType, ctrl: usize, ev: usize, evdata: *mut raw::c_void) -> RegId {
    //     RegId {
    //         wt, ctrl, ev, evdata,
    //     }
    // }
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
}

#[derive(Clone, Copy, PartialEq, Hash)]
pub struct Opaque(WidgetType, *mut ::std::os::raw::c_void);
