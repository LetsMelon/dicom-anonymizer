use anyhow::{anyhow, Result};
use dicom_core::value::{DicomDateTime, Value, ValueType};
use dicom_core::{dicom_value, DataElement, DicomValue, PrimitiveValue, Tag, VR};
use dicom_dictionary_std::tags;
use dicom_object::{open_file, DefaultDicomObject, InMemDicomObject};

use crate::enums::PatientSex;
use crate::file::AnonymizerFile;
use crate::meta::{AnonymizerMeta, AnonymizerMetaBuilder};
use crate::TagAction;

#[derive(Debug, Clone)]
pub struct Anonymizer {
    file: Option<AnonymizerFile>,
    meta: AnonymizerMeta,
}

// Constructors
impl Anonymizer {
    pub fn new() -> Result<Self> {
        Ok(Self {
            file: Option::None,
            meta: AnonymizerMetaBuilder::default().build()?,
        })
    }

    pub fn from_file(path: &str) -> Result<Self> {
        let mut back = Self::new()?;

        back.file = Option::from(AnonymizerFile {
            obj: open_file(path)?,
            updated_obj: false,
        });

        Ok(back)
    }

    pub fn from_object(object: DefaultDicomObject) -> Result<Self> {
        let mut back = Self::new()?;

        back.file = Option::from(AnonymizerFile {
            obj: object,
            updated_obj: false,
        });

        Ok(back)
    }
}

impl Anonymizer {
    pub fn meta_builder() -> AnonymizerMetaBuilder {
        AnonymizerMetaBuilder::default()
    }

    pub fn meta(&mut self, meta: AnonymizerMeta) {
        self.meta = meta;
    }

    pub fn save(&mut self, path: &str) -> Result<()> {
        match &self.file {
            Some(file) => {
                if file.updated_obj {
                    self.anonymize()
                }
                self.file.as_ref().unwrap().obj.write_to_file(path)?;
                Ok(())
            }
            None => Err(anyhow!("Need to have a initialised DICOM object")),
        }
    }

    fn match_value<T>(
        &mut self,
        item: &TagAction<T>,
        tag: Tag,
        vr: VR,
        translate: fn(value: &T) -> PrimitiveValue,
    ) -> Result<()> {
        match item {
            TagAction::Change(value) => {
                self.file
                    .as_mut()
                    .unwrap()
                    .obj
                    .put(DataElement::<InMemDicomObject, Vec<u8>>::new(
                        tag,
                        vr,
                        DicomValue::from(translate(value)),
                    ))
                    .unwrap();
            }
            TagAction::Keep => {}
            TagAction::Remove => {
                todo!("Implement logic to delete a tag");
            }
        }

        Ok(())
    }

    pub fn anonymize(&mut self) {
        println!("{:?}", self.meta);

        self.match_value(
            &self.meta.patient_name.clone(),
            tags::PATIENT_NAME,
            VR::PN,
            |value| PrimitiveValue::Str(value.to_owned()),
        );

        self.match_value(
            &self.meta.patient_birth_date.clone(),
            tags::PATIENT_BIRTH_DATE,
            VR::DA,
            |value| {
                let ddt = DicomDateTime::from(value.clone());
                PrimitiveValue::from(ddt)
            },
        );

        for item in &self.meta.remove_tags {
            self.file
                .as_mut()
                .unwrap()
                .obj
                .remove_element(Tag::from(item.clone()));
        }

        self.match_value(
            &self.meta.patient_sex.clone(),
            tags::PATIENT_SEX,
            VR::CS,
            |value| PrimitiveValue::Str(value.value().to_owned()),
        );
    }
}
