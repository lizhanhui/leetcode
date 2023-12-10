struct Solution;

impl Solution {
    pub fn four_sum(mut nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut res = vec![];
        if nums.len() < 4 {
            return res;
        }

        nums.sort_unstable();

        let mut i = 0;
        let mut j = 0;
        while i < nums.len() - 3 {
            j = i + 1;
            while j < nums.len() - 2 {
                let mut l = j + 1;
                let mut r = nums.len() - 1;
                while l < r {
                    let sum = nums[i] + nums[j] + nums[l] + nums[r];
                    if sum == target {
                        res.push(vec![nums[i], nums[j], nums[l], nums[r]]);
                        while l < r && nums[l] == nums[l + 1] {
                            l += 1;
                        }
                        while l < r && nums[r] == nums[r - 1] {
                            r -= 1;
                        }
                        l += 1;
                        r -= 1;
                    } else if sum < target {
                        l += 1;
                    } else {
                        r -= 1;
                    }
                }
                while j < nums.len() - 2 && nums[j] == nums[j + 1] {
                    j += 1;
                }
            }

            while i < nums.len() - 3 && nums[i] == nums[i + 1] {
                i += 1;
            }
        }
        res
    }
}
