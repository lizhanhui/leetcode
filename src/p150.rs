struct Solution;

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let len = height.len();
        let mut m_l = Vec::with_capacity(len);
        m_l.resize(len, 0);
        m_l[0] = height[0];

        let mut m_r = Vec::with_capacity(len);
        m_r.resize(len, 0);
        m_r[len - 1] = height[len - 1];

        for i in 1..len {
            m_l[i] = m_l[i - 1].max(height[i]);
            m_r[len - i - 1] = m_r[len - i].max(height[len - i - 1]);
        }
        let mut s = 0;
        for i in 0..len {
            s += m_l[i].min(m_r[i]) - height[i];
        }
        s
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn case1() {
        let height = vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1];
        let expected = 6;
        assert_eq!(super::Solution::trap(height), expected);
    }

    #[test]
    fn case2() {
        let height = vec![4, 2, 0, 3, 2, 5];
        let expected = 9;
        assert_eq!(super::Solution::trap(height), expected);
    }
}
