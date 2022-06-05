use dicom_core::Tag;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, Clone)]
pub struct CustomTag(Tag);

impl Serialize for CustomTag {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer {
        todo!()
    }
}

impl<'de> Deserialize<'de> for CustomTag {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de> {
        todo!()
    }
}

impl From<Tag> for CustomTag {
    fn from(v: Tag) -> Self {
        Self {
            0: v
        }
    }
}

impl From<CustomTag> for Tag {
    fn from(ct: CustomTag) -> Self {
        ct.0
    }
}
