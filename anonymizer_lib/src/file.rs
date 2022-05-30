use dicom_object::DefaultDicomObject;

#[derive(Debug)]
pub struct AnonymizerFile {
    pub(crate) obj: DefaultDicomObject,
    pub(crate) updated_obj: bool,
}
