use std::fmt::Display;
use std::fs;

use crate::count::*;

#[derive(Debug)]
pub struct Wc {
    filenames: Vec<std::path::PathBuf>,
    words: bool,
    lines: bool,
    bytes: bool,
    chars: bool,
    max_line_length: bool,
}

impl Wc {
    pub fn new(
        filenames: Vec<std::path::PathBuf>,
        words: bool,
        lines: bool,
        bytes: bool,
        chars: bool,
        max_line_length: bool,
    ) -> Self {
        Self {
            filenames,
            words,
            lines,
            bytes,
            chars,
            max_line_length,
        }
    }
}

impl Display for Wc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\t")?;

        // for max_line_len flag
        let mut longest = String::from("");

        let mut total_wc = 0;
        let mut total_lines = 0;
        let mut total_bytes = 0;
        let mut total_chars = 0;

        for filename in &self.filenames {
            let contents = fs::read_to_string(&filename).unwrap();

            if self.lines {
                total_lines += count_lines(&contents);
                write!(f, "{}\t", count_lines(&contents))?;
            }

            if self.words {
                total_wc += count_words(&contents);
                write!(f, "{}\t", count_words(&contents))?;
            }

            if self.bytes {
                total_bytes += count_bytes(&contents);
                write!(f, "{}\t", count_bytes(&contents))?;
            }

            if self.chars {
                total_chars += count_chars(&contents);
                write!(f, "{}\t", count_chars(&contents))?;
            }

            if self.max_line_length {
                if max_line_len(&contents) > max_line_len(&longest) {
                    longest = String::from(&contents);
                }

                write!(f, "{}", max_line_len(&contents))?;
            }

            write!(f, "{}\n\t", &filename.display())?
        }

        if self.filenames.len() > 1 {
            if self.lines {
                write!(f, "{}\t", total_lines)?
            }

            if self.words {
                write!(f, "{}\t", total_wc)?
            }

            if self.bytes {
                write!(f, "{}\t", total_bytes)?
            }

            if self.chars {
                write!(f, "{}\t", total_chars)?
            }

            if self.max_line_length {
                write!(f, "{}", max_line_len(&longest))?
            }

            write!(f, "total")?
        }

        Ok(())
    }
}
