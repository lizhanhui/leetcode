struct Solution;

impl Solution {
    pub fn get_maximum_generated(n: i32) -> i32 {
        match n {
            0..=1 => {
                return n;
            }
            _ => {}
        }
        
        let mut gen = vec![0; n as usize + 1];
        gen[0] = 0;
        gen[1] = 1;

        for i in 2..=n as usize {
            if i % 2 == 0 {
                gen[i] = gen[i / 2];
            } else {
                gen[i] = gen[i / 2] + gen[i / 2 + 1];
            }
        }

        gen.into_iter().max().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_maximum_generated() {
        assert_eq!(Solution::get_maximum_generated(7), 3);
        assert_eq!(Solution::get_maximum_generated(2), 1);
        assert_eq!(Solution::get_maximum_generated(3), 2);
    }
}