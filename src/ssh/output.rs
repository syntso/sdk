use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct Output {
    pub exit: Option<u32>,
    pub signal: Option<String>,
    pub stdout: String,
    pub stderr: String,
    pub truncated: bool,
    pub duration_ms: u64,
}

impl Output {
    pub fn succeeded(&self) -> bool {
        self.exit == Some(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_what_ssh_exec_returns() {
        let output: Output = serde_json::from_str(
            r#"{"exit":0,"signal":null,"stdout":"up 3 days\n","stderr":"","truncated":false,
                "duration_ms":12,"kex":"curve25519-sha256","cipher":"chacha20-poly1305"}"#,
        )
        .unwrap();

        assert!(output.succeeded());
        assert_eq!(output.stdout, "up 3 days\n");
        assert_eq!(output.duration_ms, 12);
    }
}
