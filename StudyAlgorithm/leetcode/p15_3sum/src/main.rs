use std::collections::HashMap;

fn main() {
    let vec = vec![-1, 0, 1, 2, -1, -4];
    println!("Hello, world!, {:?}", three_sum(vec));
}

pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut nums_map = HashMap::new();
    for (_i, x) in nums.iter().enumerate() {
        let count = nums_map.entry(x).or_insert(0);
        *count += 1;
    }

    let mut ret_map = HashMap::new();
    let mut ret = Vec::new();

    let mut i = 0;
    while i < nums.len() {
        let icount = nums_map.entry(&nums[i]).or_insert(0);
        *icount -= 1;

        let mut j = i + 1;
        while j < nums.len() {
            let r = -(nums[i] + nums[j]);

            let jcount = nums_map.entry(&nums[j]).or_insert(0);
            *jcount -= 1;

            if let Some(curcount) = nums_map.get(&r) {
                if *curcount != 0 {
                    let mut cur = Vec::new();
                    cur.push(nums[i]);
                    cur.push(nums[j]);
                    cur.push(r);
                    cur.sort();
                    ret_map.insert(cur, 1);
                }
            }

            let jcount = nums_map.entry(&nums[j]).or_insert(0);
            *jcount += 1;

            j = j + 1;
        }

        let icount = nums_map.entry(&nums[i]).or_insert(0);
        *icount += 1;
        i = i + 1;
    }

    for (key, _value) in ret_map {
        ret.push(key)
    }

    ret
}
