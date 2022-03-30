fn palindrome_permutation(s: &str) -> bool {
    let mut counts = std::collections::HashMap::<char, u32>::new();
    for c in s.chars() {
        let entry = counts.entry(c).or_insert(0);
        *entry += 1;
    }

    let mut hit_middle = false;
    for (_, val) in counts {
        if val % 2 != 0 {
            if hit_middle {
                return  false;
            }
            hit_middle = true;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::palindrome_permutation;

    #[test]
    fn problem_1_4() {
        assert!(palindrome_permutation("abab"));
        assert!(palindrome_permutation("fbaedcabcdef"));
        assert!(palindrome_permutation("asdfasdfw"));

        assert!(!palindrome_permutation("mnmmasdfm"));
        assert!(!palindrome_permutation("hello"));
        assert!(!palindrome_permutation("decode me"));
    }
}
