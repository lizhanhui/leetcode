struct Solution;

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        debug_assert_eq!(nums1.len(), (m + n) as usize, "Input data are invalid");
        let mut i = m - 1;
        let mut j = n - 1;
        let mut k = m + n - 1;
        loop {
            if j < 0 {
                break;
            }

            if i < 0 {
                nums1[k as usize] = nums2[j as usize];
                j -= 1;
                k -= 1;
                continue;
            }

            let v_i = nums1[i as usize];
            let v_j = nums2[j as usize];
            let dst = &mut nums1[k as usize];
            if v_j >= v_i {
                *dst = v_j;
                j -= 1;
            } else {
                *dst = v_i;
                i -= 1;
            }
            k -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn case1() {
        let mut nums1 = vec![1, 2, 3, 0, 0, 0];
        let m = 3;
        let mut nums2 = vec![2, 5, 6];
        let n = 3;
        let expected = vec![1, 2, 2, 3, 5, 6];
        super::Solution::merge(&mut nums1, m, &mut nums2, n);
        assert_eq!(expected, nums1);
    }

    #[test]
    fn case2() {
        let mut nums1 = vec![1];
        let m = 1;
        let mut nums2 = vec![];
        let n = 0;
        let expected = vec![1];
        super::Solution::merge(&mut nums1, m, &mut nums2, n);
        assert_eq!(nums1, expected);
    }
}
