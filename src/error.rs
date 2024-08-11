pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
/// Error type for the browsercrack-rs library.
pub enum Error {}

impl core::error::Error for Error {}
impl core::fmt::Display for Error {
    fn fmt(&self, _f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        todo!()
    }
}
