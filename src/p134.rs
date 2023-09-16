struct Solution;

impl Solution {
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        if cost.iter().sum::<i32>() > gas.iter().sum() {
            return -1;
        }

        debug_assert_eq!(gas.len(), cost.len());
        let len = gas.len();

        for i in 0..len {
            let mut t_g = 0;
            let mut t_c = 0;
            let mut fail = false;
            for j in 0..len {
                let idx = (i + j) % len;
                t_g += gas[idx];
                t_c += cost[idx];
                if t_c > t_g || t_g == 0 {
                    fail = true;
                    break;
                }
            }

            if !fail {
                return i as i32;
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn case1() {
        let gas = vec![1, 2, 3, 4, 5];
        let cost = vec![3, 4, 5, 1, 2];
        let expected = 3;
        assert_eq!(super::Solution::can_complete_circuit(gas, cost), expected);
    }

    #[test]
    fn case2() {
        let gas = vec![2, 3, 4];
        let cost = vec![3, 4, 3];
        let expected = -1;
        assert_eq!(super::Solution::can_complete_circuit(gas, cost), expected);
    }
}
