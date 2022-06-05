use std::str::FromStr;
use derive_more::{Display};
use dicom_core::Tag;
use tags_list_lib::List as TagsList;
use strum::EnumCount;
use serde::{Serialize, Deserialize};

use crate::tag::CustomTag;

#[derive(Display, Copy, Clone, Debug, EnumCount, Serialize, Deserialize)]
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
    Vec(Vec<CustomTag>),
    List(TagsList),
    VecList(Vec<TagsList>),
}

impl From<Vec<Tag>> for RemoveTagsInput {
    fn from(v_t: Vec<Tag>) -> Self {
        RemoveTagsInput::Vec(CustomTag::from_vec(v_t))
    }
}

impl From<Vec<CustomTag>> for RemoveTagsInput {
    fn from(v_ct: Vec<CustomTag>) -> Self {
        RemoveTagsInput::Vec(v_ct)
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
