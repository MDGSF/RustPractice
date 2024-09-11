# nom

## resources

- <https://github.com/rust-bakery/nom>
- <https://tfpk.github.io/nominomicon/introduction.html>

## parsers

- `nom::sequence`: <https://docs.rs/nom/latest/nom/sequence/index.html>
  - delimited
  - preceded
  - terminated
  - pair
  - separated_pair
  - tuple
- `nom::bytes::complete`: <https://docs.rs/nom/latest/nom/bytes/complete/index.html>
  - tag
  - tag_no_case
  - take
  - take_xxx
  - escaped
  - is_a
  - is_not
- `nom::character::complete`: <https://docs.rs/nom/latest/nom/character/complete/index.html>
  - alpha0
  - alpha1

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

**Alternatives:**

```rust
use nom::branch::alt;
```

The `alt()` combinator will execute each parser in a tuple until it finds one
that does not error. If all error, then by default you are given the error
from the last error.

**Composition:**

```rust
use nom::sequence::tuple;
```

The `tuple()` combinator takes a tuple of parsers, and either returns Ok
with a tuple of all of their successful parses, or it returns the Err of
the first failed parser.


**delimited:**

Matches an object from the first parser and discards it, then gets an object
from the second parser, and finally matches an object from the third parser
and discards it.

```rust
delimited(char('('), take(2), char(')'))
```

- input: `"(ab)cd"`
- output: `Ok(("cd", "ab"))`

**preceded:**

Matches an object from the first parser and discards it, then gets an object
from the second parser.

```rust
preceded(tag("ab"), tag("XY"))
```

- input: `"abXYZ"`
- output: `Ok(("Z", "XY"))`

**terminated:**

Gets an object from the first parser, then matches an object from the second
parser and discards it.

```rust
terminated(tag("ab"), tag("XY"))
```

- input: `"abXYZ"`
- output: `Ok(("Z", "ab"))`

**pair:**

Gets an object from the first parser, then gets another object from
the second parser.

```rust
pair(tag("ab"), tag("XY"))
```

- input: `"abXYZ"`
- output: `Ok(("Z", ("ab", "XY")))`

**separated_pair:**

Gets an object from the first parser, then matches an object from the
sep_parser and discards it, then gets another object from the second parser.

```rust
separated_pair(tag("hello"), char(','), tag("world"))
```

- input: `"hello,world!"`
- output: `Ok(("!", ("hello", "world")))`

