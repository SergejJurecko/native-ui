// #[cfg(target_os = "macos")]
extern crate cocoa;
use self::cocoa::appkit::{NSApp, NSApplication};

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
