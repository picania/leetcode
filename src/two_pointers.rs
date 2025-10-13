#![allow(dead_code)]

// [125. Valid Palindrome](https://leetcode.com/problems/valid-palindrome)
mod p125 {

    pub struct Solution;

    impl Solution {
        pub fn is_palindrome(s: &str) -> bool {
            let s = s
                .as_bytes()
                .iter()
                .filter(|c| c.is_ascii_alphanumeric())
                .map(|c| c.to_ascii_lowercase())
                .collect::<Vec<_>>();

            if s.is_empty() {
                return true;
            }

            let mut i = 0;
            let mut j = s.len() - 1;

            while i < j {
                if s[i] != s[j] {
                    return false;
                }
                i += 1;
                j -= 1;
            }

            true
        }
    }

    #[cfg(test)]
    mod test {
        use crate::two_pointers::p125::Solution;

        #[test]
        fn example_i() {
            assert!(Solution::is_palindrome("A man, a plan, a canal: Panama"));
        }

        #[test]
        fn example_ii() {
            assert!(!Solution::is_palindrome("raceacar"));
        }

        #[test]
        fn example_iii() {
            assert!(Solution::is_palindrome(" "));
        }

        #[test]
        fn example_iv() {
            assert!(!Solution::is_palindrome("0P"));
        }
    }
}
