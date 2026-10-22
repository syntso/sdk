use crate::error::Error;
use crate::wit::synt::extension::progress;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Progress {
    task: u64,
}

impl Progress {
    pub fn begin(title: &str) -> Result<Self, Error> {
        Self::start(title, false)
    }

    pub fn cancellable(title: &str) -> Result<Self, Error> {
        Self::start(title, true)
    }

    pub fn resume(task: u64) -> Self {
        Self { task }
    }

    pub fn task(&self) -> u64 {
        self.task
    }

    pub fn report(&self, done: f32, message: &str) -> Result<(), Error> {
        Ok(progress::report(
            self.task,
            Some(done.clamp(0.0, 1.0)),
            Some(message),
        )?)
    }

    pub fn busy(&self, message: &str) -> Result<(), Error> {
        Ok(progress::report(self.task, None, Some(message))?)
    }

    pub fn cancelled(&self) -> bool {
        progress::cancelled(self.task)
    }

    pub fn end(self) {
        progress::end(self.task);
    }

    fn start(title: &str, cancellable: bool) -> Result<Self, Error> {
        let task = progress::begin(title, cancellable)?;

        Ok(Self { task })
    }
}
