fn check_permutation(a: &str, b: &str) -> bool {
    type CharCounts = std::collections::HashMap<char, u32>;

    if a.len() != b.len() {
        return false;
    }

    let mut a_counts = CharCounts::new();
    for c in a.chars() {
        let entry = a_counts.entry(c).or_insert(0);
        *entry += 1;
    }

    let mut b_counts = CharCounts::new();
    for c in b.chars() {
        let entry = b_counts.entry(c).or_insert(0);
        *entry += 1;
    }

    for (key, value) in a_counts {
        if value != b_counts[&key] {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::check_permutation;

    #[test]
    fn problem_1_2() {
        assert!(check_permutation("test", "sett"));
        assert!(check_permutation("make", "keam"));

        assert!(!check_permutation("test", "sets"));
        assert!(!check_permutation("negative", "false"));
    }
}
