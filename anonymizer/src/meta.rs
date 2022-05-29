use dicom_core::Tag;
use dicom_core::value::DicomDateTime;
use derive_more::{Display};
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
            RemoveTagsInput::List(v) => {
                obj.remove_tags(v.value().into());
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
