struct Solution;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        Self::fast(&height)
    }

    fn fast(height: &[i32]) -> i32 {
        let mut l = 0;
        let mut r = height.len() - 1;
        let mut max = 0;
        loop {
            if l >= r {
                break;
            }

            let h = height[l].min(height[r]);
            if h * (r - l) as i32 > max {
                max = h * (r - l) as i32;
            }

            if height[l] <= height[r] {
                l += 1;
            } else {
                r -= 1;
            }
        }

        max
    }

    fn slow(height: &[i32]) -> i32 {
        let mut max = 0;
        let mut i = 0;
        let mut j = 0;

        let len = height.len();

        let mut l = 0;
        let mut r = 0;

        loop {
            if i >= len - 1 {
                break;
            }

            if i > l && height[i] <= height[l] {
                i += 1;
                continue;
            }

            j = i + 1;
            loop {
                if j >= len {
                    break;
                }

                if j < r && height[j] <= height[r] {
                    j += 1;
                    continue;
                }

                let h = height[i].min(height[j]);
                let d = (j - i) as i32;
                if h * d > max {
                    max = h * d;
                    l = i;
                    r = j;
                }
                j += 1;
            }
            i += 1;
        }
        max
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn case1() {
        let height = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
        let expected = 49;
        assert_eq!(expected, super::Solution::max_area(height));
    }

    #[test]
    fn case2() {
        let height = vec![1, 1];
        let expected = 1;
        assert_eq!(expected, super::Solution::max_area(height));
    }
}
