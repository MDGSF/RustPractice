impl Solution {
  pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
    let mut board = vec![vec!['.'; n as usize]; n as usize];
    let mut result = vec![];
    Solution::backtrack(&mut board, &mut result, n, 0);
    result
  }

  fn backtrack(board: &mut Vec<Vec<char>>, result: &mut Vec<Vec<String>>, n: i32, row: i32) {
    for column in 0..n {
      if !Solution::collision(&board, n, row, column) {
        board[row as usize][column as usize] = 'Q';

        if row == n - 1 {
          result.push(board.iter().map(|vec| vec.iter().collect()).collect());
        } else {
          Solution::backtrack(board, result, n, row + 1);
        }

        board[row as usize][column as usize] = '.';
      }
    }
  }

  fn collision(board: &Vec<Vec<char>>, n: i32, row: i32, column: i32) -> bool {
    let mut up_row = row - 1;
    let mut left_column = column - 1;
    let mut right_column = column + 1;

    while up_row >= 0 {
      if board[up_row as usize][column as usize] == 'Q' {
        return true;
      }

      if left_column >= 0 && board[up_row as usize][left_column as usize] == 'Q' {
        return true;
      }

      if right_column < n && board[up_row as usize][right_column as usize] == 'Q' {
        return true;
      }

      up_row -= 1;
      left_column -= 1;
      right_column += 1;
    }

    false
  }
}

pub struct Solution;
