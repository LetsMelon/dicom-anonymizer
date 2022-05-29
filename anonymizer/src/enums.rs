use std::str::FromStr;
use derive_more::{Display};
use dicom_core::Tag;
use tags_list::List as TagsList;
use strum::EnumCount;

#[derive(Display, Clone, Debug, EnumCount)]
pub enum PatientSex {
    M,
    F,
    O,
}

impl FromStr for PatientSex {
    type Err = ::strum::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        assert!(PatientSex::COUNT == 3, "TODO: implement new patient sex value");
        match s.to_ascii_lowercase().as_str() {
            "m" => Ok(PatientSex::M),
            "f" => Ok(PatientSex::F),
            "o" => Ok(PatientSex::O),
            _ => Err(::strum::ParseError::VariantNotFound)
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
    fn default() -> Self { PatientSex::M }
}

pub enum RemoveTagsInput {
    Vec(Vec<Tag>),
    List(TagsList),
    VecList(Vec<TagsList>)
}

impl From<Vec<Tag>> for RemoveTagsInput {
    fn from(v: Vec<Tag>) -> Self {
        RemoveTagsInput::Vec(v)
    }
}

impl From<TagsList> for RemoveTagsInput {
    fn from(t: TagsList) -> Self {
        RemoveTagsInput::List(t)
    }
}

impl From<Vec<TagsList>> for RemoveTagsInput {
    fn from(vt: Vec<TagsList>) -> Self {
        RemoveTagsInput::VecList(vt)
    }
}
