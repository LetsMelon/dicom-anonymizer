use dicom_core::{DataElement, dicom_value, VR, value::DicomDateTime};
use dicom_dictionary_std::tags;
use dicom_object::{DefaultDicomObject, open_file};
use anyhow::{anyhow, Result};

use crate::file::AnonymizerFile;
use crate::meta::{AnonymizerMeta, AnonymizerMetaBuilder};
use crate::PatientSex;

#[derive(Debug)]
pub struct Anonymizer {
    file: Option<AnonymizerFile>,
    meta: AnonymizerMeta,
}

// Constructors
impl Anonymizer {
    fn new() -> Result<Self> {
        Ok(Self {
            file: Option::None,
            meta: AnonymizerMetaBuilder::default().build()?,
        })
    }

    pub fn from_file(path: &str) -> Result<Self> {
        let mut back = Self::new()?;

        back.file = Option::from({
            let file = AnonymizerFile {
                path: path.clone().to_owned(),
                obj: open_file(path)?,
                updated_obj: false,
            };

            file
        });

        Ok(back)
    }

    pub fn from_object(object: DefaultDicomObject) -> Result<Self> {
        let mut back = Self::new()?;

        back.file = Option::from(AnonymizerFile {
            path: "".to_string(),
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
            },
            None => Err(anyhow!("Need to have a initialised DICOM object")),
        }
    }

    pub fn anonymize(&mut self) {
        // let patient_name: InMemElement = ;
        println!("{:?}", self.meta);

        match_field!(&self.meta.patient_name, (|v: &str| {
            self.file.as_mut().unwrap().obj.put(DataElement::new(
                tags::PATIENT_NAME,
                VR::PN,
                dicom_value!(Str, v),
            ));
        }));

        match_field!(&self.meta.patient_birth_date, (|v: &DicomDateTime| {
            self.file.as_mut().unwrap().obj.put(DataElement::new(
                tags::PATIENT_BIRTH_DATE,
                VR::DA,
                dicom_value!(DateTime, *v),
            ));
        }));

        for item in &self.meta.remove_tags {
            self.file.as_mut().unwrap().obj.remove_element(*item);
        }

        match_field!(&self.meta.patient_sex, (|v: &PatientSex| {
            self.file.as_mut().unwrap().obj.put(DataElement::new(
                tags::PATIENT_SEX,
                VR::CS,
                dicom_value!(Str, v.value()),
            ));
        }));
    }
}
