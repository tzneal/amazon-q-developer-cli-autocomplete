pub use crate::proto::figterm::*;

impl InsertTextRequest {
    pub fn to_term_string(&self) -> String {
        let mut out = String::new();

        match &self.offset.map(|i| i.signum()) {
            Some(1) => out.extend(std::iter::repeat_n(
                "\x1b[C",
                self.offset.unwrap_or(0).unsigned_abs() as usize,
            )),
            Some(-1) => out.extend(std::iter::repeat_n(
                "\x1b[D",
                self.offset.unwrap_or(0).unsigned_abs() as usize,
            )),
            _ => {},
        }

        out.extend(std::iter::repeat_n('\x08', self.deletion.unwrap_or(0) as usize));

        if let Some(insertion) = &self.insertion {
            out.push_str(insertion);
        }

        if self.immediate == Some(true) {
            out.push('\r');
        }

        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_term_string() {
        assert_eq!(
            InsertTextRequest {
                deletion: Some(3),
                insertion: Some("hello".to_string()),
                offset: Some(2),
                ..Default::default()
            }
            .to_term_string(),
            "\u{1b}[C\u{1b}[C\u{08}\u{08}\u{08}hello"
        );
    }
}
