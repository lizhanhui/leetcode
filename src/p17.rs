struct Solution;

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        if digits.is_empty() {
            return vec![];
        }
        let mut m = std::collections::HashMap::new();
        m.insert('2', vec!['a', 'b', 'c']);
        m.insert('3', vec!['d', 'e', 'f']);
        m.insert('4', vec!['g', 'h', 'i']);
        m.insert('5', vec!['j', 'k', 'l']);
        m.insert('6', vec!['m', 'n', 'o']);
        m.insert('7', vec!['p', 'q', 'r', 's']);
        m.insert('8', vec!['t', 'u', 'v']);
        m.insert('9', vec!['w', 'x', 'y', 'z']);
        let c = digits.chars().collect::<Vec<_>>();
        let mut idx = vec![0; c.len()];
        let mut res = vec![];
        loop {
            let mut s = String::new();
            idx.iter().zip(c.iter()).for_each(|(i, e)| {
                let dic = m.get(e).unwrap();
                s.push(dic[*i]);
            });

            res.push(s);

            if Self::next(&mut idx, &c, &m) {
                break;
            }
        }

        res
    }

    fn next(idx: &mut [usize], c: &[char], m: &std::collections::HashMap<char, Vec<char>>) -> bool {
        let mut carry = true;
        idx.iter_mut().zip(c.iter()).for_each(|(i, e)| {
            if carry {
                *i += 1;
                carry = false;
            }

            if *i + 1 > m.get(e).unwrap().len() {
                *i = 0;
                carry = true;
            }
        });

        idx.iter().all(|i| *i == 0)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn case1() {
        let digits = "23";
        let ans = vec!["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"];
        println!(
            "{:?}",
            super::Solution::letter_combinations(digits.to_string())
        );
    }
}
