#[cfg(target_os = "linux")]
mod linux;

#[cfg(target_os = "macos")]
mod macos;

#[cfg(target_os = "windows")]
mod windows;
pub(crate) use windows::*;

pub mod fs {
    #[cfg(target_os = "windows")]
    pub use crate::os::windows::fs::*;
}
