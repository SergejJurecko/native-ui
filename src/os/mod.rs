#[cfg(target_os = "macos")]
#[path = "macos/mod.rs"]
mod os;

pub(crate) use self::os::*;
