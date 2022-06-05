use dicom_core::header::{ElementNumber, GroupNumber};
use dicom_core::Tag;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomTag {
    group: GroupNumber,
    element: ElementNumber,
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
