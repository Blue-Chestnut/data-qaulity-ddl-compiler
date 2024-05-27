use std::io::BufRead;

#[derive(Debug, PartialEq)]
pub struct Line {
    pub start: i32,
    pub end: i32,
    pub content: String,
}

impl Line {
    // pub fn from_file(path: &str) -> Vec<Line> {
    //     let file = File::open(path);
    //
    //     if file.is_err() {
    //         panic!("Could not open file {}", path);
    //     }
    //
    //     let reader = BufReader::new(file.unwrap());
    //     let mut lines = Vec::new();
    //     let mut start = 0;
    //     let mut end = 0;
    //
    //     for line in reader.lines() {
    //         if line.is_err() {
    //             panic!("Could not read line");
    //         }
    //
    //         let line = line.unwrap();
    //         end += line.len() as i32;
    //         lines.push(Line { start, end, content: line });
    //
    //         start = end + 1;
    //     }
    //
    //     lines
    //
    // }

    pub fn from_string(content: String) -> Vec<Line> {
        let mut lines = Vec::new();
        let mut start = 0;
        let mut end = 0;

        for line in content.lines() {
            end += line.len() as i32;
            lines.push(Line { start, end, content: line.to_string() });

            start = end + 1;
        }

        lines
    }
}


#[cfg(test)]
mod tests {
    use std::fs::read_to_string;
    use super::*;

    #[test]
    fn test_lines_from_str() {
        let contents = read_to_string("./test_data/test_lines.sqlx").unwrap();
        let desired_lines = vec![
            Line { start: 0, end: 33, content: "Create table if not exists Test {".to_string() },
            Line { start: 34, end: 53, content: "    Id Varchar(10) {".to_string() },
            Line { start: 54, end: 69, content: "        -unique}".to_string() },
            Line { start: 70, end: 101, content: "    Price FLOAT(3,8) PRIMARY KEY".to_string() },
            Line { start: 102, end: 103, content: "};".to_string() },
        ];

        let lines = Line::from_string(contents);

        assert_eq!(lines, desired_lines);
    }

    #[test]
    fn test_lines_from_string_with_escape() {
        let contents = read_to_string("./test_data/test_lines_escaped_new_line.sqlx").unwrap();
        let desired_lines = vec![
            Line { start: 0, end: 46, content: "Create table if not exists Test \"example \\n\" {".to_string() },
            Line { start: 47, end: 48, content: "};".to_string() },
        ];

        let lines = Line::from_string(contents);

        assert_eq!(lines, desired_lines);
    }

}