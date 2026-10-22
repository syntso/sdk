use crate::wit::synt::extension::terminal;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Output {
    pub bytes: Vec<u8>,
    pub next: u64,
    pub ended: bool,
}

impl Output {
    pub fn text(&self) -> String {
        String::from_utf8_lossy(&self.bytes).into_owned()
    }
}

impl From<terminal::Output> for Output {
    fn from(output: terminal::Output) -> Self {
        Self {
            bytes: output.bytes,
            next: output.next,
            ended: output.ended,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_the_output_as_text() {
        let output = Output::from(terminal::Output {
            bytes: b"ready\r\n".to_vec(),
            next: 7,
            ended: false,
        });

        assert_eq!(output.text(), "ready\r\n");
        assert_eq!(output.next, 7);
        assert!(!output.ended);
    }
}
