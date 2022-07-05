use serde::{Deserialize, Serialize};

use crate::enums::{PatientSex, RemoveTagsInput};
use crate::types::{CustomDicomDateTime, CustomTag};
use crate::TagAction;

#[derive(Debug, Builder, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[builder(derive(Debug))]
pub struct AnonymizerMeta {
    #[builder(setter(into, strip_option), default)]
    pub(crate) patient_name: TagAction<String>,

    #[builder(setter(into, strip_option), default)]
    pub(crate) patient_birth_date: TagAction<CustomDicomDateTime>,

    #[builder(setter(custom, into, strip_option), default)]
    pub(crate) remove_tags: Vec<CustomTag>,

    #[builder(setter(into, strip_option), default)]
    pub(crate) patient_sex: TagAction<PatientSex>,
}

impl AnonymizerMetaBuilder {
    pub fn remove_tag(&mut self, value: CustomTag) -> &mut Self {
        let mut obj = self;

        if obj.remove_tags.is_none() {
            obj.remove_tags = Some(Vec::<CustomTag>::new());
        }
        obj.remove_tags.as_mut().unwrap().push(value);

        obj
    }

    pub fn remove_tags(&mut self, values: RemoveTagsInput) -> &mut Self {
        let obj = self;

        match values {
            RemoveTagsInput::Vec(v) => {
                for item in v {
                    obj.remove_tag(item);
                }
            }
            RemoveTagsInput::List(t) => {
                obj.remove_tags(RemoveTagsInput::from(CustomTag::from_vec(t.value())));
            }
            RemoveTagsInput::VecList(vt) => {
                for item in vt {
                    obj.remove_tags(RemoveTagsInput::from(item));
                }
            }
        }

        obj
    }
}

#[cfg(test)]
mod tests {

    mod serialize {
        use crate::types::{CustomDicomDateTime, CustomTag};
        use crate::{AnonymizerMeta, PatientSex, TagAction};
        use chrono::FixedOffset;
        use dicom_core::value::{DicomDate, DicomDateTime};

        #[test]
        fn default() {
            let am = AnonymizerMeta {
                patient_name: TagAction::default(),
                patient_birth_date: TagAction::default(),
                remove_tags: vec![],
                patient_sex: TagAction::default(),
            };
            insta::assert_json_snapshot!(am);
        }

        #[test]
        fn basic_values() {
            let am = AnonymizerMeta {
                patient_name: TagAction::Remove,
                patient_birth_date: TagAction::Keep,
                remove_tags: vec![CustomTag {
                    group: 0,
                    element: 0,
                }],
                patient_sex: TagAction::Remove,
            };
            insta::assert_json_snapshot!(am);
        }

        #[test]
        fn complex_values() {
            let am = AnonymizerMeta {
                patient_name: TagAction::Change("New Patient Name".to_string()),
                patient_birth_date: TagAction::Change(CustomDicomDateTime::new(
                    DicomDateTime::from_date(
                        DicomDate::from_ymd(2016, 8, 12).unwrap(),
                        FixedOffset::east(0),
                    ),
                )),
                remove_tags: vec![CustomTag {
                    group: 0,
                    element: 0,
                }],
                patient_sex: TagAction::Change(PatientSex::O),
            };
            insta::assert_json_snapshot!(am);
        }
    }
}
