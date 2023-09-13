struct Solution;

impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        for i in 0..nums.len() - 1 {
            if nums[i] > 0 {
                continue;
            }

            let mut may_skip = false;
            for j in (0..i).rev() {
                if nums[j] as usize > i - j {
                    may_skip = true;
                    break;
                }
            }
            if !may_skip {
                return false;
            }
        }
        true
    }

    fn backtracking(nums: &Vec<i32>) -> bool {
        // Backtracking
        let mut stack = std::collections::VecDeque::new();
        stack.push_back(0);
        loop {
            if stack.is_empty() {
                break;
            }

            if let Some(index) = stack.pop_front() {
                let opts = nums[index] as usize;
                if index + opts >= nums.len() - 1 {
                    return true;
                }
                for i in 1..=opts {
                    let dst = index + i;
                    if !stack.contains(&dst) {
                        stack.push_back(dst);
                    }
                }
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn case1() {
        let nums = vec![2, 3, 1, 1, 4];
        assert_eq!(true, super::Solution::can_jump(nums));
    }

    #[test]
    fn case2() {
        let nums = vec![3, 2, 1, 0, 4];
        assert_eq!(false, super::Solution::can_jump(nums));
    }

    #[test]
    fn memory_limit() {
        let nums = vec![
            8, 2, 4, 4, 4, 9, 5, 2, 5, 8, 8, 0, 8, 6, 9, 1, 1, 6, 3, 5, 1, 2, 6, 6, 0, 4, 8, 6, 0,
            3, 2, 8, 7, 6, 5, 1, 7, 0, 3, 4, 8, 3, 5, 9, 0, 4, 0, 1, 0, 5, 9, 2, 0, 7, 0, 2, 1, 0,
            8, 2, 5, 1, 2, 3, 9, 7, 4, 7, 0, 0, 1, 8, 5, 6, 7, 5, 1, 9, 9, 3, 5, 0, 7, 5,
        ];
        super::Solution::can_jump(nums);
    }
}
