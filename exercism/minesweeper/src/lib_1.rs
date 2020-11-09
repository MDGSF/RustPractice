pub fn annotate(minefield: &[&str]) -> Vec<String> {
  let rows = minefield.len();
  if rows == 0 {
    return Vec::new();
  }
  let cols = minefield[0].len();

  let directions = vec![
    vec![-1, -1],
    vec![-1, 0],
    vec![-1, 1],
    vec![0, -1],
    vec![0, 1],
    vec![1, -1],
    vec![1, 0],
    vec![1, 1],
  ];

  let mut result = Vec::new();
  for row in 0..rows {
    let mut row_result = String::new();
    for col in 0..cols {
      if minefield[row].as_bytes()[col] == b'*' {
        row_result.push_str("*");
        continue;
      }

      let old_row = row as isize;
      let old_col = col as isize;
      let mut mine_counter = 0;
      for direction in directions.iter() {
        let new_row = old_row + direction[0];
        let new_col = old_col + direction[1];
        if new_row >= 0 && new_row < rows as isize && new_col >= 0 && new_col < cols as isize {
          let new_row = new_row as usize;
          let new_col = new_col as usize;
          if minefield[new_row].as_bytes()[new_col] == b'*' {
            mine_counter += 1;
          }
        }
      }
      if mine_counter > 0 {
        row_result.push_str(&mine_counter.to_string());
      } else {
        row_result.push_str(" ");
      }
    }
    result.push(row_result);
  }

  result
}
