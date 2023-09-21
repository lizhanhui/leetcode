use core::num;

struct Solution;

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if 1 == num_rows {
            return s;
        }

        let mut res = String::new();
        let chars = s.chars().collect::<Vec<_>>();
        let period = 2 * num_rows as usize - 2;
        for row in 0..num_rows {
            if row == 0 || row == num_rows - 1 {
                let mut j = row as usize;
                loop {
                    if j >= chars.len() {
                        break;
                    }
                    res.push(chars[j]);
                    j += period;
                }
            } else {
                let mut j = row as usize;
                loop {
                    if j >= chars.len() {
                        break;
                    }
                    res.push(chars[j]);
                    let idx = (j + period) / period * period - j % period;
                    if idx < chars.len() {
                        res.push(chars[idx]);
                    }
                    j += period;
                }
            }
        }

        res
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn case1() {
        let s = "PAYPALISHIRING";
        let num_rows = 3;
        let expected = "PAHNAPLSIIGYIR";
        assert_eq!(expected, &super::Solution::convert(s.to_string(), num_rows));
    }

    #[test]
    fn case2() {
        let s = "PAYPALISHIRING";
        let num_rows = 4;
        let expected = "PINALSIGYAHRPI";
        assert_eq!(expected, &super::Solution::convert(s.to_string(), num_rows));
    }

    #[test]
    fn case3() {
        let s = "A";
        let num_rows = 1;
        let expected = "A";
        assert_eq!(expected, &super::Solution::convert(s.to_string(), num_rows));
    }
}
