pub fn annotate(minefield: &[&str]) -> Vec<String> {
  (0..minefield.len())
    .map(|row| {
      (0..minefield[row].len())
        .map(|col| match minefield[row].as_bytes()[col] {
          b'*' => '*',
          _ => match (row.saturating_sub(1)..=row + 1)
            .flat_map(|row| (col.saturating_sub(1)..=col + 1).map(move |col| (row, col)))
            .filter(|&(row, col)| row < minefield.len() && col < minefield[row].len())
            .filter(|&(row, col)| minefield[row].as_bytes()[col] == b'*')
            .count()
          {
            0 => ' ',
            n => std::char::from_digit(n as u32, 10).unwrap(),
          },
        })
        .collect()
    })
    .collect()
}
