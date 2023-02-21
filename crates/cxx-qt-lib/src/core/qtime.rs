// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use cxx::{type_id, ExternType};
use std::fmt;

#[cxx::bridge]
mod ffi {
    #[namespace = "Qt"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/qt.h");
        type DateFormat = crate::DateFormat;
    }

    unsafe extern "C++" {
        include!("cxx-qt-lib/qtime.h");
        include!("cxx-qt-lib/qstring.h");

        type QTime = super::QTime;
        type QString = crate::QString;

        /// Returns a QTime object containing a time ms milliseconds later
        /// than the time of this object (or earlier if ms is negative).
        #[rust_name = "add_msecs"]
        fn addMSecs(self: &QTime, ms: i32) -> QTime;

        /// Returns a QTime object containing a time s seconds later than the
        /// time of this object (or earlier if s is negative).
        #[rust_name = "add_secs"]
        fn addSecs(self: &QTime, s: i32) -> QTime;

        /// Returns the hour part (0 to 23) of the time.
        fn hour(self: &QTime) -> i32;

        /// Returns true if the time is null (i.e., the QTime object was
        /// constructed using the default constructor); otherwise returns false.
        /// A null time is also an invalid time.
        #[rust_name = "is_null"]
        fn isNull(self: &QTime) -> bool;

        /// Returns true if the time is valid; otherwise returns false.
        #[rust_name = "is_valid"]
        fn isValid(self: &QTime) -> bool;

        /// Returns the minute part (0 to 59) of the time.
        fn minute(self: &QTime) -> i32;

        /// Returns the millisecond part (0 to 999) of the time.
        fn msec(self: &QTime) -> i32;

        /// Returns the number of msecs since the start of the day, i.e. since 00:00:00.
        #[rust_name = "msecs_since_start_of_day"]
        fn msecsSinceStartOfDay(self: &QTime) -> i32;

        /// Returns the second part (0 to 59) of the time.
        fn second(self: &QTime) -> i32;

        /// Sets the time to hour h, minute m, seconds s and milliseconds ms.
        #[rust_name = "set_hms"]
        fn setHMS(self: &mut QTime, h: i32, m: i32, s: i32, ms: i32) -> bool;

        /// Returns the time as a string. The format parameter determines the format of the result string.
        #[rust_name = "format"]
        fn toString(self: &QTime, format: &QString) -> QString;

        /// Returns the time as a string. The format parameter determines the format of the string.
        #[rust_name = "format_enum"]
        fn toString(self: &QTime, format: DateFormat) -> QString;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        #[doc(hidden)]
        #[rust_name = "qtime_current_time"]
        fn qtimeCurrentTime() -> QTime;

        #[doc(hidden)]
        #[rust_name = "qtime_from_msecs_since_start_of_day"]
        fn qtimeFromMSecsSinceStartOfDay(msecs: i32) -> QTime;

        #[doc(hidden)]
        #[rust_name = "qtime_from_string"]
        fn qtimeFromString(string: &QString, format: &QString) -> QTime;
        #[doc(hidden)]
        #[rust_name = "qtime_from_string_enum"]
        fn qtimeFromString(string: &QString, format: DateFormat) -> QTime;

        #[doc(hidden)]
        #[rust_name = "qtime_msecs_to"]
        fn qtimeMSecsTo(time: &QTime, t: QTime) -> i32;

        #[doc(hidden)]
        #[rust_name = "qtime_secs_to"]
        fn qtimeSecsTo(time: &QTime, t: QTime) -> i32;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[doc(hidden)]
        #[rust_name = "qtime_init_default"]
        fn construct() -> QTime;
        #[doc(hidden)]
        #[rust_name = "qtime_init"]
        fn construct(h: i32, m: i32, s: i32, ms: i32) -> QTime;
        #[doc(hidden)]
        #[rust_name = "qtime_to_qstring"]
        fn toQString(value: &QTime) -> QString;
    }
}

/// The QTime class provides clock time functions.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
#[repr(C)]
pub struct QTime {
    mds: i32,
}

impl QTime {
    /// Returns the current time as reported by the system clock.
    pub fn current_time() -> Self {
        ffi::qtime_current_time()
    }

    /// Returns a new QTime instance with the time set to the number of msecs
    /// since the start of the day, i.e. since 00:00:00.
    pub fn from_msecs_since_start_of_day(msecs: i32) -> Self {
        ffi::qtime_from_msecs_since_start_of_day(msecs)
    }

