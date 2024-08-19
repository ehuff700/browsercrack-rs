mod fs;

pub(super) mod extra {
    use alloc::vec;
    use alloc::{string::String, vec::Vec};

    use windows_sys::Win32::System::Environment::GetEnvironmentVariableW;

    #[derive(Debug)]
    pub struct OsString {
        pub (crate) bytes: Vec<u16>,
    }

    impl OsString {
        pub fn new(bytes: Vec<u16>) -> Self {
            OsString { bytes }
        }
        
        pub fn from_string(string: &str) -> Self {
            let bytes: Vec<u16> = string.encode_utf16().collect();
            Self { bytes }
        }
        pub fn push_byte(&mut self, b: u16) {
            self.bytes.push(b);
        }
        /// Pushes a string to the end of the buffer.
        pub fn push_str(&mut self, s: &str) {
            self.bytes.extend(s.encode_utf16());
        }

        /// Returns a pointer to the buffers contents.
        pub fn as_ptr(&self) -> *const u16 {
            self.bytes.as_ptr()
        }
    }
    impl core::fmt::Display for OsString {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            let string = String::from_utf16_lossy(&self.bytes);
            write!(f, "{}", string)
        }
    }
    /// Retrieves the value of an environment variable with the given key.
    pub fn env_var(key: &str) -> Option<OsString> {
        let mut key_bytes = key.encode_utf16().collect::<Vec<u16>>();
        key_bytes.push(0); // Null-terminate the string, per docs.

        let chars_needed =
            unsafe { GetEnvironmentVariableW(key_bytes.as_ptr(), core::ptr::null_mut(), 0) };
        if chars_needed == 0 {
            return None;
        }
        let mut wide_str = vec![0u16; chars_needed as usize];

        let ret = unsafe {
            GetEnvironmentVariableW(
                key_bytes.as_ptr(),
                wide_str.as_mut_ptr(),
                wide_str.len() as u32,
            )
        };

        if ret == 0 {
            return None;
        }
        wide_str.truncate(wide_str.len() - 1); // get rid of the null terminator
        Some(OsString { bytes: wide_str })
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        extern crate std;
        use std::println;

        #[test]
        fn test_env_var() {
            let v = env_var("LOCALAPPDATA");
            assert!(v.is_some_and(|ref v| v.bytes[v.bytes.len() - 1] != 0));

        }
    }
}
