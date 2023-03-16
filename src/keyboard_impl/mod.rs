mod convert_map;
mod types;

#[cfg(target_os = "linux")]
#[path = "x11/mod.rs"]
mod platform;

#[cfg(target_os = "windows")]
#[path = "windows/mod.rs"]
mod platform;
#[cfg(not(test))]
pub use hbb_common::log::{error, trace}; // Use log crate when building application
#[cfg(test)]
pub use std::{println as trace, println as error}; // Workaround to use prinltn! for logs.

pub use self::platform::*;
pub use convert_map::*;
pub use types::*;
