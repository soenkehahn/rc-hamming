fn main() {
    let _ = hamming_distance("AAA", "GGG");
}

/// Compute the hamming distance between two strings.
fn hamming_distance(a: &str, b: &str) -> u32 {
    if a.len() != b.len() {
        panic!("Strings are of different length");
    }

    let mut result = 0;
    for (a_char, b_char) in a.chars().zip(b.chars()) {
        if a_char != b_char {
            result += 1;
        }

    }
    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn empty_strands() {
        assert_eq!(hamming_distance("", ""), 0);
    }

    #[test]
    fn identical_strands() {
        assert_eq!(hamming_distance("A", "A"), 0);
    }

    #[test]
    fn long_identical_strands() {
        assert_eq!(hamming_distance("GGACTGA", "GGACTGA"), 0);
    }

    #[test]
    fn complete_distance_in_single_nucleotide_strands() {
        assert_eq!(hamming_distance("A", "G"), 1);
    }

    #[test]
    fn complete_distance_in_small_strands() {
        assert_eq!(hamming_distance("AG", "CT"), 2);
    }

    #[test]
    fn small_distance_in_small_strands() {
        assert_eq!(hamming_distance("AT", "CT"), 1);
    }

    #[test]
    #[should_panic]
    fn test_disallow_first_strand_longer() {
        hamming_distance("AATG", "AAA");
    }

    #[test]
    #[should_panic]
    fn test_disallow_second_strand_longer() {
        hamming_distance("ATA", "AGTG");
    }
}
