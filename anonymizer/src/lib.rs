#[macro_use]
mod macros;

#[macro_use]
extern crate derive_builder;
use std::error::Error;
use dicom_core::{DataElement, dicom_value, VR, value::DicomDateTime};
use dicom_dictionary_std::tags;
use dicom_object::{DefaultDicomObject, open_file};

#[derive(Debug, Builder)]
pub struct AnonymizerMeta {
    #[builder(setter(into, strip_option), default)]
    patient_name: Option<String>,

    #[builder(setter(into, strip_option), default)]
    patient_birth_date: Option<DicomDateTime>
}

#[derive(Debug)]
pub struct AnonymizerFile {
    path: String,
    obj: DefaultDicomObject,
    updated_obj: bool,
}

#[derive(Debug)]
pub struct Anonymizer {
    file: Option<AnonymizerFile>,
    meta: AnonymizerMeta,
}

// Constructors
impl Anonymizer {
    fn new() -> Result<Anonymizer, Box<dyn Error>> {
        Ok(Self {
            file: Option::None,
            meta: AnonymizerMetaBuilder::default().build()?,
        })
    }

    pub fn from_file(path: &str) -> Result<Self, Box<dyn Error>> {
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

    pub fn from_object(object: DefaultDicomObject) -> Result<Self, Box<dyn Error>> {
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

    pub fn save(&mut self, path: &str) -> Result<(), Box<dyn Error>> {
        match &self.file {
            Some(file) => {
                if file.updated_obj {
                    self.anonymize()
                }
                self.file.as_ref().unwrap().obj.write_to_file(path)?;
                Ok(())
            },
            None => panic!("Nothing to save"),
        }
    }

    pub fn anonymize(&mut self) {
        // let patient_name: InMemElement = ;

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
    }
}
