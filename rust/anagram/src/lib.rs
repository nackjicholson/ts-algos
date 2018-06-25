use std::collections::HashMap;

pub fn are_anagrams(s1: &str, s2: &str) -> bool {
    let mut char_map: HashMap<char, i32> = HashMap::new();
    for ch in s1.chars() {
        let count = char_map.entry(ch).or_insert(0);
        *count += 1;
    }
    for ch in s2.chars() {
        if !char_map.contains_key(&ch) {
            return false;
        } else {
            let count = char_map.get_mut(&ch).unwrap();
            *count -= 1;
        }
    }
    char_map.iter().all(|(_, count)| *count == 0 as i32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_detects_anagrams() {
        assert!(are_anagrams("earth", "heart"));
    }

    #[test]
    fn it_detects_non_anagrams() {
        assert!(are_anagrams("foo", "bar") == false);
    }
}

