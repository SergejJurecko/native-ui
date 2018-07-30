extern crate fnv;
extern crate native_ui_sys as ffi;

mod api;
mod ui;
mod wrappers;

pub use api::Opaque;
pub use api::*;
pub use ui::*;

/// Controller trait that receives events from GUI widgets, event loop or other threads.
/// First it must be registered with a widget event, then it must be given to Ui::reg_ctrler.
/// Non widget events are registered in Ui.
pub trait Controller<T> {
    /// GUI triggered events
    fn event(&mut self, ev: EvId, obj: api::Opaque);
    /// Window close
    fn close_event(&mut self, ev: EvId, obj: api::Opaque) -> bool;
    /// Inter-controller messages
    fn msg(&mut self, msg: &T);
    /// Created with ui::ctrl_id.
    /// Uniquelly identifies a controller.
    fn id(&self) -> CtrlId;
}
/// Event ID so different events can be distinguished when sent to a single controller.
#[derive(PartialEq, Copy, Clone)]
pub struct EvId(usize);
/// ID of controller that handles events. ID is used to distinguish controllers
/// so messages can be sent between them using Ui::send_msg
#[derive(PartialEq, Eq, Hash, Copy, Clone)]
pub struct CtrlId(usize);

pub(crate) fn int_opaque(o: &api::Opaque) -> Option<ImplOpaque> {
    if let Some(op) = ui::UiImpl::get_widget(o.1) {
        return Some(op.clone());
    }
    None
}

// use std::os::raw;
struct RegId {
    widget: api::Opaque,
    ctrl: usize,
    ev: usize,
    // evdata: *mut raw::c_void,
}
impl RegId {
    fn new(widget: api::Opaque, ctrl: usize, ev: usize) -> RegId {
        RegId {
            widget,
            ctrl,
            ev,
            // evdata: ::std::ptr::null_mut(),
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
struct ImplOpaque(WidgetType, *mut ::std::os::raw::c_void);
