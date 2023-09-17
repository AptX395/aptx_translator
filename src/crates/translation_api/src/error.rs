use std::{error, fmt::{Display, Formatter, Result}};
use crate::Api;

#[derive(Debug)]
pub struct Error {
    // TODO: Implement translate error
    api: Api,
    code: i32,
    reason: String,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        // TODO: Implement translate error hint
        write!(f, "[{:?}][{}]: {}", self.api, self.code, self.reason)
    }
}

impl error::Error for Error {}
