# dicom-anonymizer

CLI program written in Rust to anonymize DICOM files

```
anonymizer 0.1.0
Domenic Melcher

USAGE:
    anonymizer [OPTIONS] <FILE>

ARGS:
    <FILE>    DICOM file to anonymize

OPTIONS:
    -d, --dry-run
            If set then the file will not be saved

    -h, --help
            Print help information

    -o, --output <output>
            Output path for DICOM file

    -p, --patient-name <patient_name>
            Change the patient name

        --patient-birth-day <patient_birth_day>
            Change the patient birthday (yyy-mm-dd or yyyy-m-d)

        --patient-sex <patient_sex>
            Change the patient sex (M,F,O)

        --remove-tags <remove_tags>...
            Remove dicom tags from the object. Example: 0x0010-0x0020,0x0010-0x0040

    -V, --version
            Print version information
```
