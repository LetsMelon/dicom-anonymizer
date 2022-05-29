use std::u16;
use dicom_core::chrono::FixedOffset;
use dicom_core::Tag;
use dicom_core::value::DicomDateTime;
use anonymizer::{Anonymizer, PatientSex};
use anyhow::Result;
use clap::Parser;
use std::str::FromStr;
use chrono::{DateTime, NaiveDate, NaiveTime, Utc};

#[derive(Parser, Debug)]
#[clap(author="Domenic Melcher", version, about, long_about = None)]
struct Cli {
    #[clap(short, long, parse(from_os_str))]
    input: std::path::PathBuf,

    #[clap(short, long, parse(from_os_str))]
    output: std::path::PathBuf,

    #[clap(short, long, multiple_values = true)]
    patient_name: Option<Vec<String>>,

    #[clap(long)]
    patient_sex: Option<String>,

    #[clap(long)]
    patient_birth_date: Option<String>,

    #[clap(short, long, multiple_values = true, value_delimiter = ',')]
    remove_tags: Vec<String>,
}

fn main() -> Result<()> {
    let args: Cli = Cli::parse();

    // println!("{:?}", args);

    let mut obj = Anonymizer::from_file(&args.input.to_string_lossy())?;
    let mut builder = Anonymizer::meta_builder();

    match args.patient_name {
        Some(v) => {
            builder.patient_name(v.join(" "));
        }
        None => (),
    }

    match args.patient_sex {
        Some(ps) => {
            builder.patient_sex(PatientSex::from_str(&ps)?);
        },
        None => (),
    };

    match args.patient_birth_date {
        Some(pbd) => {
            let ndt = NaiveDate::parse_from_str(&*pbd, "%Y-%m-%d")?
                .and_time(NaiveTime::from_hms(0, 0,0));
            let dt_offset: DateTime<FixedOffset> = DateTime::<Utc>::from_utc(ndt, Utc).into();
            builder.patient_birth_date(DicomDateTime::try_from(&dt_offset)?);
        },
        None => (),
    };

    let mut remove_tags = Vec::<Tag>::new();
    for item in args.remove_tags {
        let splitted = item.split("-").collect::<Vec<&str>>();

        let group_number = u16::from_str_radix( splitted[0].trim_start_matches("0x"), 16)?;
        let element_number = u16::from_str_radix( splitted[1].trim_start_matches("0x"), 16)?;
        remove_tags.push(Tag {
            0: group_number,
            1: element_number,
        });
    }
    builder.remove_tags(remove_tags.into());

    obj.meta(builder.build()?);

    obj.anonymize();

    obj.save(&args.output.to_string_lossy())?;

    Ok(())
}
