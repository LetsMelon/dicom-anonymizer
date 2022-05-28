use dicom_core::Tag;
use dicom_core::value::DicomDateTime;

#[derive(Debug, Builder)]
pub struct AnonymizerMeta {
    #[builder(setter(into, strip_option), default)]
    pub(crate) patient_name: Option<String>,

    #[builder(setter(into, strip_option), default)]
    pub(crate) patient_birth_date: Option<DicomDateTime>,

    #[builder(setter(custom, into, strip_option), default)]
    pub(crate) remove_tags: Vec<Tag>
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

    pub fn remove_tags(&mut self, values: Vec<Tag>) -> &mut Self {
        let obj = self;

        for item in values {
            obj.remove_tag(item);
        }

        obj
    }
}
