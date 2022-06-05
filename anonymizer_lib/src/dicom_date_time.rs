use dicom_core::value::DicomDateTime;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, Clone)]
pub struct CustomDicomDateTime(DicomDateTime);

impl Serialize for CustomDicomDateTime {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer {
        todo!()
    }
}

impl<'de> Deserialize<'de> for CustomDicomDateTime {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de> {
        todo!()
    }
}

impl From<DicomDateTime> for CustomDicomDateTime {
    fn from(ddt: DicomDateTime) -> Self {
        Self {
            0: ddt,
        }
    }
}

impl From<CustomDicomDateTime> for DicomDateTime {
    fn from(cddt: CustomDicomDateTime) -> Self {
        cddt.0
    }
}
