#![allow(dead_code)]

/// An implementation of [two sum](https://leetcode.com/problems/two-sum/description/) problem.
pub mod two_sum {
    
    pub struct Solution;

    impl Solution {
        pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
            let mut first = 0_usize;
            let mut second = 0_usize;
            for (i, v) in nums.iter().enumerate() {
                let remain = target - *v;
                for (k, v) in nums.iter().enumerate().skip(i + 1) {
                    if remain == *v {
                        first = i;
                        second = k;
                        break;
                    }
                }
            }

            vec![first as i32, second as i32]
        }
    }
}

#[cfg(test)]
mod tests {

    use super::two_sum::Solution;

    fn check(nums: Vec<i32>, target: i32, indecies: Vec<i32>) {
        let res = Solution::two_sum(nums.clone(), target);

        let err_msg = format!("with input nums:{:?} and taget:{}", nums, target);
        assert_eq!(target, &nums[res[0] as usize] + &nums[res[1] as usize], "{}", &err_msg);
        assert_eq!(indecies[0], res[0], "{}", &err_msg);
        assert_eq!(indecies[1], res[1], "{}", &err_msg);
    }

    #[test]
    fn two_sum() {
        check(vec![2, 7, 11, 15], 9, vec![0, 1]);
        check(vec![3, 2, 4], 6, vec![1, 2]);
        check(vec![3, 3], 6, vec![0, 1]);
    }
}
