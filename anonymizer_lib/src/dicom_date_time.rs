use std::fmt;
use chrono::DateTime;
use dicom_core::value::DicomDateTime;
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use field_count::FieldCount;
use serde::de::{MapAccess, SeqAccess, Visitor};
use serde::ser::SerializeStruct;

#[derive(Debug, Clone, FieldCount)]
pub struct CustomDicomDateTime {
    data: DicomDateTime,
}

impl CustomDicomDateTime {
    pub fn new(data: DicomDateTime) -> Self {
        Self {
            data
        }
    }
}

impl Serialize for CustomDicomDateTime {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer {
        assert!(CustomDicomDateTime::field_count() == 1);

        let mut state = serializer.serialize_struct("CustomDicomDateTime", 1)?;
        state.serialize_field("data", &self.data.to_string())?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for CustomDicomDateTime {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        enum Field { Data }

        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Field, D::Error>
                where
                    D: Deserializer<'de>,
            {
                struct FieldVisitor;

                impl<'de> Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str("`data`")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                        where
                            E: de::Error,
                    {
                        match value {
                            "data" => Ok(Field::Data),
                            _ => Err(de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct CustomDicomDateTimeVisitor;

        impl<'de> Visitor<'de> for CustomDicomDateTimeVisitor {
            type Value = CustomDicomDateTime;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct CustomDicomDateTime")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<CustomDicomDateTime, V::Error>
                where
                    V: SeqAccess<'de>,
            {
                let data: String = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;

                let offset = match DateTime::parse_from_str(&*data, "%F %T %:z") {
                    Ok(o) => o,
                    Err(_) => {
                        return Err(de::Error::custom(format!("Error in parse_from_str ({})", data)))
                    }
                };
                match DicomDateTime::try_from(&offset) {
                    Ok(d) => Ok(CustomDicomDateTime::new(d)),
                    Err(_) => Err(de::Error::custom("Error")),
                }
            }

            fn visit_map<V>(self, mut map: V) -> Result<CustomDicomDateTime, V::Error>
                where
                    V: MapAccess<'de>,
            {
                let mut data = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Data => {
                            if data.is_some() {
                                return Err(de::Error::duplicate_field("data"));
                            }
                            data = Some(map.next_value()?);
                        }
                    }
                }
                let data_raw: String = data.ok_or_else(|| de::Error::missing_field("data"))?;
                let offset = match DateTime::parse_from_str(&*data_raw, "%F %T %:z") {
                    Ok(o) => o,
                    Err(_) => {
                        return Err(de::Error::custom(format!("Error in parse_from_str ({})", data_raw)))
                    }
                };
                 match DicomDateTime::try_from(&offset) {
                    Ok(d) => Ok(CustomDicomDateTime::new(d)),
                    Err(_) => Err(de::Error::custom("Error")),
                }
            }
        }

        assert!(CustomDicomDateTime::field_count() == 1);
        const FIELDS: &'static [&'static str] = &["data"];
        deserializer.deserialize_struct("CustomDicomDateTime", FIELDS, CustomDicomDateTimeVisitor)
    }
}

impl From<DicomDateTime> for CustomDicomDateTime {
    fn from(ddt: DicomDateTime) -> Self {
        Self {
            data: ddt,
        }
    }
}

impl From<CustomDicomDateTime> for DicomDateTime {
    fn from(cddt: CustomDicomDateTime) -> Self {
        cddt.data
    }
}

impl PartialEq<CustomDicomDateTime> for CustomDicomDateTime {
    fn eq(&self, other: &CustomDicomDateTime) -> bool {
        let d1 = self.data;
        let d2 = other.data;

        let t1 = d1.time().unwrap();
        let t2 = d2.time().unwrap();

        d1.date() == d2.date()
            && d1.offset() == d2.offset()
            && t1.hour() == t2.hour()
            && t1.minute() == t2.minute()
            && t1.second() == t2.second()
    }
}

impl PartialEq<DicomDateTime> for CustomDicomDateTime {
    fn eq(&self, other: &DicomDateTime) -> bool {
        self.eq(&CustomDicomDateTime::from(*other))
    }
}

impl PartialEq<CustomDicomDateTime> for DicomDateTime {
    fn eq(&self, other: &CustomDicomDateTime) -> bool {
        other.eq(self)
    }
}

#[cfg(test)]
mod tests {
    use std::error::Error;
    use chrono::FixedOffset;
    use dicom_core::value::{DicomDate, DicomDateTime, DicomTime};
    use serde_json::from_str;
    use crate::dicom_date_time::CustomDicomDateTime;

    impl CustomDicomDateTime {
        fn factory_ddt() -> Result<DicomDateTime, Box<dyn Error>> {
            Ok(DicomDateTime::from_date_and_time(
                DicomDate::from_ymd(2000, 11, 5)?,
                DicomTime::from_hms(12,14,5)?,
                FixedOffset::east(60*60)
            )?)
        }

        fn factory() -> Result<Self, Box<dyn Error>> {
            Ok(CustomDicomDateTime::new(CustomDicomDateTime::factory_ddt()?))
        }
    }

    #[test]
    fn custom_and_dicom_date_time_are_the_same() {
        let d_dt = CustomDicomDateTime::factory_ddt().unwrap();
        let c_dt = CustomDicomDateTime::new(d_dt);

        assert_eq!(c_dt, d_dt);
        assert_eq!(c_dt, CustomDicomDateTime::from(d_dt));
        assert_eq!(DicomDateTime::from(c_dt), d_dt);
    }

    #[test]
    fn can_be_serialized() {
        let c_dt = CustomDicomDateTime::factory().unwrap();

        assert_eq!(
            "{\"data\":\"2000-11-05 12:14:05 +01:00\"}",
            serde_json::to_string(&c_dt).unwrap()
        )
    }

    #[test]
    fn can_be_deserialized() {
        let c_dt = CustomDicomDateTime::factory().unwrap();

        let parsed = from_str::<CustomDicomDateTime>("{\"data\":\"2000-11-05 12:14:05 +01:00\"}").unwrap();

        assert_eq!(parsed, c_dt)
    }
}
