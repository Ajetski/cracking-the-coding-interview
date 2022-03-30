fn is_unique(s: &str) -> bool {
    let mut visited = [false; 256];
    for c in s.chars() {
        if visited[c as usize] {
            return false;
        }
        visited[c as usize] = true;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::is_unique;

    #[test]
    fn problem_1_1() {
        assert!(is_unique(""));
        assert!(is_unique("world"));
        assert!(is_unique("fancy"));

        assert!(!is_unique("test"));
        assert!(!is_unique("people"));
        assert!(!is_unique("capable"));
    }
}
