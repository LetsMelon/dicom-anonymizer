use dicom_core::Tag;
use tags_list_lib::List as TagsList;

use crate::types::CustomTag;

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
