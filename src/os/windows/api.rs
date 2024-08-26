#![allow(non_camel_case_types, non_snake_case, clippy::upper_case_acronyms)]

mod constants {
    use wsyscall_rs::wintypes::NTSTATUS;

    use super::*;

    pub type HANDLE = *mut core::ffi::c_void;
    pub type PVOID = *mut core::ffi::c_void;
    pub type CONST_PVOID = *const core::ffi::c_void;
    pub type ULONG = core::ffi::c_ulong;

    pub type PIO_APC_ROUTINE = unsafe extern "system" fn(
        ApcContext: *mut core::ffi::c_void,
        IoStatusBlock: *mut IO_STATUS_BLOCK,
        Reserved: u32,
    );

    pub const FILE_ATTRIBUTE_NORMAL: u32 = 128;
    pub const FILE_GENERIC_READ: u32 = 1179785;
    pub const FILE_GENERIC_WRITE: u32 = 1179926;
    pub const FILE_OPEN: u32 = 1;
    pub const FILE_OVERWRITE_IF: u32 = 5;
    pub const FILE_SEQUENTIAL_ONLY: u32 = 4;
    pub const FILE_SHARE_READ: u32 = 1;
    pub const FILE_SYNCHRONOUS_IO_NONALERT: u32 = 0x00000020;
    pub const FILE_NON_DIRECTORY_FILE: u32 = 0x00000040;
    pub const OBJ_CASE_INSENSITIVE: u32 = 0x00000040;

    pub const FORMAT_MESSAGE_ALLOCATE_BUFFER: u32 = 256;
    pub const FORMAT_MESSAGE_FROM_SYSTEM: u32 = 4096;
    pub const FORMAT_MESSAGE_IGNORE_INSERTS: u32 = 512;
    pub const STATUS_END_OF_FILE: NTSTATUS = NTSTATUS(0xc0000011u32 as i32);
}

#[allow(non_snake_case, non_camel_case_types)]
mod structs {
    use wsyscall_rs::wintypes::{NTSTATUS, UNICODE_STRING};

    use super::{NtClose, HANDLE};

    #[repr(C)]
    pub struct IO_STATUS_BLOCK {
        pub u: IO_STATUS_BLOCK_u,
        pub Information: usize,
    }

    #[repr(C)]
    pub union IO_STATUS_BLOCK_u {
        pub Status: NTSTATUS,
        pub Pointer: *mut core::ffi::c_void,
    }

    #[repr(C)]
    pub struct OBJECT_ATTRIBUTES {
        pub Length: u32,
        pub RootDirectory: *mut core::ffi::c_void,
        pub ObjectName: *mut UNICODE_STRING,
        pub Attributes: u32,
        pub SecurityDescriptor: *mut core::ffi::c_void,
        pub SecurityQualityOfService: *mut core::ffi::c_void,
    }

    #[repr(C)]
    pub struct FILE_BASIC_INFORMATION {
        pub CreationTime: i64,
        pub LastAccessTime: i64,
        pub LastWriteTime: i64,
        pub ChangeTime: i64,
        pub FileAttributes: u32,
    }

    #[repr(transparent)]
    #[derive(Debug)]
    /// Custom struct to wrap windows handles returned by the Nt API, to automatically close them on Drop.
    pub struct WindowsHandle(pub(crate) HANDLE);
    impl core::ops::Deref for WindowsHandle {
        type Target = *mut core::ffi::c_void;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl core::ops::DerefMut for WindowsHandle {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl core::ops::Drop for WindowsHandle {
        fn drop(&mut self) {
            let _ = unsafe { NtClose(self.0) };
        }
    }
}

#[allow(non_snake_case, non_camel_case_types)]
mod functions {
    use super::constants::PIO_APC_ROUTINE;
    use super::{CONST_PVOID, FILE_BASIC_INFORMATION, HANDLE, PVOID, ULONG};

    use wsyscall_rs::wintypes::NTSTATUS;
    use wsyscall_rs::{dynamic_invoke_imp, syscall};
    use wsyscall_rs::{syscall_imp, wintypes::UNICODE_STRING};

    use super::structs::{IO_STATUS_BLOCK, OBJECT_ATTRIBUTES};

    #[inline]
    pub unsafe fn InitializeObjectAttributes(
        p: *mut OBJECT_ATTRIBUTES,
        n: *mut UNICODE_STRING,
        a: u32,
        r: *mut core::ffi::c_void,
        s: *mut core::ffi::c_void,
    ) {
        (*p).Length = size_of::<OBJECT_ATTRIBUTES>() as u32;
        (*p).RootDirectory = r;
        (*p).Attributes = a;
        (*p).ObjectName = n;
        (*p).SecurityDescriptor = s;
        (*p).SecurityQualityOfService = core::ptr::null_mut();
    }

    syscall_imp!(NtCreateFile, (
        FileHandle: *mut HANDLE,
        DesiredAccess: ULONG,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        IoStatusBlock: *mut IO_STATUS_BLOCK,
        AllocationSize: PVOID,
        FileAttributes: ULONG,
        ShareAccess: ULONG,
        CreateDisposition: ULONG,
        CreateOptions: ULONG,
        EaBuffer: PVOID,
        EaLength: ULONG
    ));
    syscall_imp!(NtReadFile, (
        Handle: HANDLE,
        Event: HANDLE,
        ApcRoutine: *mut PIO_APC_ROUTINE,
        ApcContext: CONST_PVOID,
        IoStatusBlock: *mut IO_STATUS_BLOCK,
        Buffer: PVOID,
        Length: ULONG,
        ByteOffset: CONST_PVOID,
        Key: *const ULONG
    ));

    syscall_imp!(NtWriteFile, (
        FileHandle: HANDLE,
        Event: HANDLE,
        ApcRoutine: *mut PIO_APC_ROUTINE,
        ApcContext: CONST_PVOID,
        IoStatusBlock: *mut IO_STATUS_BLOCK,
        Buffer: CONST_PVOID,
        Length: ULONG,
        ByteOffset: CONST_PVOID,
        Key: *const ULONG
    ));

    syscall_imp!(NtQueryAttributesFile, (ObjectAttributes: *const OBJECT_ATTRIBUTES, FileInformation: *mut FILE_BASIC_INFORMATION));
    syscall_imp!(NtClose, (Handle: HANDLE));

    /* ntdll.dll imports */
    dynamic_invoke_imp!("ntdll.dll", RtlInitUnicodeString, (DestinationString: *mut UNICODE_STRING, SourceString: *const u16));
    dynamic_invoke_imp!("ntdll.dll", RtlNtStatusToDosError, (Status: NTSTATUS) -> u32);

    /* kernel32.dll imports */
    dynamic_invoke_imp!("KERNEL32.DLL", FormatMessageW, (dwflags: u32, lpsource: *const core::ffi::c_void, dwmessageid: u32, dwlanguageid: u32, lpbuffer: *mut u16, nsize: u32, arguments: *const *const i8) -> u32);
    dynamic_invoke_imp!("KERNEL32.DLL", LocalFree, (hmem: *mut core::ffi::c_void) -> core::ffi::c_void);
    dynamic_invoke_imp!("KERNEL32.DLL", GetLastError, () -> u32);
    dynamic_invoke_imp!("KERNEL32.DLL", SetLastError, (dwerrorcode: u32));
}

pub use constants::*;
pub use functions::*;
pub use structs::*;
