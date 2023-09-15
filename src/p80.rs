struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.len() <= 2 {
            return nums.len() as i32;
        }

        let mut i = 0;
        let mut j = 1;
        let mut occurance = 1;
        loop {
            if j >= nums.len() {
                break;
            }

            if nums[j] == nums[i as usize] {
                occurance += 1;
                if occurance <= 2 {
                    i += 1;

                    if j > i {
                        nums.swap(i, j);
                    }
                }
            } else {
                occurance = 1;
                i += 1;
                nums.swap(i, j);
            }

            j += 1;
        }

        i as i32 + 1
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn case1() {
        let mut nums = vec![1, 1, 1, 2, 2, 3];
        let expected = 5;
        let actual = super::Solution::remove_duplicates(&mut nums);
        assert_eq!(expected, actual);
        println!("{:?}", &nums[..expected as usize]);
    }
}
