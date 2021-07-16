impl Solution {
  pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
    if n < k {
      return Vec::new();
    }

    let mut vecs: Vec<Vec<i32>> = Vec::new();
    let mut vec: Vec<i32> = Vec::new();
    Solution::backtrack(&mut vecs, &mut vec, n, k, 1);
    vecs
  }

  fn backtrack(vecs: &mut Vec<Vec<i32>>, vec: &mut Vec<i32>, n: i32, k: i32, start: usize) {
    if vec.len() == k as usize {
      vecs.push(vec.clone());
      return;
    }

    for i in start..=((n - (k - vec.len() as i32) + 1) as usize) {
      // 做选择
      vec.push(i as i32);

      // 将该选择从列表移除后递归
      Solution::backtrack(vecs, vec, n, k, i + 1);

      // 撤销选择
      vec.remove(vec.len() - 1);
    }
  }
}

pub struct Solution;
