#[derive(Clone, Copy, PartialEq, Hash)]
pub struct Opaque(pub(crate) ::WidgetType, pub(crate) usize);

pub enum Align {
    Fill = 0,
    Start = 1,
    Center = 2,
    End = 3,
}

pub enum At {
    Leading = 0,
    Top = 1,
    Trailing = 2,
    Bottom = 3,
}

pub fn disable<T: AsRef<Opaque>>(o: &T) {
    ::int_opaque(o.as_ref()).map(|ex| unsafe {
        ::ffi::uiControlDisable(ex.1 as _);
    });
}
pub fn enable<T: AsRef<Opaque>>(o: &T) {
    ::int_opaque(o.as_ref()).map(|ex| unsafe {
        ::ffi::uiControlEnable(ex.1 as _);
    });
}
pub fn enabled<T: AsRef<Opaque>>(o: &T) -> bool {
    ::int_opaque(o.as_ref())
        .map(|ex| unsafe { ::ffi::uiControlEnabled(ex.1 as _) != 0 })
        .unwrap_or(false)
}

mod button;
mod check_box;
mod combobox;
mod date_time_picker;
mod editable_combobox;
mod entry;
mod form;
mod group;
mod label;
mod layout;
mod multiline_entry;
mod progress_bar;
mod radio_buttons;
mod separator;
mod slider;
mod spinbox;
mod tab;
mod window;
// mod menu_item;
// mod menu;
mod grid;
mod tray;

pub use self::button::*;
pub use self::check_box::*;
pub use self::combobox::*;
pub use self::date_time_picker::*;
pub use self::editable_combobox::*;
pub use self::entry::*;
pub use self::form::*;
pub use self::grid::*;
pub use self::group::*;
pub use self::label::*;
pub use self::layout::*;
pub use self::multiline_entry::*;
pub use self::progress_bar::*;
pub use self::radio_buttons::*;
pub use self::separator::*;
pub use self::slider::*;
pub use self::spinbox::*;
pub use self::tab::*;
pub use self::tray::*;
pub use self::window::*;
// pub use self::menu_item::*;
// pub use self::menu::*;
