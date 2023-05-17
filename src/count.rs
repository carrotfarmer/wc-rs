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
