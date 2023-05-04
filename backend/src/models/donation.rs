use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Donation {
    value: u16,
    string_representation: String,
}

impl Donation {
    const MIN_VALUE: u16 = 5;

    fn new(value: u16) -> anyhow::Result<Self> {
        if value < Self::MIN_VALUE {
            return Err(anyhow::Error::msg(format!(
                "Donation must be at least {} Euros but was {}.",
                Self::MIN_VALUE,
                value,
            )));
        }

        let string_representation = value.to_string();

        Ok(Self {
            value,
            string_representation,
        })
    }
}

impl TryFrom<String> for Donation {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Donation::new(value.parse()?)
    }
}

impl TryFrom<&str> for Donation {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Donation::new(value.parse()?)
    }
}

impl FromStr for Donation {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Donation::try_from(s)
    }
}

impl TryFrom<usize> for Donation {
    type Error = anyhow::Error;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        Donation::new(u16::try_from(value)?)
    }
}

impl TryFrom<u64> for Donation {
    type Error = anyhow::Error;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        Donation::new(u16::try_from(value)?)
    }
}

impl TryFrom<u32> for Donation {
    type Error = anyhow::Error;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Donation::new(u16::try_from(value)?)
    }
}

impl TryFrom<u16> for Donation {
    type Error = anyhow::Error;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Donation::new(value)
    }
}

impl TryFrom<u8> for Donation {
    type Error = anyhow::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Donation::new(value as u16)
    }
}

impl TryFrom<isize> for Donation {
    type Error = anyhow::Error;

    fn try_from(value: isize) -> Result<Self, Self::Error> {
        Donation::new(u16::try_from(value)?)
    }
}

impl TryFrom<i64> for Donation {
    type Error = anyhow::Error;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        Donation::new(u16::try_from(value)?)
    }
}

impl TryFrom<i32> for Donation {
    type Error = anyhow::Error;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        Donation::new(u16::try_from(value)?)
    }
}

impl TryFrom<i16> for Donation {
    type Error = anyhow::Error;

    fn try_from(value: i16) -> Result<Self, Self::Error> {
        Donation::new(u16::try_from(value)?)
    }
}

impl TryFrom<i8> for Donation {
    type Error = anyhow::Error;

    fn try_from(value: i8) -> Result<Self, Self::Error> {
        Donation::new(u16::try_from(value)?)
    }
}

impl From<Donation> for u64 {
    fn from(value: Donation) -> Self {
        value.value as u64
    }
}

impl From<Donation> for u32 {
    fn from(value: Donation) -> Self {
        value.value as u32
    }
}

impl From<Donation> for u16 {
    fn from(value: Donation) -> Self {
        value.value
    }
}

impl TryFrom<Donation> for u8 {
    type Error = std::num::TryFromIntError;

    fn try_from(value: Donation) -> Result<Self, Self::Error> {
        u8::try_from(value.value)
    }
}

impl From<Donation> for usize {
    fn from(value: Donation) -> Self {
        value.value as usize
    }
}

impl From<Donation> for i64 {
    fn from(value: Donation) -> Self {
        value.value as i64
    }
}

impl From<Donation> for i32 {
    fn from(value: Donation) -> Self {
        value.value as i32
    }
}

impl From<Donation> for i16 {
    fn from(value: Donation) -> Self {
        value.value as i16
    }
}

impl TryFrom<Donation> for i8 {
    type Error = std::num::TryFromIntError;

    fn try_from(value: Donation) -> Result<Self, Self::Error> {
        i8::try_from(value.value)
    }
}

impl From<Donation> for isize {
    fn from(value: Donation) -> Self {
        value.value as isize
    }
}

impl AsRef<str> for Donation {
    fn as_ref(&self) -> &str {
        self.string_representation.as_ref()
    }
}

impl Display for Donation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn donation_must_not_be_negative() {
        let donation = Donation::try_from(-1);
        assert!(donation.is_err());
    }

    #[test]
    fn donation_must_not_be_0() {
        let donation = Donation::new(0);
        assert!(donation.is_err());
    }

    #[test]
    fn donation_must_be_at_least_5() {
        let donation = Donation::new(4);
        assert!(donation.is_err());
    }

    #[test]
    fn donation_can_be_at_least_5() {
        let donation = Donation::new(5);
        assert!(donation.is_ok());
    }

    #[test]
    fn donation_can_be_parsed_from_string() {
        let donation = Donation::new(73).unwrap();
        let string_parsed_donation: Donation = "73".parse().unwrap();
        assert_eq!(donation, string_parsed_donation);
    }
}
