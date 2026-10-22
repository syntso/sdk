mod output;
mod size;

use crate::error::Error;
use crate::wit::synt::extension::terminal;

pub use output::Output;
pub use size::Size;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Terminal {
    session: u64,
}

impl Terminal {
    pub fn local(size: Size) -> Result<Self, Error> {
        Self::open(None, size)
    }

    pub fn on(host: &str, size: Size) -> Result<Self, Error> {
        Self::open(Some(host), size)
    }

    pub fn open(target: Option<&str>, size: Size) -> Result<Self, Error> {
        let session = terminal::open(target, size.into())?;

        Ok(Self { session })
    }

    pub fn resume(session: u64) -> Self {
        Self { session }
    }

    pub fn session(&self) -> u64 {
        self.session
    }

    pub fn write(&self, input: &[u8]) -> Result<(), Error> {
        Ok(terminal::write(self.session, input)?)
    }

    pub fn read(&self, after: u64) -> Result<Output, Error> {
        Ok(terminal::read(self.session, after)?.into())
    }

    pub fn close(self) -> Result<(), Error> {
        Ok(terminal::close(self.session)?)
    }
}
