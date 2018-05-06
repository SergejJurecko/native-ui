mod ffi;
mod window;
mod button;
mod layout;
mod ui;
pub use ui::*;
pub use window::*;
pub use button::*;
pub use layout::*;

// pub trait Widget {
//     fn opaque(&self) -> Opaque;
// }

pub trait Controller {
    fn event(&self, ev: EvId, obj: Opaque);
    /// Created with ui::ctrl_id.
    /// Uniquelly identifies a controller.
    fn id(&self) -> &CtrlId;
}

#[derive(PartialEq)] //Copy, Clone,
pub struct EvId(usize);
#[derive(PartialEq, Eq, Hash)]
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
}

#[derive(Clone, Copy, PartialEq)]
pub struct Opaque(WidgetType, *mut ::std::os::raw::c_void);

// fn gt<'a>(p_state: *mut bcds_trans) -> &'a mut bcds_trans {
// 	unsafe {
// 		&mut *p_state
// 	}
// }

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}