#![allow(dead_code)]

/// An implementation of [Two Sum](https://leetcode.com/problems/two-sum/description/) problem.
pub mod two_sum {

    pub struct Solution;

    impl Solution {
        pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
            let mut map = std::collections::HashMap::<i32, usize>::new();
            let mut first = 0_usize;
            let mut second = 0_usize;

            for (i, v) in nums.iter().enumerate() {
                let remain = target - *v;

                if let Some(index) = map.get(&remain) {
                    first = i;
                    second = *index;
                } else {
                    map.insert(*v, i);
                }
            }

            vec![first as i32, second as i32]
        }
    }
}

/// An implementation of [Valid Parentheses](https://leetcode.com/problems/valid-parentheses/description/) problem.
pub mod valid_parentheses {

    pub struct Solution;

    impl Solution {
        pub fn is_valid(s: String) -> bool {
            let mut stack = Vec::<char>::new();
            for c in s.chars() {
                match c {
                    ')' => {
                        if stack.last().is_some_and(|c| *c == '(') {
                            stack.pop();
                        } else {
                            stack.push(c);
                        }
                    }
                    ']' => {
                        if stack.last().is_some_and(|c| *c == '[') {
                            stack.pop();
                        } else {
                            stack.push(c);
                        }
                    }
                    '}' => {
                        if stack.last().is_some_and(|c| *c == '{') {
                            stack.pop();
                        } else {
                            stack.push(c);
                        }
                    }
                    _ => stack.push(c),
                }
            }

            stack.is_empty()
        }
    }
}

#[cfg(test)]
mod tests {

    fn check(nums: Vec<i32>, target: i32) {
        use super::two_sum::Solution;

        let res = Solution::two_sum(nums.clone(), target);

        let err_msg = format!("with input nums:{:?} and taget:{}", nums, target);
        assert_eq!(
            target,
            &nums[res[0] as usize] + &nums[res[1] as usize],
            "{}",
            &err_msg
        );
        assert!(res[0] != res[1], "{}", &err_msg);
    }

    #[test]
    fn two_sum() {
        check(vec![2, 7, 11, 15], 9);
        check(vec![3, 2, 4], 6);
        check(vec![3, 3], 6);
    }

    #[test]
    fn valid_parentheses() {
        use super::valid_parentheses::Solution;

        assert!(Solution::is_valid("[]".into()));
        assert!(Solution::is_valid("()".into()));
        assert!(Solution::is_valid("{}".into()));

        assert!(Solution::is_valid("()[]{}".into()));
        assert!(Solution::is_valid("({[]})".into()));

        assert!(!Solution::is_valid("(]".into()));
        assert!(!Solution::is_valid("]".into()));
        assert!(!Solution::is_valid("]()".into()));
        assert!(!Solution::is_valid("[])".into()));
        assert!(!Solution::is_valid("(])".into()));
    }
}
