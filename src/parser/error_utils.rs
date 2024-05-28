use crate::parser::lines::Line;
use lalrpop_util::lexer::Token;

pub fn gen_unknown_token_error_message(location: usize, lines: Vec<Line>) -> String {
    let line_number = Line::get_line(location as u32, &lines);
    let current_line = lines.get(line_number).unwrap();
    // create empty string with spaces to align the squiggle line to wrong token
    // 23 is the length of "InvalidToken in line" and the line number buffer is for
    // adjustments of the line number, e.g. "30" -> 2, "2" -> 1, "100" -> 3
    let line_number_buffer_len = line_number.to_string().len();
    let pre_line =
        " ".repeat(location - (current_line.start as usize) + 23 + line_number_buffer_len);

    format!(
        "InvalidToken in line {}: {}, \n{}~~",
        line_number + 1,
        current_line.content,
        pre_line
    )
}

pub fn gen_syntax_error_message(
    token: (usize, Token, usize),
    lines: Vec<Line>,
    expected: Vec<String>,
) -> String {
    let line_number = Line::get_line(token.2 as u32, &lines);
    let current_line = lines.get(line_number).unwrap();
    let mapped_expected = expected
        .iter()
        .map(|x| x[1..x.len() - 1].to_owned())
        .collect::<Vec<String>>();

    let pre_line = " ".repeat(token.0 - (current_line.start as usize));
    let squiggle_line = "~".repeat(token.2 - token.0);

    let line_number_buffer = " ".repeat(line_number.to_string().len());

    format!(
        "SyntaxError in line {}: Unrecognized token {:?} expected {:?}\n\
                \t\tline {}: {}\n\t\t        {}{}{}",
        line_number + 1,
        token.1 .1,
        mapped_expected,
        line_number + 1,
        current_line.content,
        line_number_buffer,
        pre_line,
        squiggle_line
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case((0, Token(12, "test"), 1), vec![Line {start: 0, end: 22, content: ". create table test{}".to_owned()}],
    vec!["\"  \"".to_string()],
    "SyntaxError in line 1: Unrecognized token \"test\" expected [\"  \"]\n\t\tline 1: . create table test{}\n\t\t         ~")]
    #[case((41, Token(4, "."), 42), vec![
        Line {start: 0, end: 24, content: "create table test case{".to_owned()},
        Line {start: 25, end: 49, content: "    Quantity INT(.3)".to_owned()},
        Line {start: 50, end: 53, content: "};".to_owned()}
        ],
    vec!["\"  \"".to_string()],
    "SyntaxError in line 2: Unrecognized token \".\" expected [\"  \"]\n\t\tline 2:     Quantity INT(.3)\n\t\t                         ~")]
    fn gen_syntax_error_message_test(
        #[case] token: (usize, Token, usize),
        #[case] lines: Vec<Line>,
        #[case] expected: Vec<String>,
        #[case] expected_msg: &str,
    ) {
        assert_eq!(
            gen_syntax_error_message(token, lines, expected),
            expected_msg
        );
    }

    #[rstest]
    #[case(18, vec![Line {start: 0, end: 25, content: "create table test case{}".to_owned()}],
    "InvalidToken in line 1: create table test case{}, \n                                          ~~")]
    #[case(31, vec![
        Line {start: 0, end: 24, content: "create table test case{".to_owned()},
        Line {start: 25, end: 49, content: "    Quantity INT(hello)".to_owned()},
        Line {start: 50, end: 53, content: "};".to_owned()}
        ],
    "InvalidToken in line 2:     Quantity INT(hello), \n                              ~~")]
    fn gen_invalid_token_error_message_test(
        #[case] location: usize,
        #[case] lines: Vec<Line>,
        #[case] expected_msg: &str,
    ) {
        assert_eq!(
            gen_unknown_token_error_message(location, lines),
            expected_msg
        );
    }
}
