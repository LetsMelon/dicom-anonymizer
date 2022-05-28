use dicom_core::chrono::FixedOffset;
use dicom_core::Tag;
use dicom_core::value::{DicomDate, DicomDateTime};
use anonymizer::{Anonymizer};

fn main() {
    let mut t2 = Anonymizer::from_file("/Users/letsmelon/Desktop/test.dcm").unwrap();
    t2.meta(
        Anonymizer::meta_builder()
            .patient_name("Domenic Melcher")
            .patient_birth_date(DicomDateTime::from_date(
                DicomDate::from_ymd(2022, 1, 1).unwrap(),
                FixedOffset::east(0))
            )
            .remove_tag(Tag {
                0: 0x0010,
                1: 0x0040,
            })
            .remove_tags(vec![Tag {
                0: 0x0010,
                1: 0x0020,
            }])
            .build().unwrap()
    );

    // println!("{:?}", t2.anonymize());
    t2.anonymize();

    // t2.save("/Users/letsmelon/Desktop/test_out.dcm");
}
