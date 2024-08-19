use alloc::vec::Vec;

use crate::traits::Browser;

use super::{Cookie, Login};


pub struct ChromiumBrowser {
    master_key: Vec<u8>,
}

impl ChromiumBrowser {
    pub fn new() -> Self {
        ChromiumBrowser {
            master_key: Vec::new(),
        }
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
