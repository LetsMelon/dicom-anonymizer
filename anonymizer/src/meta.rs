use dicom_core::Tag;
use dicom_core::value::DicomDateTime;

use crate::enums::{PatientSex, RemoveTagsInput};

#[derive(Debug, Builder)]
pub struct AnonymizerMeta {
    #[builder(setter(into, strip_option), default)]
    pub(crate) patient_name: Option<String>,

    #[builder(setter(into, strip_option), default)]
    pub(crate) patient_birth_date: Option<DicomDateTime>,

    #[builder(setter(custom, into, strip_option), default)]
    pub(crate) remove_tags: Vec<Tag>,

    #[builder(setter(into, strip_option), default)]
    pub(crate) patient_sex: Option<PatientSex>,
}

impl AnonymizerMetaBuilder {
    pub fn remove_tag(&mut self, value: Tag) -> &mut Self {
        let mut obj = self;

        if obj.remove_tags.is_none() {
            obj.remove_tags = Some(Vec::<Tag>::new());
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
                obj.remove_tags(t.value().into());
            },
            RemoveTagsInput::VecList(vt) => {
                for item in vt {
                    obj.remove_tags(item.into());
                }
            }
        }

        obj
    }
}
