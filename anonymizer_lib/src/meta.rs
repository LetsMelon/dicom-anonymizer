use serde::{Deserialize, Serialize};

use crate::enums::{PatientSex, RemoveTagsInput};
use crate::types::{CustomDicomDateTime, CustomTag};
use crate::TagAction;

#[derive(Debug, Builder, Clone, Serialize, Deserialize)]
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
