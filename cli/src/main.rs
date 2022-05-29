use dicom_core::chrono::FixedOffset;
use dicom_core::Tag;
use dicom_core::value::{DicomDate, DicomDateTime};
use anonymizer::{Anonymizer, PatientSex};
use tags_list::List;

fn main() {
    let mut t2 = Anonymizer::from_file("/Users/letsmelon/Desktop/test.dcm").unwrap();
    t2.meta(
        Anonymizer::meta_builder()
            .patient_name("Domenic Melcher")
            .patient_birth_date(DicomDateTime::from_date(
                DicomDate::from_ymd(2022, 1, 1).unwrap(),
                FixedOffset::east(0))
            )
            .remove_tags(List::PATIENT.into())
            .patient_sex(PatientSex::O)
            .build().unwrap()
    );

    t2.anonymize();

    // t2.save("/Users/letsmelon/Desktop/test_out_3.dcm");
}
