# nom

## resources

- <https://github.com/rust-bakery/nom>
- <https://tfpk.github.io/nominomicon/introduction.html>

## parsers

- `tag`
- `tag_no_case`
- `alpha0`: Recognizes zero or more lowercase and uppercase alphabetic characters: `/[a-zA-Z]/`.
- `alpha1` does the same but returns at least one character
- `alphanumeric0`: Recognizes zero or more numerical and alphabetic characters: `/[0-9a-zA-Z]/`.
- `alphanumeric1` does the same but returns at least one character
- `digit0`: Recognizes zero or more numerical characters: `/[0-9]/`.
- `digit1` does the same but returns at least one character
- `multispace0`: Recognizes zero or more spaces, tabs, carriage returns and line feeds.
- `multispace1` does the same but returns at least one character
- `space0`: Recognizes zero or more spaces and tabs.
- `space1` does the same but returns at least one character
- `line_ending`: Recognizes an end of line (both \n and \r\n)
- `newline`: Matches a newline character \n
- `tab`: Matches a tab character \t

