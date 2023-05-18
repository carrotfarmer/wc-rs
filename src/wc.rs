use std::fmt::Display;
use std::fs;

use crate::colors::{colorize, Color};
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
            if !filename.exists() {
                println!(
                    "{}",
                    colorize(&format!("No such file: {:?}", filename), Color::Red)
                );
                continue;
            }

            let contents = fs::read_to_string(&filename).unwrap();

            if self.lines {
                total_lines += count_lines(&contents);
                write!(
                    f,
                    "{}\t",
                    colorize(&count_lines(&contents).to_string(), Color::Blue)
                )?;
            }

            if self.words {
                total_wc += count_words(&contents);
                write!(
                    f,
                    "{}\t",
                    colorize(&count_words(&contents).to_string(), Color::Blue)
                )?;
            }

            if self.bytes {
                total_bytes += count_bytes(&contents);
                write!(
                    f,
                    "{}\t",
                    colorize(&count_bytes(&contents).to_string(), Color::Blue)
                )?;
            }

            if self.chars {
                total_chars += count_chars(&contents);
                write!(
                    f,
                    "{}\t",
                    colorize(&count_chars(&contents).to_string(), Color::Blue)
                )?;
            }

            if self.max_line_length {
                if max_line_len(&contents) > max_line_len(&longest) {
                    longest = String::from(&contents);
                }

                write!(
                    f,
                    "{}\t",
                    colorize(&count_chars(&contents).to_string(), Color::Blue)
                )?;
            }

            write!(
                f,
                "{}\n\t",
                colorize(&filename.display().to_string(), Color::GreenBold),
            )?
        }

        if self.filenames.len() > 1 {
            if self.lines {
                write!(f, "{}\t", colorize(total_lines, Color::Yellow))?
            }

            if self.words {
                write!(f, "{}\t", colorize(total_wc, Color::Yellow))?
            }

            if self.bytes {
                write!(f, "{}\t", colorize(total_bytes, Color::Yellow))?
            }

            if self.chars {
                write!(f, "{}\t", colorize(total_chars, Color::Yellow))?
            }

            if self.max_line_length {
                write!(f, "{}\t", colorize(max_line_len(&longest), Color::Yellow))?
            }

            write!(f, "{}\t", colorize("total", Color::MagentaBold))?
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn remove_color_codes(text: &str) -> String {
        text.replace("\x1b[31m", "")
            .replace("\x1b[32m", "")
            .replace("\x1b[1;32m", "")
            .replace("\x1b[34m", "")
            .replace("\x1b[1;35m", "")
            .replace("\x1b[33m", "")
            .replace("\x1b[0m", "")
    }

    #[test]
    fn test_wc() {
        let filenames = vec![std::path::PathBuf::from("test.txt")];
        let wc = Wc::new(filenames, true, true, true, true, true);

        let output = format!("{}", wc);
        let output_without_color = remove_color_codes(&output);

        assert_eq!(
            output_without_color,
            "\t49\t301\t1462\t1462\t1462\ttest.txt\n\t"
        );
    }

    #[test]
    fn test_wc_multiple_files() {
        let filenames = vec![
            std::path::PathBuf::from("test.txt"),
            std::path::PathBuf::from("test2.txt"),
        ];
        let wc = Wc::new(filenames, true, true, true, true, true);

        let output = format!("{}", wc);
        let output_without_color = remove_color_codes(&output);

        assert_eq!(
            output_without_color,
            "\t49\t301\t1462\t1462\t1462\ttest.txt\n\t55\t286\t1392\t1392\t1392\ttest2.txt\n\t104\t587\t2854\t2854\t44\ttotal\t"
        );
    }

    #[test]
    fn test_wc_nonexistent_file() {
        let filenames = vec![std::path::PathBuf::from("nonexistent.txt")];

        let wc = Wc::new(filenames, true, true, true, true, true).to_string();
        let output_without_color = remove_color_codes(&wc);

        assert_eq!(output_without_color, "\t");
    }

    #[test]
    fn test_wc_multiple_files_with_nonexistent_file() {
        let filenames = vec![
            std::path::PathBuf::from("test.txt"),
            std::path::PathBuf::from("nonexistent.txt"),
        ];

        let wc = Wc::new(filenames, true, true, true, true, true).to_string();
        let output_without_color = remove_color_codes(&wc);

        assert_eq!(
            output_without_color,
            "\t49\t301\t1462\t1462\t1462\ttest.txt\n\t49\t301\t1462\t1462\t43\ttotal\t"
        );
    }

    #[test]
    fn test_wc_lines() {
        let filenames = vec![std::path::PathBuf::from("test.txt")];

        let wc = Wc::new(filenames, true, false, false, false, false).to_string();
        let output_without_color = remove_color_codes(&wc);

        assert_eq!(output_without_color, "\t301\ttest.txt\n\t");
    }

    #[test]
    fn test_wc_words() {
        let filenames = vec![std::path::PathBuf::from("test.txt")];

        let wc = Wc::new(filenames, false, true, false, false, false).to_string();
        let output_without_color = remove_color_codes(&wc);

        assert_eq!(output_without_color, "\t49\ttest.txt\n\t");
    }

    #[test]
    fn test_wc_bytes() {
        let filenames = vec![std::path::PathBuf::from("test.txt")];

        let wc = Wc::new(filenames, false, false, true, false, false).to_string();
        let output_without_color = remove_color_codes(&wc);

        assert_eq!(output_without_color, "\t1462\ttest.txt\n\t");
    }

    #[test]
    fn test_wc_chars() {
        let filenames = vec![std::path::PathBuf::from("test.txt")];

        let wc = Wc::new(filenames, false, false, false, true, false).to_string();
        let output_without_color = remove_color_codes(&wc);

        assert_eq!(output_without_color, "\t1462\ttest.txt\n\t");
    }

    #[test]
    fn test_wc_max_line_length() {
        let filenames = vec![std::path::PathBuf::from("test.txt")];

        let wc = Wc::new(filenames, false, false, false, false, true).to_string();
        let output_without_color = remove_color_codes(&wc);

        assert_eq!(output_without_color, "\t1462\ttest.txt\n\t");
    }

    #[test]
    fn test_wc_multiple_flags() {
        let filenames = vec![std::path::PathBuf::from("test.txt")];

        let wc = Wc::new(filenames, true, true, false, false, false).to_string();
        let output_without_color = remove_color_codes(&wc);

        assert_eq!(output_without_color, "\t49\t301\ttest.txt\n\t");
    }
}
