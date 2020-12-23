static NEIGBOURHOOD_OFFSETS: &'static [(i32, i32)] = &[
  (-1, -1),
  (-1, 0),
  (-1, 1),
  (0, -1),
  (0, 1),
  (1, -1),
  (1, 0),
  (1, 1),
];

pub fn annotate(minefield: &[&str]) -> Vec<String> {
  let rows = minefield.len() as i32;
  if rows == 0 {
    return Vec::new();
  }
  let cols = minefield[0].len() as i32;

  (0..rows)
    .map(|row| {
      (0..cols)
        .map(|col| {
          if minefield[row as usize].as_bytes()[col as usize] == b'*' {
            '*'
          } else {
            match NEIGBOURHOOD_OFFSETS
              .iter()
              .map(|&(offset_row, offset_col)| (row + offset_row, col + offset_col))
              .filter(|&(row, col)| (0 <= row && row < rows) && (0 <= col && col < cols))
              .filter(|&(row, col)| minefield[row as usize].as_bytes()[col as usize] == b'*')
              .count()
            {
              0 => ' ',
              n => (n as u8 + '0' as u8) as char,
            }
          }
        })
        .collect()
    })
    .collect()
}
