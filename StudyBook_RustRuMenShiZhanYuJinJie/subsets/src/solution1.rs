impl Solution {
  pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
    if nums.len() == 0 {
      return Vec::new();
    }

    let mut vecs: Vec<Vec<i32>> = Vec::new();
    let mut vec: Vec<i32> = Vec::new();
    Solution::backtrack(&mut vecs, &mut vec, &nums, 0);
    vecs
  }

  fn backtrack(vecs: &mut Vec<Vec<i32>>, vec: &mut Vec<i32>, nums: &Vec<i32>, start: usize) {
    // 将路径记入结果集
    vecs.push(vec.clone());

    for i in start..nums.len() {
      // 做选择
      vec.push(nums[i]);

      Solution::backtrack(vecs, vec, nums, i + 1);

      // 撤销选择，将该选择重新加入选择列表
      vec.remove(vec.len() - 1);
    }
  }
}

pub struct Solution;

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test() {}
}
