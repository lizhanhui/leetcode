/// Given an integer array nums, rotate the array to the right by k steps,
/// where k is non-negative.
struct Solution;

impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let k = k as usize % nums.len();
        if 0 == k {
            return;
        }

        Self::invert(nums, 0, nums.len() - 1);
        Self::invert(nums, 0, k - 1);
        Self::invert(nums, k, nums.len() - 1);
    }

    fn invert(nums: &mut Vec<i32>, mut start: usize, mut end: usize) {
        loop {
            if start >= end {
                break;
            }

            nums.swap(start, end);
            start += 1;
            end -= 1;
        }
    }

    fn rotate_by_one(nums: &mut Vec<i32>) {
        let last = *nums.last().unwrap();
        let src = nums.as_mut_ptr();
        let dst = unsafe { src.add(1) };
        unsafe { std::ptr::copy(src, dst, nums.len() - 1) };
        nums[0] = last;
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn case1() {
        let mut nums = vec![1, 2, 3, 4, 5, 6, 7];
        let k = 3;
        super::Solution::rotate(&mut nums, k);
        let expect = vec![5, 6, 7, 1, 2, 3, 4];
        assert_eq!(expect, nums);
    }

    #[test]
    fn case2() {
        let mut nums = vec![-1, -100, 3, 99];
        let k = 2;
        super::Solution::rotate(&mut nums, k);
        let expect = vec![3, 99, -1, -100];
        assert_eq!(expect, nums);
    }
}
