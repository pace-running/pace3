use rand::Rng;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PaymentReference {
    value: String,
}

impl PaymentReference {
    fn new(value: String) -> anyhow::Result<Self> {
        if !value.starts_with("LGR-") {
            Err(anyhow::Error::msg(
                "PaymentReference must start with `LGR-`.",
            ))
        } else if value.len() != 9 {
            Err(anyhow::Error::msg(
                "PaymentReference must have exactly 9 characters.",
            ))
        } else if value.matches(|c: char| c.is_ascii_uppercase()).count() != 8 {
            Err(anyhow::Error::msg(
                "PaymentReference must have only uppercase ASCII characters [A-Z].",
            ))
        } else {
            Ok(Self { value })
        }
    }

    pub fn random() -> Self {
        const RANDOM_CHARS_CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ"; // talisman-ignore-line
        const RANDOM_CHARS_LENGTH: usize = 5;

        let mut rng = rand::thread_rng();

        let random_part: String = (0..RANDOM_CHARS_LENGTH)
            .map(|_| {
                let index = rng.gen_range(0..RANDOM_CHARS_CHARSET.len());
                RANDOM_CHARS_CHARSET[index] as char
            })
            .collect();

        let value = format!("LGR-{random_part}");

        Self { value }
    }

    pub fn as_str(&self) -> &str {
        self.value.as_str()
    }
}

impl TryFrom<String> for PaymentReference {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        PaymentReference::new(value)
    }
}

impl std::str::FromStr for PaymentReference {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        PaymentReference::try_from(s.to_string())
    }
}

impl From<PaymentReference> for String {
    fn from(value: PaymentReference) -> Self {
        value.value
    }
}

impl std::fmt::Display for PaymentReference {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[cfg(test)]
mod tests {
    use super::PaymentReference;

    #[test]
    fn payment_reference_accepts_valid_string() {
        let result = PaymentReference::new("LGR-ABCDE".to_string());
        assert!(result.is_ok())
    }

    #[test]
    fn payment_reference_must_start_with_defined_string() {
        let result = PaymentReference::new("ABC-ABCDE".to_string());
        assert!(result.is_err())
    }

    #[test]
    fn payment_reference_must_not_be_less_than_9_letters_string() {
        let result = PaymentReference::new("LGR-ABCD".to_string());
        assert!(result.is_err())
    }

    #[test]
    fn payment_reference_must_not_be_more_than_9_letters_string() {
        let result = PaymentReference::new("LGR-ABCDEF".to_string());
        assert!(result.is_err())
    }

    #[test]
    fn payment_reference_must_be_all_uppercase_string() {
        let result = PaymentReference::new("LGR-abcde".to_string());
        assert!(result.is_err())
    }
}
