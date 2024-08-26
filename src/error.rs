use crate::os::*;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, PartialEq)]
/// Error type for the browsercrack-rs library.
pub enum Error {
    #[cfg(target_os = "windows")]
    OsError(windows_error::WindowsError),
    NoBrowsersAvailable,
}

impl Error {
    /// Returns the last operating system error that occurred.
    pub fn last_os_error() -> Self {
        #[cfg(target_os = "windows")]
        {
            let last_error_code = unsafe { GetLastError() };
            Self::OsError(windows_error::WindowsError::new(last_error_code))
        }
    }

    /// Constructs a new `Error` from a raw operating system error code.
    pub fn from_raw_os_error(code: i32) -> Self {
        #[cfg(target_os = "windows")]
        {
            use wsyscall_rs::wintypes::NTSTATUS;
            let rtl_to_win32 = unsafe { RtlNtStatusToDosError(NTSTATUS(code)) };
            Self::OsError(windows_error::WindowsError::new(rtl_to_win32))
        }
    }
}
impl core::error::Error for Error {}
impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::OsError(win_error) => write!(f, "{}", win_error),
            Self::NoBrowsersAvailable => write!(f, "no browsers available"),
        }
    }
}

#[cfg(target_os = "windows")]
pub(crate) mod windows_error {
    use crate::os::{
        FormatMessageW, LocalFree, FORMAT_MESSAGE_ALLOCATE_BUFFER, FORMAT_MESSAGE_FROM_SYSTEM,
        FORMAT_MESSAGE_IGNORE_INSERTS,
    };
    use alloc::string::String;
    use wsyscall_rs::wintypes::NTERROR;

    use super::Error;

    /* From implementation to convert from an NTERROR to this crate's error. */
    impl From<NTERROR> for Error {
        fn from(value: NTERROR) -> Self {
            Error::from_raw_os_error(value.0)
        }
    }

    #[derive(PartialEq)]
    pub struct WindowsError(u32);
    impl WindowsError {
        pub(crate) fn new(code: u32) -> Self {
            WindowsError(code)
        }

        fn display_code(&self) -> Option<String> {
            let (buffer, size) = unsafe {
                let mut buffer = core::ptr::null_mut();
                let size = FormatMessageW(
                    FORMAT_MESSAGE_ALLOCATE_BUFFER
                        | FORMAT_MESSAGE_FROM_SYSTEM
                        | FORMAT_MESSAGE_IGNORE_INSERTS,
                    core::ptr::null_mut(),
                    self.0,
                    0,
                    &mut buffer as *mut _ as *mut _,
                    0,
                    core::ptr::null(),
                );
                (buffer, size)
            };
            if size > 0 {
                let parts = unsafe { core::slice::from_raw_parts(buffer, (size - 2) as _) };
                let message = String::from_utf16_lossy(parts);
                unsafe { LocalFree(buffer as *mut _) };
                return Some(message);
            }
            unsafe { LocalFree(buffer as *mut core::ffi::c_void) };
            None
        }

        fn display(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            write!(
                f,
                "{} ({:#010X})",
                self.display_code()
                    .unwrap_or(String::from("unknown error message")),
                self.0
            )
        }
    }
    impl core::error::Error for WindowsError {}
    impl core::fmt::Display for WindowsError {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            self.display(f)
        }
    }

    impl core::fmt::Debug for WindowsError {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            self.display(f)
        }
    }
}
