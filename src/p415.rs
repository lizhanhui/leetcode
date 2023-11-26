struct Solution;

impl Solution {
    pub fn add_strings(num1: String, num2: String) -> String {
        let mut carriage = 0;
        let mut ans = String::new();
        let mut i = 0;
        let num1 = num1.chars().collect::<Vec<char>>();
        let num2 = num2.chars().collect::<Vec<char>>();

        let n1 = num1.len();
        let n2 = num2.len();
        let mut buffer = std::collections::VecDeque::new();

        loop {
            let l = if i < n1 {
                match num1.get(n1 - 1 - i) {
                    Some(c) => *c as u32 - '0' as u32,
                    None => {
                        unreachable!()
                    }
                }
            } else {
                0
            };
            let r = if i < n2 {
                match num2.get(n2 - 1 - i) {
                    Some(c) => *c as u32 - '0' as u32,
                    None => {
                        unreachable!()
                    }
                }
            } else {
                0
            };

            if l + r + carriage == 0 {
                break;
            }
            let c = unsafe { char::from_u32_unchecked('0' as u32 + (l + r + carriage) % 10) };
            carriage = (l + r + carriage) / 10;
            i += 1;
            buffer.push_front(c);
        }
        while let Some(c) = buffer.pop_front() {
            ans.push(c);
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn case1() {
        let num1 = "11";
        let num2 = "123";
        assert_eq!(
            &super::Solution::add_strings(num1.to_owned(), num2.to_owned()),
            "134"
        );
    }
}
