use diesel::prelude::*;
use diesel::sql_types::BigInt;

#[derive(QueryableByName, Clone, Copy, Debug, PartialEq, Eq)]
pub struct StartNumber {
    #[diesel(sql_type = BigInt)]
    #[diesel(column_name = start_number)]
    value: i64,
}

impl StartNumber {
    pub const DENYLIST: [i64; 20] = [
        18, 28, 33, 45, 74, 84, 88, 444, 191, 192, 198, 420, 1312, 1717, 1887, 1910, 1919, 1933,
        1488, 1681,
    ];

    pub fn new(value: i64) -> anyhow::Result<Self> {
        if StartNumber::DENYLIST.contains(&value) {
            return Err(anyhow::Error::msg(format!(
                "Start number {} is not allowed.",
                &value,
            )));
        } else if !value.is_positive() {
            return Err(anyhow::Error::msg(
                "Start numbers must have positive values.",
            ));
        }

        Ok(Self { value })
    }
}

impl From<StartNumber> for i64 {
    fn from(value: StartNumber) -> Self {
        value.value
    }
}

impl From<&StartNumber> for i64 {
    fn from(value: &StartNumber) -> Self {
        value.value
    }
}

impl std::fmt::Display for StartNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl TryFrom<i64> for StartNumber {
    type Error = anyhow::Error;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        StartNumber::new(value)
    }
}

impl TryFrom<i32> for StartNumber {
    type Error = anyhow::Error;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        StartNumber::new(value as i64)
    }
}

impl TryFrom<i16> for StartNumber {
    type Error = anyhow::Error;

    fn try_from(value: i16) -> Result<Self, Self::Error> {
        StartNumber::new(value as i64)
    }
}

impl TryFrom<i8> for StartNumber {
    type Error = anyhow::Error;

    fn try_from(value: i8) -> Result<Self, Self::Error> {
        StartNumber::new(value as i64)
    }
}

impl TryFrom<u64> for StartNumber {
    type Error = anyhow::Error;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        StartNumber::new(value as i64)
    }
}

impl TryFrom<u32> for StartNumber {
    type Error = anyhow::Error;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        StartNumber::new(value as i64)
    }
}

impl TryFrom<u16> for StartNumber {
    type Error = anyhow::Error;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        StartNumber::new(value as i64)
    }
}

impl TryFrom<u8> for StartNumber {
    type Error = anyhow::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        StartNumber::new(value as i64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn try_from_u64() {
        let result = StartNumber::try_from(73u64);
        assert_eq!(result.unwrap().value, 73i64)
    }
}
