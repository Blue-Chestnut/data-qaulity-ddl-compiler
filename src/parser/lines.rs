#[derive(Debug, PartialEq)]
pub struct Line {
    pub start: u32,
    pub end: u32,
    pub content: String,
}

impl Line {
    pub fn from_string(content: String) -> Vec<Line> {
        let mut lines = Vec::new();
        let mut start = 0;
        let mut end: u32 = 0;

        for line in content.lines() {
            end += line.len() as u32 + 2;
            lines.push(Line {
                start,
                end,
                content: line.to_string(),
            });

            start = end + 1;
        }

        lines
    }

    pub fn get_line(char_number: u32, lines: &Vec<Line>) -> usize {
        if char_number > lines.last().unwrap().end {
            panic!("Invalid char number. Number must be smaller than the last char");
        }

        let mut line_number: usize = 0;

        for line in lines {
            if line.start <= char_number && line.end >= char_number {
                return line_number;
            }

            line_number += 1;
        }

        line_number
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_lines_from_str() {
        let contents = read_to_string("./test_data/test_lines.sqlx").unwrap();
        let desired_lines = vec![
            Line {
                start: 0,
                end: 35,
                content: "Create table if not exists Test {".to_string(),
            },
            Line {
                start: 36,
                end: 57,
                content: "    Id Varchar(10) {".to_string(),
            },
            Line {
                start: 58,
                end: 75,
                content: "        -unique}".to_string(),
            },
            Line {
                start: 76,
                end: 109,
                content: "    Price FLOAT(3,8) PRIMARY KEY".to_string(),
            },
            Line {
                start: 110,
                end: 113,
                content: "};".to_string(),
            },
        ];

        let lines = Line::from_string(contents);

        assert_eq!(lines, desired_lines);
    }

    #[test]
    fn test_lines_from_string_with_escape() {
        let contents = read_to_string("./test_data/test_lines_escaped_new_line.sqlx").unwrap();
        let desired_lines = vec![
            Line {
                start: 0,
                end: 48,
                content: "Create table if not exists Test \"example \\n\" {".to_string(),
            },
            Line {
                start: 49,
                end: 52,
                content: "};".to_string(),
            },
        ];

        let lines = Line::from_string(contents);

        assert_eq!(lines, desired_lines);
    }

    #[test]
    fn test_get_line() {
        let lines = vec![
            Line {
                start: 0,
                end: 33,
                content: "Create table if not exists Test {".to_string(),
            },
            Line {
                start: 34,
                end: 53,
                content: "    Id Varchar(10) {".to_string(),
            },
            Line {
                start: 54,
                end: 69,
                content: "        -unique}".to_string(),
            },
            Line {
                start: 70,
                end: 101,
                content: "    Price FLOAT(3,8) PRIMARY KEY".to_string(),
            },
            Line {
                start: 102,
                end: 103,
                content: "};".to_string(),
            },
        ];

        let line_number = Line::get_line(0, &lines);
        assert_eq!(line_number, 0);
        let line_number = Line::get_line(33, &lines);
        assert_eq!(line_number, 0);
        let line_number = Line::get_line(40, &lines);
        assert_eq!(line_number, 1);
    }
}
