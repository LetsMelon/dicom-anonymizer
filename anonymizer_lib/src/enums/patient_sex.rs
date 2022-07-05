use derive_more::Display;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use strum::EnumCount;

#[derive(Display, Copy, Clone, Debug, EnumCount, Serialize, Deserialize, Eq, PartialEq)]
pub enum PatientSex {
    M,
    F,
    O,
}

impl FromStr for PatientSex {
    type Err = ::strum::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        assert!(
            PatientSex::COUNT == 3,
            "TODO: implement new patient sex value"
        );
        match s.to_ascii_lowercase().as_str() {
            "m" => Ok(PatientSex::M),
            "f" => Ok(PatientSex::F),
            "o" => Ok(PatientSex::O),
            _ => Err(::strum::ParseError::VariantNotFound),
        }
    }
}

impl PatientSex {
    pub fn value(&self) -> &str {
        match *self {
            PatientSex::M => "M",
            PatientSex::F => "F",
            PatientSex::O => "O",
        }
    }
}

impl Default for PatientSex {
    fn default() -> Self {
        PatientSex::M
    }
}

#[cfg(test)]
mod tests {
    use super::PatientSex;
    use strum::EnumCount;

    mod from_str {
        use super::PatientSex;
        use std::str::FromStr;

        #[test]
        fn ok() {
            assert_eq!(PatientSex::M, PatientSex::from_str("M").unwrap());
            assert_eq!(PatientSex::F, PatientSex::from_str("F").unwrap());
            assert_eq!(PatientSex::O, PatientSex::from_str("O").unwrap());
            assert_eq!(PatientSex::M, PatientSex::from_str("m").unwrap());
            assert_eq!(PatientSex::F, PatientSex::from_str("f").unwrap());
            assert_eq!(PatientSex::O, PatientSex::from_str("o").unwrap());
        }

        #[test]
        fn error() {
            let ps = PatientSex::from_str("MM");

            match ps {
                Ok(_) => assert!(false, "Should throw an error"),
                Err(_) => assert!(true),
            };
        }
    }

    #[test]
    fn enum_count() {
        assert_eq!(PatientSex::COUNT, 3);
    }
}
