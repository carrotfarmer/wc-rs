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
            if !filename.exists() {
                println!("No such file: {:?}", filename);
                continue;
            }

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wc() {
        let filenames = vec![std::path::PathBuf::from("test.txt")];
        let wc = Wc::new(filenames, true, true, true, true, true);

        assert_eq!(format!("{}", wc), "\t49\t301\t1462\t1462\t43test.txt\n\t");
    }

    #[test]
    fn test_wc_multiple_files() {
        let filenames = vec![
            std::path::PathBuf::from("test.txt"),
            std::path::PathBuf::from("test2.txt"),
        ];
        let wc = Wc::new(filenames, true, true, true, true, true);

        assert_eq!(
            format!("{}", wc),
            "\t49\t301\t1462\t1462\t43test.txt\n\t55\t286\t1392\t1392\t44test2.txt\n\t104\t587\t2854\t2854\t44total"
        );
    }

    #[test]
    fn test_wc_nonexistent_file() {
        let filenames = vec![std::path::PathBuf::from("nonexistent.txt")];

        let wc = Wc::new(filenames, true, true, true, true, true).to_string();
        assert_eq!(wc, "\t");
    }

    #[test]
    fn test_wc_multiple_files_with_nonexistent_file() {
        let filenames = vec![
            std::path::PathBuf::from("test.txt"),
            std::path::PathBuf::from("nonexistent.txt"),
        ];

        let wc = Wc::new(filenames, true, true, true, true, true).to_string();
        assert_eq!(
            wc,
            "\t49\t301\t1462\t1462\t43test.txt\n\t49\t301\t1462\t1462\t43total"
        );
    }

    #[test]
    fn test_wc_lines() {
        let filenames = vec![std::path::PathBuf::from("test.txt")];

        let wc = Wc::new(filenames, true, false, false, false, false).to_string();
        assert_eq!(wc, "\t301\ttest.txt\n\t");
    }

    #[test]
    fn test_wc_words() {
        let filenames = vec![std::path::PathBuf::from("test.txt")];

        let wc = Wc::new(filenames, false, true, false, false, false).to_string();
        assert_eq!(wc, "\t49\ttest.txt\n\t");
    }

    #[test]
    fn test_wc_bytes() {
        let filenames = vec![std::path::PathBuf::from("test.txt")];

        let wc = Wc::new(filenames, false, false, true, false, false).to_string();
        assert_eq!(wc, "\t1462\ttest.txt\n\t");
    }

    #[test]
    fn test_wc_chars() {
        let filenames = vec![std::path::PathBuf::from("test.txt")];

        let wc = Wc::new(filenames, false, false, false, true, false).to_string();
        assert_eq!(wc, "\t1462\ttest.txt\n\t");
    }

    #[test]
    fn test_wc_max_line_length() {
        let filenames = vec![std::path::PathBuf::from("test.txt")];

        let wc = Wc::new(filenames, false, false, false, false, true).to_string();
        assert_eq!(wc, "\t43test.txt\n\t");
    }

    #[test]
    fn test_wc_multiple_flags() {
        let filenames = vec![std::path::PathBuf::from("test.txt")];

        let wc = Wc::new(filenames, true, true, false, false, false).to_string();
        assert_eq!(wc, "\t49\t301\ttest.txt\n\t");
    }
}
