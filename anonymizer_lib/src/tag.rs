use dicom_core::header::{ElementNumber, GroupNumber};
use dicom_core::Tag;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CustomTag {
    pub group: GroupNumber,
    pub element: ElementNumber,
}

impl From<Tag> for CustomTag {
    fn from(v: Tag) -> Self {
        CustomTag {
            group: v.0,
            element: v.1
        }
    }
}

impl From<CustomTag> for Tag {
    fn from(ct: CustomTag) -> Self {
        Tag {
            0: ct.group,
            1: ct.element
        }
    }
}

impl PartialEq<Tag> for CustomTag {
    fn eq(&self, other: &Tag) -> bool {
        self.group == other.group() && self.element == other.element()
    }
}

impl PartialEq<CustomTag> for Tag {
    fn eq(&self, other: &CustomTag) -> bool {
        self.group() == other.group && self.element() == other.element
    }
}

impl CustomTag {
    pub fn new(group: GroupNumber, element: ElementNumber) -> Self {
        Self {
            group, element
        }
    }

    pub fn from_vec(v_t: Vec<Tag>) -> Vec<CustomTag> {
        v_t.iter().map(|item| CustomTag::from(item.to_owned())).collect()
    }
}

#[cfg(test)]
mod tests {
    use dicom_core::Tag;
    use serde_json::from_str;
    use crate::types::CustomTag;

    #[test]
    fn custom_and_dicom_tag_are_the_same() {
        let c_t = CustomTag::new(10,10);
        let d_t = Tag(10, 10);

        assert_eq!(c_t, d_t);
        assert_eq!(c_t, CustomTag::from(d_t));
        assert_eq!(Tag::from(c_t), d_t);
    }

    #[test]
    fn can_be_serialized() {
        let c_t = CustomTag::new(10,10);

        assert_eq!("{\"group\":10,\"element\":10}", serde_json::to_string(&c_t).unwrap())
    }

    #[test]
    fn can_be_deserialized() {
        let c_t = CustomTag::new(20,15);

        assert_eq!(from_str::<CustomTag>("{\"group\":20,\"element\":15}").unwrap(), c_t)
    }
}
