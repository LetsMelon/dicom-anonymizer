use derive_more::{Display};
use dicom_core::Tag;
use tags_list::List as TagsList;

#[derive(Display, Clone, Debug)]
pub enum PatientSex {
    M,
    F,
    O,
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
