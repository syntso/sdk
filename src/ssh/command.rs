use std::time::Duration;

use serde::Serialize;

use super::Output;
use crate::error::Error;
use crate::host::call;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Command {
    command: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    timeout_seconds: Option<u64>,
}

impl Command {
    pub fn new(command: impl Into<String>) -> Self {
        Self {
            command: command.into(),
            timeout_seconds: None,
        }
    }

    #[must_use]
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout_seconds = Some(timeout.as_secs().max(1));
        self
    }

    pub fn run(&self, host: &str) -> Result<Output, Error> {
        call("ssh.exec", Some(host), self)
    }
}

pub fn exec(host: &str, command: &str) -> Result<Output, Error> {
    Command::new(command).run(host)
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn sends_the_params_ssh_exec_takes() {
        let plain = serde_json::to_value(Command::new("uptime")).unwrap();
        let timed =
            serde_json::to_value(Command::new("make").timeout(Duration::from_secs(90))).unwrap();

        assert_eq!(plain, json!({ "command": "uptime" }));
        assert_eq!(timed, json!({ "command": "make", "timeout_seconds": 90 }));
    }
}