    /// Returns the QTime represented by the string, using the format given, or an invalid time if the string cannot be parsed.
    pub fn from_string(string: &ffi::QString, format: &ffi::QString) -> Self {
        ffi::qtime_from_string(string, format)
    }

    /// Returns the time represented in the string as a QTime using the format given, or an invalid time if this is not possible.
    pub fn from_string_enum(string: &ffi::QString, format: ffi::DateFormat) -> Self {
        ffi::qtime_from_string_enum(string, format)
    }

    /// Returns the number of milliseconds from this time to t.
    /// If t is earlier than this time, the number of milliseconds returned is negative.
    pub fn msecs_to(&self, t: Self) -> i32 {
        ffi::qtime_msecs_to(self, t)
    }

    /// Constructs a time with hour h, minute m, seconds s and milliseconds ms.
    pub fn new(h: i32, m: i32, s: i32, ms: i32) -> Self {
        ffi::qtime_init(h, m, s, ms)
    }

    /// Returns the number of seconds from this time to t.
    /// If t is earlier than this time, the number of seconds returned is negative.
    pub fn secs_to(&self, t: Self) -> i32 {
        ffi::qtime_secs_to(self, t)
    }
}

impl Default for QTime {
    /// Constructs a null time object.
    fn default() -> Self {
        ffi::qtime_init_default()
    }
}

impl fmt::Display for QTime {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", ffi::qtime_to_qstring(self))
    }
}

impl fmt::Debug for QTime {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{self}")
    }
}

impl From<i32> for QTime {
    /// Returns a new QTime instance with the time set to the number of msecs
    /// since the start of the day, i.e. since 00:00:00.
    fn from(msecs: i32) -> Self {
        ffi::qtime_from_msecs_since_start_of_day(msecs)
    }
}

impl From<&QTime> for i32 {
    /// Returns the number of msecs since the start of the day, i.e. since 00:00:00.
    fn from(time: &QTime) -> Self {
        time.msecs_since_start_of_day()
    }
}

// Safety:
//
// Static checks on the C++ side ensure that QTime is trivial.
unsafe impl ExternType for QTime {
    type Id = type_id!("QTime");
    type Kind = cxx::kind::Trivial;
}

#[cfg(feature = "serde")]
use serde::ser::SerializeMap;

#[cfg(feature = "serde")]
struct QTimeVisitor;

#[cfg(feature = "serde")]
impl<'de> serde::de::Visitor<'de> for QTimeVisitor {
    type Value = QTime;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("QTime")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let mut hour = None;
        let mut minute = None;
        let mut second = None;
        let mut msec = None;

        while let Some((key, value)) = map.next_entry()? {
            match key {
                "hour" => hour = Some(value),
                "minute" => minute = Some(value),
                "second" => second = Some(value),
                "msec" => msec = Some(value),
                others => {
                    return Err(serde::de::Error::invalid_value(
                        serde::de::Unexpected::Str(others),
                        &"expected either hour, minute, second, or msec as a key",
                    ));
                }
            }
        }

        if let (Some(hour), Some(minute), Some(second), Some(msec)) = (hour, minute, second, msec) {
            Ok(QTime::new(hour, minute, second, msec))
        } else {
            Err(serde::de::Error::missing_field(
                "missing hour, minute, second, or msec as key",
            ))
        }
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QTime {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_map(QTimeVisitor)
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for QTime {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut map = serializer.serialize_map(Some(4))?;
        map.serialize_entry("hour", &self.hour())?;
        map.serialize_entry("minute", &self.minute())?;
        map.serialize_entry("second", &self.second())?;
        map.serialize_entry("msec", &self.msec())?;
        map.end()
    }
}

#[cfg(feature = "serde")]
#[cfg(test)]
mod serde_tests {
    use super::*;

    #[test]
    fn test_serde_deserialize() {
        let test_data: QTime =
            serde_json::from_str(r#"{"hour":1,"minute":2,"second":3,"msec":4}"#).unwrap();
        assert_eq!(test_data, QTime::new(1, 2, 3, 4));
    }

    #[test]
    fn test_serde_serialize() {
        let test_data = QTime::new(1, 2, 3, 4);
        let data_string = serde_json::to_string(&test_data).unwrap();
        assert_eq!(data_string, r#"{"hour":1,"minute":2,"second":3,"msec":4}"#);
    }
}
