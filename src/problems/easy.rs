#![allow(dead_code)]

mod two_sum {
    /// Two Sum
    /// =======
    /// 
    /// A [link](https://leetcode.com/problems/two-sum/description/).
    
    pub struct Solution;

    impl Solution {
        pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {

            vec![1, 2]
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn two_sum() {
        use super::two_sum::Solution;

        let nums = vec![2, 7, 11, 15];
        let target = 9;

        let res = Solution::two_sum(nums.clone(), target);

        assert_eq!(target, &nums[res[0] as usize] + &nums[res[1] as usize]);
    }
}
