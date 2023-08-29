struct Solution;

impl Solution {
    pub fn first_missing_positive(mut nums: Vec<i32>) -> i32 {
        let len = nums.len();
        for i in 0..len {
            let n = nums[i];
            if n == i as i32 + 1 {
                continue;
            }

            if n <= 0 || n > len as i32 {
                continue;
            }

            // Swap element i and n - 1
            nums[i] = nums[(n - 1) as usize];
            nums[(n - 1) as usize] = n;
            let m = nums[i];
            if m <= i as i32 + 1 && m > 0 {
                nums[m as usize - 1] = m;
            }
        }

        for i in 0..len {
            if nums[i] != (i + 1) as i32 {
                return i as i32 + 1;
            }
        }
        len as i32 + 1
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_case1() {
        let nums = vec![1, 2, 0];
        let result = 3;
        assert_eq!(super::Solution::first_missing_positive(nums), result);
    }

    #[test]
    fn test_case2() {
        let nums = vec![3, 4, -1, 1];
        let result = 2;
        assert_eq!(super::Solution::first_missing_positive(nums), result);
    }

    #[test]
    fn test_case3() {
        let nums = vec![7, 8, 9, 11, 12];
        let result = 1;
        assert_eq!(super::Solution::first_missing_positive(nums), result);
    }

    #[test]
    fn test_wa1() {
        let nums = vec![1];
        let result = 2;
        assert_eq!(super::Solution::first_missing_positive(nums), result);
    }

    #[test]
    fn test_wa2() {
        let nums = vec![1, 2, 6, 3, 5, 4];
        let result = 7;
        assert_eq!(super::Solution::first_missing_positive(nums), result);
    }

    #[test]
    fn test_wa3() {
        let nums = vec![
            99, 94, 96, 11, 92, 5, 91, 89, 57, 85, 66, 63, 84, 81, 79, 61, 74, 78, 77, 30, 64, 13,
            58, 18, 70, 69, 51, 12, 32, 34, 9, 43, 39, 8, 1, 38, 49, 27, 21, 45, 47, 44, 53, 52,
            48, 19, 50, 59, 3, 40, 31, 82, 23, 56, 37, 41, 16, 28, 22, 33, 65, 42, 54, 20, 29, 25,
            10, 26, 4, 60, 67, 83, 62, 71, 24, 35, 72, 55, 75, 0, 2, 46, 15, 80, 6, 36, 14, 73, 76,
            86, 88, 7, 17, 87, 68, 90, 95, 93, 97, 98,
        ];
        let result = 100;
        assert_eq!(super::Solution::first_missing_positive(nums), result);
    }
}
