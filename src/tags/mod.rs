const TAG_WILDCARD: char = '*';

/// Matches a tag against a pattern.
/// The pattern can contain '*' as a wildcard to match any number.
pub fn match_tag(tag: u32, pattern: &str) -> bool {
    let tag = tag.to_string();
    let mut tag_chars = tag.chars();
    let mut pattern_chars = pattern.chars();

    loop {
        match (tag_chars.next(), pattern_chars.next()) {
            (Some(tc), Some(pc)) => {
                if tc != pc && pc != TAG_WILDCARD {
                    return false;
                }
            }
            (Some(_), None) => return false,
            (None, Some(_)) => return false,
            (None, None) => return true,
        }
    }
}

#[cfg(test)]
mod test_match_tag {
    use super::*;

    #[test]
    fn test_exact_match() {
        assert!(match_tag(1234, "1234"));
    }

    #[test]
    fn test_exact_match_wrong() {
        assert!(!match_tag(1234, "0234"));
        assert!(!match_tag(1234, "1034"));
        assert!(!match_tag(1234, "1204"));
        assert!(!match_tag(1234, "1230"));
        assert!(!match_tag(1234, "1004"));
    }

    #[test]
    fn test_wildcard() {
        assert!(match_tag(1234, "*234"));
        assert!(match_tag(1234, "123*"));
        assert!(match_tag(1234, "1*34"));
        assert!(match_tag(1234, "12*4"));
        assert!(match_tag(1234, "1**4"));
        assert!(match_tag(1234, "*23*"));
    }

    #[test]
    fn test_shorter_tag() {
        assert!(!match_tag(123, "123*"));
        assert!(!match_tag(234, "*234"));
    }

    #[test]
    fn test_longer_tag() {
        assert!(!match_tag(1234, "12*"));
        assert!(!match_tag(1234, "*23"));
    }
}
