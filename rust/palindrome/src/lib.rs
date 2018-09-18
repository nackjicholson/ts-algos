use std::collections::HashSet;

pub fn is_palindrome(phrase: &str) -> bool {
    let s1: String = phrase
        .to_lowercase()
        .chars()
        .filter(|c| c.is_alphabetic())
        .collect();
    let s2: String = phrase
        .to_lowercase()
        .chars()
        .rev()
        .filter(|c| c.is_alphabetic())
        .collect();
    s1 == s2
}

pub fn is_any_permutation_palindrome(phrase: &str) -> bool {
    let mut set = HashSet::new();

    for c in phrase.to_lowercase().chars() {
        if set.contains(&c) {
            set.remove(&c);
        } else {
            set.insert(c);
        };
    };

    set.len() <= 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_returns_true_for_palindromes() {
        assert!(is_palindrome("racecar"));
        assert!(is_palindrome("Racecar"));
        assert!(is_palindrome("  A man, a plan, a canal. Panama!    "));
    }

    #[test]
    fn it_returns_false_for_non_palindromes() {
        assert!(!is_palindrome("foo"));
    }

    #[test]
    fn it_returns_true_if_any_permutation_is_palindrome() {
        assert!(is_any_permutation_palindrome("racecar"));
        assert!(is_any_permutation_palindrome("Racecar"));
        assert!(is_any_permutation_palindrome("A man a plan a canal Panama"));
        assert!(is_any_permutation_palindrome("foo"));
        assert!(is_any_permutation_palindrome("civic"));
        assert!(is_any_permutation_palindrome("vicic"));
        assert!(is_any_permutation_palindrome("toot"));
        assert!(is_any_permutation_palindrome("toto"));
    }

    #[test]
    fn it_returns_false_if_no_permutation_is_palindrome() {
        assert!(!is_any_permutation_palindrome("civil"));
    }
}
