use alloc::vec::Vec;

use crate::browser::{Cookie, Login};

pub trait Browser: core::fmt::Debug {
    /// Queries the browser db for all of the logins.
    fn logins(&self) -> crate::Result<Vec<Login>>;

    /// Queries the browser db for all of the cookies.
    fn cookies(&self) -> crate::Result<Vec<Cookie>>;

    /// Decrypts an encrypted database item.
    fn decrypt(&self, encrypted_bytes: &[u8]) -> crate::Result<Vec<u8>>;
}
