use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug)]
pub struct SubscriberName(String);

impl SubscriberName {
    pub fn parse(s: String) -> Result<Self, String> {
        let is_empty_or_whitespace = s.trim().is_empty();

        let is_too_long = s.graphemes(true).count() > 256;

        let forbidden_characters = ['/', '(', ')', '"', '<', '>', '\\', '&', '{', '}', '[', ']', '='];

        let has_forbidden_characters = s.chars().any(|c| forbidden_characters.contains(&c));

        if is_empty_or_whitespace || is_too_long || has_forbidden_characters {
            return Err(format!("Invalid subscriber name: {}", s));
        }

        Ok(Self(s))
    }
}

impl AsRef<str> for SubscriberName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

pub struct NewSubscriber {
    pub email: String,
    pub name: SubscriberName,
}

#[cfg(test)]
mod tests {
    use super::*;
    use claims::{assert_err, assert_ok};

    #[test]
    fn a_256_grapheme_long_name_is_valid() {
        let name = "a".repeat(256);
        assert_ok!(SubscriberName::parse(name));
    }

    #[test]
    fn a_257_grapheme_long_name_is_invalid() {
        let name = "a".repeat(257);
        assert_err!(SubscriberName::parse(name));
    }

    #[test]
    fn whitespace_only_name_is_invalid() {
        let name = " ".to_string();
        assert_err!(SubscriberName::parse(name));
    }

    #[test]
    fn empty_string_is_invalid() {
        let name = "".to_string();
        assert_err!(SubscriberName::parse(name));
    }

    #[test]
    fn name_with_forbidden_characters_is_invalid() {
        for &char in &['/', '(', ')', '"', '<', '>', '\\', '&', '{', '}', '[', ']', '='] {
            let name = format!("valid{}name", char);
            assert_err!(SubscriberName::parse(name));
        }
    }

    #[test]
    fn name_with_valid_characters_is_valid() {
        let name = "Valid Name".to_string();
        assert_ok!(SubscriberName::parse(name));
    }

    #[test]
    fn name_with_graphemes_is_valid() {
        let name = "å".repeat(256);
        assert_ok!(SubscriberName::parse(name));
    }

    #[test]
    fn name_with_too_many_graphemes_is_invalid() {
        let name = "å".repeat(257);
        assert_err!(SubscriberName::parse(name));
    }
}