pub fn check_special(word: &str) -> bool {
    is_dna_sequence(word)
}

fn is_dna_sequence(s: &str) -> bool {
    if s.len() < 4 {
        return false;
    }
    for c in s.chars() {
        match c {
            'A' | 'T' | 'C' | 'G' | 'a' | 't' | 'c' | 'g' => {
                continue;
            }
            _ => return false,
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dna_sequence() {
        assert!(is_dna_sequence("ATCGATCG"));
        assert!(is_dna_sequence("ATCG"));
        assert!(is_dna_sequence("atcgatcg"));
        assert!(!is_dna_sequence("xyzATCGAbc"));
        assert!(!is_dna_sequence("Hello"));
        assert!(!is_dna_sequence("ATC"));
    }
}
