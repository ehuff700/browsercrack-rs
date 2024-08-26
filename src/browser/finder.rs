use alloc::{boxed::Box, vec::Vec};
use wsyscall_rs::wintypes::WindowsString;

use crate::{os, traits::Browser, Error};

use super::{chromium::ChromiumBrowser, CHROME_USER_DATA, EDGE_USER_DATA};

#[derive(Debug)]
pub struct BrowserFinder {
    browsers: Vec<Box<dyn Browser>>,
}

impl BrowserFinder {
    /// Searches for chromium-based browsers throughout the operating system.
    fn search_chromium() -> Option<Vec<Box<dyn Browser>>> {
        let mut final_browsers = Vec::new();
        let ud_dirs: [&WindowsString; 2] = unsafe { [&*CHROME_USER_DATA, &*EDGE_USER_DATA] };

        let browsers: Vec<crate::Result<Box<ChromiumBrowser>>> = ud_dirs
            .into_iter()
            .take_while(|fp| os::fs::path_exists(fp))
            .map(|ud_dir| ChromiumBrowser::from_user_data_dir(ud_dir).map(Box::new))
            .collect();

        // TODO: handle errors?
        for b in browsers.into_iter().flatten() {
            final_browsers.push(b as _);
        }
        (!final_browsers.is_empty()).then_some(final_browsers)
    }
    // TODO: Implement search_nss for NSS browsers
    fn _search_nss() -> Option<Vec<Box<dyn Browser>>> {
        todo!()
    }

    /// Searches for valid browsers throughout the operating system.
    pub fn search() -> crate::Result<Self> {
        let browsers = Self::search_chromium().ok_or(Error::NoBrowsersAvailable)?;
        Ok(Self { browsers })
    }

    /// Returns an iterator over the available browsers.
    pub fn iter(&self) -> impl Iterator<Item = &Box<dyn Browser>> {
        self.browsers.iter()
    }
}

impl IntoIterator for BrowserFinder {
    type Item = Box<dyn Browser>;
    type IntoIter = alloc::vec::IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.browsers.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate std;

    #[test]
    fn test_browser_item() {
        let browser_finder = BrowserFinder::search();
        assert!(browser_finder.is_ok());
        let browser_finder = browser_finder.unwrap();
        for browser in browser_finder.iter() {
            std::println!("{:?}", browser)
        }
    }
}
