use alloc::string::String;
use windows_sys::Win32::{Foundation::{GetLastError, LocalFree}, System::Diagnostics::Debug::{FormatMessageW, FORMAT_MESSAGE_ALLOCATE_BUFFER, FORMAT_MESSAGE_FROM_SYSTEM, FORMAT_MESSAGE_IGNORE_INSERTS}};

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
/// Error type for the browsercrack-rs library.
pub enum Error {
    #[cfg(target_os = "windows")]
    OsError(WindowsError)
}

impl Error {
    pub fn last_os_error() -> Self {
        #[cfg(target_os = "windows")] {
            let last_error_code = unsafe { GetLastError() };
            Self::OsError(WindowsError::new(last_error_code))
        }
    }
}
impl core::error::Error for Error {}
impl core::fmt::Display for Error {
    fn fmt(&self, _f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        todo!()
    }
}

pub struct WindowsError(u32);
impl WindowsError {
    fn new(code: u32) -> Self {
        WindowsError(code)
    }

    fn display_code(&self) -> Option<String> {
        let (buffer, size) = unsafe {
            let mut buffer = core::ptr::null_mut();
            let size = FormatMessageW(FORMAT_MESSAGE_ALLOCATE_BUFFER | FORMAT_MESSAGE_FROM_SYSTEM | FORMAT_MESSAGE_IGNORE_INSERTS,
                core::ptr::null_mut(),
                self.0,
                0,
                &mut buffer as *mut _ as *mut _,
                0,
                core::ptr::null(),);
                (buffer, size)
        };
        if size > 0 {
            let parts = unsafe { core::slice::from_raw_parts(buffer, (size -2) as _)};
            let message = String::from_utf16_lossy(parts);
            unsafe { LocalFree(buffer as *mut _)};
            return Some(message);
        }
        unsafe { LocalFree(buffer as *mut core::ffi::c_void)};
        None
    }
}
impl core::error::Error for WindowsError {}
impl core::fmt::Display for WindowsError { 
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.display_code().unwrap_or(String::from("unknown error message")))
    }
}


impl core::fmt::Debug for WindowsError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "WindowsError(\"{}\")", self.display_code().unwrap_or(String::from("unknown error message")))
    }
}