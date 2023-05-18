pub fn count_words(contents: &str) -> u32 {
    let mut count = 0;

    for _ in contents.split_whitespace() {
        count += 1;
    }

    count
}

pub fn count_lines(contents: &str) -> u32 {
    let mut lines = 0;

    for _ in contents.lines() {
        lines += 1;
    }

    lines
}

pub fn count_bytes(contents: &str) -> u32 {
    let mut count = 0;

    for _ in contents.as_bytes() {
        count += 1;
    }

    count
}

pub fn count_chars(contents: &str) -> u32 {
    contents.len() as u32
}

pub fn max_line_len(contents: &str) -> u32 {
    let mut longest_line = "";

    for line in contents.lines() {
        if longest_line.len() < line.len() {
            longest_line = line;
        }
    }

    longest_line.len() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_words() {
        assert_eq!(count_words("hello world"), 2);
        assert_eq!(count_words("hello world\n\n\n\n"), 2);
        assert_eq!(count_words("some random text"), 3);
        assert_eq!(
            count_words(
                "
            lorem ipsum dolor sit amet
            consectetur adipiscing elit
            sed do eiusmod tempor incididunt ut labore et dolore magna aliqua
        "
            ),
            19
        );
    }

    #[test]
    fn test_count_lines() {
        assert_eq!(count_lines("hello world"), 1);
        assert_eq!(
            count_lines(
                "lorem ipsum dolor sit amet
                consectetur adipiscing elit
                sed do eiusmod tempor incididunt ut labore et dolore magna aliqua"
            ),
            3
        );

        assert_eq!(
            count_lines(
                "lorem ipsum dolor sit amet
                consectetur adipiscing elit
                sed do eiusmod tempor incididunt ut labore et dolore magna aliqua\n\n\n\n\n"
            ),
            7
        );
    }

    #[test]
    fn test_count_bytes() {
        assert_eq!(count_bytes("hello world"), 11);
        assert_eq!(count_bytes("hello world\n"), 12);
        assert_eq!(
            count_bytes(
                "
            lorem ipsum dolor sit amet
            consectetur adipiscing elit
            sed do eiusmod tempor incididunt ut labore et dolore magna aliqua
            "
            ),
            170
        );
    }

    #[test]
    fn test_count_chars() {
        assert_eq!(count_chars("hello world"), 11);
        assert_eq!(count_chars("hello world\n"), 12);
        assert_eq!(
            count_chars(
                "
            lorem ipsum dolor sit amet
            consectetur adipiscing elit
            sed do eiusmod tempor incididunt ut labore et dolore magna aliqua
            "
            ),
            170
        );
    }

    #[test]
    fn test_max_line_len() {
        assert_eq!(max_line_len("hello world\n\n\n"), 11);
        assert_eq!(
            max_line_len(
                "
            lorem ipsum dolor sit amet
            consectetur adipiscing elit
            sed do eiusmod tempor incididunt ut labore et dolore magna aliqua
            "
            ),
            77
        );
    }
}
