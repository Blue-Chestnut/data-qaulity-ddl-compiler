use crate::parser::lines::Line;
use lalrpop_util::lexer::Token;

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
    #[case((0, Token(12, "test"), 1), vec![Line {start: 0, end: 7, content: ". create table test{}".to_owned()}],
    vec!["\"  \"".to_string()],
    "SyntaxError in line 1: Unrecognized token \"test\" expected [\"  \"]\n\t\tline 1: . create table test{}\n\t\t         ~")]
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
}
