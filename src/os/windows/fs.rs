// \Google\Chrome\User Data\Local State

use alloc::{string::{String, ToString}, vec::Vec};
use windows_sys::Win32::{Foundation::{FALSE, GENERIC_READ, INVALID_HANDLE_VALUE, TRUE}, Storage::FileSystem::{CopyFileW, CreateFileW, ReadFile, FILE_ATTRIBUTE_NORMAL, FILE_SHARE_READ, OPEN_EXISTING}};

use crate::{os::OsString, Error};

/// Copies a file from src location to dst
pub fn copy(src: &OsString, dst: &OsString) -> crate::Result<()> {
    let ret = unsafe { CopyFileW(src.as_ptr(), dst.as_ptr(), FALSE)};
    if ret == 0 {
        return Err(Error::last_os_error());
    }
    Ok(())
}

/// Reads a file to a String
pub fn read_to_string(file_path: OsString) -> crate::Result<String> {
    // Opens a handle to the file
    let file_handle = unsafe { CreateFileW(file_path.as_ptr(), GENERIC_READ,FILE_SHARE_READ, core::ptr::null_mut(), OPEN_EXISTING, FILE_ATTRIBUTE_NORMAL, core::ptr::null_mut() ) };
    if file_handle == INVALID_HANDLE_VALUE {
       return Err(Error::last_os_error());
    }

    // Creates a variable to store the bytes from the file
    let mut file_bytes = Vec::with_capacity(1024);
    
    // Loops through the specified file, reads and stores file bytes to the buffer variable (this buffer is overwritten everytime the loop runs).
    // If the returned value is 0, an error would be handled.
    // The bytes read from the file will be appended to the file_bytes vector.
    // If the file read reaches 0, the loop will end.
    loop {
        let mut buffer = [0u8; 1024];
        let mut bytes_read = 0;
        let ret = unsafe { ReadFile(file_handle, buffer.as_mut_ptr(), buffer.len() as u32, &mut bytes_read, core::ptr::null_mut() ) };

        if ret == 0 {
            return Err(Error::last_os_error());
        }

        if bytes_read == 0 {
            break;
        }
       
        file_bytes.extend_from_slice(&buffer[0..bytes_read as usize]);
    }
    Ok(String::from_utf8_lossy(&file_bytes).to_string())
}

#[cfg(test)]
mod tests {
    use crate::os::env_var;
    extern crate std;
    use std::println;
    use super::*;
    
    #[test]
    fn test_read_file() {
        let mut local_state = env_var("LOCALAPPDATA").unwrap();
        local_state.push_str("\\Google\\Chrome\\User Data\\Local State\0");
        println!("Local State: {}", local_state);
        let file_string = read_to_string(local_state);
        if let Err(ref why) = file_string {
            println!("{}", why)
        }
        assert!(file_string.is_ok())
    }
}