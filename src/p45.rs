struct Solution;

impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return 0;
        }

        let mut steps = 0;
        let mut i = 0;
        loop {
            steps += 1;

            let max_step_width = nums[i] as usize;
            if i + max_step_width >= nums.len() - 1 {
                break;
            }
            let from = i + 1;
            let to = i + nums[i] as usize;
            if let Some((idx, n)) =
                &nums[from..=to]
                    .iter()
                    .enumerate()
                    .max_by(|(idx_m, m), (idx_n, n)| {
                        let l = *idx_m + (**m as usize);
                        let r = *idx_n + (**n as usize);
                        l.cmp(&r)
                    })
            {
                i += *idx + 1;
            }
        }
        steps
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn case1() {
        let nums = vec![2, 3, 1, 1, 4];
        let expected = 2;
        assert_eq!(expected, super::Solution::jump(nums));
    }

    #[test]
    fn case2() {
        let nums = vec![2, 3, 0, 1, 4];
        let expected = 2;
        assert_eq!(expected, super::Solution::jump(nums));
    }

    #[test]
    fn wa() {
        let nums = vec![0];
        let expected = 0;
        assert_eq!(expected, super::Solution::jump(nums));
    }
}
