# nom

- <https://docs.rs/nom/latest/nom/>
- <https://github.com/rust-bakery/nom/blob/main/doc/choosing_a_combinator.md>
- <https://github.com/rust-bakery/nom/blob/main/doc/making_a_new_parser_from_scratch.md>
- <https://github.com/rust-bakery/nom/blob/main/doc/nom_recipes.md>
- <https://github.com/rust-bakery/nom/blob/main/doc/error_management.md>

## nom error

```rust

use nom::Err;
use nom::IResult;
use nom::error::Error;
use nom::error::ErrorKind;

pub type nom::IResult<I, O, E = nom::error::Error<I>> = core::Result<(I, O), nom::Err<E>>;

pub enum core::Result<T, E> {
    Ok(T),
    Err(E),
}

pub struct nom::error::Error<I> {
  /// position of the error in the input data
  pub input: I,
  /// nom error code
  pub code: ErrorKind,
}

pub enum nom::Err<Failure, Error = Failure> {
  /// There was not enough data
  Incomplete(Needed),
  /// The parser had an error (recoverable)
  Error(Error),
  /// The parser had an unrecoverable error: we got to the right
  /// branch and we know other branches won't work, so backtrack
  /// as fast as possible
  Failure(Failure),
}


IResult<&str, char>
// nom::IResult<&str, char, E = nom::error::Error<I>>
// core::Result<(&str, char), nom::Err<E>>;
// -> Ok((I, O))
// -> Ok((&str, char))
// -> Ok(("a", 'b'))
// -> Err(Err::Error(Error::new(" abc", ErrorKind::Char)))
// -> core::Result::Err(nom::Err::Error(nom::error::Error::new(" abc", ErrorKind::Char)))
```
