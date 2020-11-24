#![deny(clippy::pedantic)]
use std::{char::from_digit, convert::TryFrom};

pub fn annotate(minefield: &[&str]) -> Vec<String> {
  (0..minefield.len())
    .map(|y| {
      (0..minefield[y].len())
        .map(|x| match minefield[y].as_bytes()[x] {
          b'*' => '*',
          _ => match (x.saturating_sub(1)..=x + 1)
            .flat_map(|x| (y.saturating_sub(1)..=y + 1).map(move |y| (x, y)))
            .filter_map(|(x, y)| minefield.get(y)?.as_bytes().get(x))
            .filter(|&&x| x == b'*')
            .count()
          {
            0 => ' ',
            x => from_digit(u32::try_from(x).unwrap(), 10).unwrap(),
          },
        })
        .collect()
    })
    .collect()
}
