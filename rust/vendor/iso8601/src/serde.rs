use alloc::{fmt, string::ToString};
use core::str::FromStr;
use serde::{
    de::{self, Visitor},
    Deserialize, Deserializer, Serialize, Serializer,
};

mod date {

    use super::*;
    impl<'de> Deserialize<'de> for crate::Date {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            deserializer.deserialize_str(DateVisitor)
        }
    }

    impl Serialize for crate::Date {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            serializer.serialize_str(&self.to_string())
        }
    }

    struct DateVisitor;
    impl<'de> Visitor<'de> for DateVisitor {
        type Value = crate::Date;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            write!(formatter, "a string that follows iso8601 date format")
        }

        fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            match Self::Value::from_str(s) {
                Ok(p) => Ok(p),
                Err(_) => Err(de::Error::invalid_value(de::Unexpected::Str(s), &self)),
            }
        }
    }

    #[test]
    fn serialize() {
        let date_json = r#""2023-02-10""#;
        let date = crate::date("2023-02-10").unwrap();

        let serialized_date = serde_json::to_string(&date).unwrap();

        assert_eq!(serialized_date, date_json);
    }

    #[test]
    fn deserialize() {
        let date_json = r#""2023-02-10""#;
        let date = crate::date("2023-02-10").unwrap();

        let deserialized_date = serde_json::from_str::<crate::Date>(date_json).unwrap();

        assert_eq!(deserialized_date, date);
    }
}

mod time {
    use super::*;

    impl<'de> Deserialize<'de> for crate::Time {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            deserializer.deserialize_str(TimeVisitor)
        }
    }

    impl Serialize for crate::Time {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            serializer.serialize_str(&self.to_string())
        }
    }

    struct TimeVisitor;
    impl<'de> Visitor<'de> for TimeVisitor {
        type Value = crate::Time;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            write!(formatter, "a string that follows iso8601 time format")
        }

        fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            match Self::Value::from_str(s) {
                Ok(p) => Ok(p),
                Err(_) => Err(de::Error::invalid_value(de::Unexpected::Str(s), &self)),
            }
        }
    }

    #[test]
    fn serialize() {
        let time_json = r#""18:12:15.0+00:00""#;

        let deserialized_time = serde_json::from_str::<crate::Time>(time_json).unwrap();
        let serialized_time = serde_json::to_string(&deserialized_time).unwrap();

        assert_eq!(serialized_time, time_json);
    }

    #[test]
    fn deserialize() {
        let time_json = r#""18:12:15""#;

        let expected_time = crate::time("18:12:15").unwrap();
        let deserialized_time = serde_json::from_str::<crate::Time>(time_json).unwrap();

        assert_eq!(deserialized_time, expected_time);
    }
}

mod datetime {
    use super::*;
    impl<'de> Deserialize<'de> for crate::DateTime {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            deserializer.deserialize_str(DateTimeVisitor)
        }
    }

    impl Serialize for crate::DateTime {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            serializer.serialize_str(&self.to_string())
        }
    }

    struct DateTimeVisitor;
    impl<'de> Visitor<'de> for DateTimeVisitor {
        type Value = crate::DateTime;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            write!(formatter, "a string that follows iso8601 Datetime format")
        }

        fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            match Self::Value::from_str(s) {
                Ok(p) => Ok(p),
                Err(_) => Err(de::Error::invalid_value(de::Unexpected::Str(s), &self)),
            }
        }
    }

    #[test]
    fn serialize() {
        let datetime_json = r#""2023-02-10T18:12:15.0+00:00""#;
        let datetime = crate::datetime("2023-02-10T18:12:15.0+00:00").unwrap();

        let serialized_datetime = serde_json::to_string(&datetime).unwrap();

        assert_eq!(serialized_datetime, datetime_json);
    }

    #[test]
    fn deserialize() {
        let datetime_json = r#""2023-02-10T18:12:15""#;
        let datetime = crate::datetime("2023-02-10T18:12:15").unwrap();

        let deserialized_datetime = serde_json::from_str::<crate::DateTime>(datetime_json).unwrap();

        assert_eq!(deserialized_datetime, datetime);
    }

    #[test]
    fn deserialize_short() {
        let datetime_json = r#""2023-02-10T18:12""#;
        let datetime = crate::datetime("2023-02-10T18:12").unwrap();

        let deserialized_datetime = serde_json::from_str::<crate::DateTime>(datetime_json).unwrap();

        assert_eq!(deserialized_datetime, datetime);
    }
}

mod duration {
    use super::*;

    impl<'de> Deserialize<'de> for crate::Duration {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            deserializer.deserialize_str(DurationVisitor)
        }
    }

    impl Serialize for crate::Duration {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            serializer.serialize_str(&self.to_string())
        }
    }

    struct DurationVisitor;
    impl<'de> Visitor<'de> for DurationVisitor {
        type Value = crate::Duration;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            write!(formatter, "a string that follows iso8601 Duration format")
        }

        fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            match Self::Value::from_str(s) {
                Ok(p) => Ok(p),
                Err(_) => Err(de::Error::invalid_value(de::Unexpected::Str(s), &self)),
            }
        }
    }

    #[test]
    fn serialize() {
        let duration_json = r#""P1Y2M3DT4H5M6S""#;
        let duration = crate::duration("P1Y2M3DT4H5M6S").unwrap();

        let serialized_duration = serde_json::to_string(&duration).unwrap();

        assert_eq!(serialized_duration, duration_json);
    }

    #[test]
    fn deserialize() {
        let duration_json = r#""P1Y2M3DT4H5M6S""#;
        let duration = crate::duration("P1Y2M3DT4H5M6S").unwrap();

        let deserialized_duration = serde_json::from_str::<crate::Duration>(duration_json).unwrap();

        assert_eq!(deserialized_duration, duration);
    }
}
