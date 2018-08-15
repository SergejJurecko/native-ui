// #[cfg(target_os = "macos")]
extern crate cocoa;
use self::cocoa::appkit::{NSApp, NSApplication, NSApplicationDefined};
use self::cocoa::appkit::{NSEvent, NSEventModifierFlags, NSEventSubtype};
use self::cocoa::base::nil;
use self::cocoa::foundation::NSPoint;

mod tray;
pub(crate) use self::tray::*;

pub(crate) fn init(cfg: Option<::Config>) {
    unsafe {
        let app = NSApp();
        app.activateIgnoringOtherApps_(1);
        let cfg = cfg.unwrap_or_default();
        Tray::tray_only(cfg.tray_only);
    }
}

pub(crate) fn post_empty_event() {
    unsafe {
        let event =
                NSEvent::otherEventWithType_location_modifierFlags_timestamp_windowNumber_context_subtype_data1_data2_(
                nil,
                NSApplicationDefined,
                NSPoint::new(0.0, 0.0),
                NSEventModifierFlags::empty(),
                0.0,
                0,
                ::std::ptr::null_mut(),
                NSEventSubtype::NSWindowExposedEventType,
                0,
                0);
        NSApp().postEvent_atStart_(event, 0);
    }
}
