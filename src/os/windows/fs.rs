// \Google\Chrome\User Data\Local State

use alloc::{
    string::{String, ToString},
    vec::Vec,
};

use wsyscall_rs::wintypes::{WindowsString, STATUS_SUCCESS, UNICODE_STRING};

use super::*;

/// Copies a file from src location to dst
pub fn copy(src: &WindowsString, dst: &WindowsString) -> crate::Result<()> {
    let file_string = read_to_string(src)?;
    write(dst, file_string)?;
    Ok(())
}

/// Reads a file to a String
pub fn read_to_string(file_path: &WindowsString) -> crate::Result<String> {
    // Initialize parameters needed for `NtCreateFile`
    let mut handle = WindowsHandle(-1isize as *mut core::ffi::c_void);
    let mut io_status = unsafe { core::mem::zeroed::<IO_STATUS_BLOCK>() };
    let mut oa = unsafe { core::mem::zeroed::<OBJECT_ATTRIBUTES>() };
    let mut unicode_string = unsafe { core::mem::zeroed::<UNICODE_STRING>() };

    let mut path_bytes = Vec::with_capacity(4 + file_path.as_bytes().len() + 1);
    path_bytes.extend("\\??\\".encode_utf16());
    path_bytes.extend(file_path.as_bytes());
    path_bytes.push(0);

    unsafe {
        RtlInitUnicodeString(&mut unicode_string, path_bytes.as_ptr());
        InitializeObjectAttributes(
            &mut oa,
            &mut unicode_string,
            OBJ_CASE_INSENSITIVE,
            core::ptr::null_mut(),
            core::ptr::null_mut(),
        );
    };
    unsafe {
        NtCreateFile(
            &mut *handle as *mut _,
            FILE_GENERIC_READ,
            &mut oa,
            &mut io_status,
            core::ptr::null_mut(),
            FILE_ATTRIBUTE_NORMAL,
            FILE_SHARE_READ,
            FILE_OPEN,
            FILE_NON_DIRECTORY_FILE | FILE_SYNCHRONOUS_IO_NONALERT | FILE_SEQUENTIAL_ONLY,
            core::ptr::null_mut(),
            0,
        )
    }?;

    // TODO: reserve size of the file upfront (find which API call is needed??)
    // Creates a variable to store the bytes from the file
    let mut file_bytes = Vec::with_capacity(1024);

    // Loops through the specified file, reads and stores file bytes to the buffer variable (this buffer is overwritten everytime the loop runs).
    // If the returned value is not 0 or end of file, an error would be handled.
    // The bytes read from the file will be appended to the file_bytes vector.
    // If the file returns a STATUS_END_OF_FILE, the loop will end.
    loop {
        let mut io_status = unsafe { core::mem::zeroed::<IO_STATUS_BLOCK>() };
        let mut buffer = [0u8; 1024];
        let mut bytes_read = 0;
        let read_result = unsafe {
            NtReadFile(
                *handle,
                core::ptr::null_mut(),
                core::ptr::null_mut(),
                core::ptr::null_mut(),
                &mut io_status,
                buffer.as_mut_ptr() as *mut _,
                buffer.len() as u32,
                core::ptr::null_mut(),
                core::ptr::null_mut(),
            )
        };
        // IO_STATUS_BLOCK.Information contains the number of bytes read.
        bytes_read += io_status.Information;
        match read_result {
            STATUS_SUCCESS => {
                file_bytes.extend_from_slice(&buffer[0..bytes_read]);
            }
            STATUS_END_OF_FILE => break,
            _ => read_result?,
        };
    }
    Ok(String::from_utf8_lossy(&file_bytes).to_string())
}

/// Write a slice as the entire contents of a file.
///
/// This function will create a file if it does not exist, and will entirely replace its contents if it does.
pub fn write(path: &WindowsString, contents: impl AsRef<[u8]>) -> crate::Result<()> {
    let slice = contents.as_ref();
    // Initialize parameters needed for `NtCreateFile`
    let mut handle = WindowsHandle(-1isize as *mut core::ffi::c_void);
    let mut io_status = unsafe { core::mem::zeroed::<IO_STATUS_BLOCK>() };
    let mut oa = unsafe { core::mem::zeroed::<OBJECT_ATTRIBUTES>() };
    let mut unicode_string = unsafe { core::mem::zeroed::<UNICODE_STRING>() };

    let mut path_bytes = Vec::with_capacity(4 + path.as_bytes().len() + 1);
    path_bytes.extend("\\??\\".encode_utf16());
    path_bytes.extend(path.as_bytes());
    path_bytes.push(0);

    unsafe {
        RtlInitUnicodeString(&mut unicode_string, path_bytes.as_ptr());
        InitializeObjectAttributes(
            &mut oa,
            &mut unicode_string,
            OBJ_CASE_INSENSITIVE,
            core::ptr::null_mut(),
            core::ptr::null_mut(),
        );
    };
    unsafe {
        NtCreateFile(
            &mut *handle as *mut _,
            FILE_GENERIC_WRITE,
            &mut oa,
            &mut io_status,
            core::ptr::null_mut(),
            FILE_ATTRIBUTE_NORMAL,
            FILE_SHARE_READ,
            FILE_OVERWRITE_IF,
            FILE_NON_DIRECTORY_FILE | FILE_SYNCHRONOUS_IO_NONALERT,
            core::ptr::null_mut(),
            0,
        )
    }?;

    unsafe {
        NtWriteFile(
            *handle,
            core::ptr::null_mut(),
            core::ptr::null_mut(),
            core::ptr::null_mut(),
            &mut io_status,
            slice.as_ptr() as *const _,
            slice.len() as u32,
            core::ptr::null_mut(),
            core::ptr::null_mut(),
        )
    }?;

    Ok(())
}
#[cfg(test)]
mod tests {
    extern crate std;
    use std::println;

    use wsyscall_rs::SusGetEnvironmentVariable;

    use crate::{windows_error::WindowsError, Error};

    use super::*;

    #[test]
    fn test_read_file() {
        let mut local_state = SusGetEnvironmentVariable("LOCALAPPDATA").unwrap();
        local_state.push_str("\\Google\\Chrome\\User Data\\Local State");
        println!("Local State: {}", local_state);
        let file_string = read_to_string(&local_state);
        if let Err(ref why) = file_string {
            println!("{}", why)
        }
        assert!(file_string.is_ok());

        let mut bad_file = SusGetEnvironmentVariable("LOCALAPPDATA").unwrap();
        bad_file.push_str("\\Google\\Chrome\\User Data\\Local Stat_BAD");
        let file_string = read_to_string(&bad_file);
        assert!(file_string.is_err());
        assert_eq!(
            file_string.unwrap_err(),
            Error::OsError(WindowsError::new(2))
        );
    }

    #[test]
    fn test_copy_file() {
        let mut local_state = SusGetEnvironmentVariable("LOCALAPPDATA").unwrap();
        local_state.push_str("\\Google\\Chrome\\User Data\\Local State");

        let dst_str = concat!(env!("CARGO_MANIFEST_DIR"), "\\test.txt");
        let dst = WindowsString::from_string(dst_str);
        let ret = copy(&local_state, &dst);
        assert!(ret.is_ok());
        std::fs::remove_file(std::path::Path::new(dst_str)).unwrap();
    }
}
