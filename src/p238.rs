struct Solution;

// Given an integer array nums, return an array answer such that answer[i]
// is equal to the product of all the elements of nums except nums[i].
// The product of any prefix or suffix of nums is guaranteed to fit in a 32-bit integer.
//You must write an algorithm that runs in O(n) time and without using the division operation.
impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let len = nums.len();
        let mut l = Vec::with_capacity(len);
        l.resize(len, 0);
        l[0] = nums[0];

        let mut r = Vec::with_capacity(len);
        r.resize(len, 0);
        r[len - 1] = nums[len - 1];

        for i in 0..len {
            if i > 0 {
                l[i] = l[i - 1] * nums[i];
                r[len - i - 1] = r[len - i] * nums[len - i - 1];
            }
        }

        let mut res = Vec::with_capacity(len);
        res.resize(len, 0);

        for i in 0..len {
            let lhs = if i > 0 { l[i - 1] } else { 1 };
            let rhs = if i < len - 1 { r[i + 1] } else { 1 };
            res[i] = lhs * rhs;
        }

        res
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn case1() {
        let nums = vec![1, 2, 3, 4];
        let expected = vec![24, 12, 8, 6];
        assert_eq!(super::Solution::product_except_self(nums), expected);
    }

    #[test]
    fn case2() {
        let nums = vec![-1, 1, 0, -3, 3];
        let expected = vec![0, 0, 9, 0, 0];
        assert_eq!(super::Solution::product_except_self(nums), expected);
    }
}
