use derive_more::Display;
use dicom_core::Tag;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use strum::EnumCount;
use tags_list_lib::List as TagsList;

use crate::tag::CustomTag;

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

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum TagAction<T> {
    /// Change the tag with the given value, similar to `Option::Some(T)`
    Change(T),
    /// Changes nothing, similar to `Option::None`
    Keep,
    /// Remove the tag
    Remove,
}

impl<T> TagAction<T> {
    /// Maps an `TagAction::Change<T>` to `Option<U>` by applying a function to a contained value.
    /// Copied from https://doc.rust-lang.org/std/option/enum.Option.html#method.map
    #[inline]
    pub fn map<U, F>(self, f: F) -> TagAction<U>
    where
        F: FnOnce(T) -> U,
    {
        match self {
            TagAction::Change(v) => TagAction::Change(f(v)),
            TagAction::Keep => TagAction::Keep,
            TagAction::Remove => TagAction::Remove,
        }
    }
}

impl<T> Default for TagAction<T> {
    fn default() -> Self {
        TagAction::Keep
    }
}

impl<T> From<Option<T>> for TagAction<T> {
    fn from(opt: Option<T>) -> Self {
        match opt {
            None => TagAction::default(),
            Some(value) => TagAction::Change(value),
        }
    }
}

impl<T> From<TagAction<T>> for Option<T> {
    fn from(tag_action: TagAction<T>) -> Self {
        match tag_action {
            TagAction::Change(value) => Option::Some(value),
            TagAction::Keep => Option::None,
            TagAction::Remove => Option::None,
        }
    }
}

#[cfg(test)]
mod tests {
    mod tag_action {
        use crate::TagAction;

        #[test]
        fn has_option_map_like_function() {
            assert_eq!(
                TagAction::Change("MyString"),
                TagAction::Change(1).map(|_| "MyString")
            );
            assert_eq!(
                TagAction::Change("MyString"),
                TagAction::Change(Option::Some("MyString")).map(|value| value.unwrap())
            );
            assert_ne!(
                TagAction::Change("MyString"),
                TagAction::Keep.map(|_: String| "MyString")
            );
        }

        #[test]
        fn can_be_transformed_into_an_option_and_back() {
            type T = usize;

            assert_eq!(
                Option::Some("MyString"),
                Option::from(TagAction::Change("MyString")),
            );
            assert_eq!(
                Option::<T>::None,
                Option::from(TagAction::<T>::Keep),
            );
            assert_eq!(
                Option::<T>::None,
                Option::from(TagAction::<T>::Remove),
            );

            assert_eq!(
                TagAction::Change(1),
                TagAction::from(Option::Some(1)),
            );
            assert_eq!(
                TagAction::<T>::Keep,
                TagAction::from(Option::<T>::None),
            );
        }
    }
}
