struct Solution;

impl Solution {
    pub fn hammingWeight(n: u32) -> i32 {
        let mut m = n;
        let mut cnt = 0;
        loop {
            if m & 0x1 == 0x1 {
                cnt += 1;
            }
            m = m >> 1;
            if 0 == m {
                break;
            }
        }
        cnt
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn case1() {
        let n = 0b00000000000000000000000000001011;
        assert_eq!(3, super::Solution::hammingWeight(n));
    }
    #[test]
    fn case2() {
        let n = 0b00000000000000000000000010000000;
        assert_eq!(1, super::Solution::hammingWeight(n));
    }
    #[test]
    fn case3() {
        let n = 0b11111111111111111111111111111101;
        assert_eq!(31, super::Solution::hammingWeight(n));
    }
}
