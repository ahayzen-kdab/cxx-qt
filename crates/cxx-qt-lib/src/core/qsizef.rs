// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use cxx::{type_id, ExternType};
use std::fmt;

#[cxx::bridge]
mod ffi {
    #[namespace = "Qt"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/qt.h");
        type AspectRatioMode = crate::AspectRatioMode;
    }

    unsafe extern "C++" {
        include!("cxx-qt-lib/qmarginsf.h");
        type QMarginsF = crate::QMarginsF;
        include!("cxx-qt-lib/qsize.h");
        type QSize = crate::QSize;
        include!("cxx-qt-lib/qsizef.h");
        type QSizeF = super::QSizeF;
        include!("cxx-qt-lib/qstring.h");
        type QString = crate::QString;

        /// Returns a size holding the minimum width and height of this size and the given otherSize.
        #[rust_name = "bounded_to"]
        fn boundedTo(self: &QSizeF, other_size: &QSizeF) -> QSizeF;

        /// Returns a size holding the maximum width and height of this size and the given otherSize.
        #[rust_name = "expanded_to"]
        fn expandedTo(self: &QSizeF, other_size: &QSizeF) -> QSizeF;

        /// Returns the height.
        fn height(self: &QSizeF) -> f64;

        /// Returns true if either of the width and height is less than or equal to 0; otherwise returns false.
        #[rust_name = "is_empty"]
        fn isEmpty(self: &QSizeF) -> bool;

        /// Returns true if both the width and height are 0.0 (ignoring the sign); otherwise returns false.
        #[rust_name = "is_null"]
        fn isNull(self: &QSizeF) -> bool;

        /// Returns true if both the width and height are equal to or greater than 0; otherwise returns false.
        #[rust_name = "is_valid"]
        fn isValid(self: &QSizeF) -> bool;

        /// Returns the size that results from growing this size by margins.
        #[rust_name = "grown_by"]
        fn grownBy(self: &QSizeF, margins: QMarginsF) -> QSizeF;

        /// Scales the size to a rectangle with the given size, according to the specified mode.
        fn scale(self: &mut QSizeF, size: &QSizeF, mode: AspectRatioMode);

        /// Returns a size scaled to a rectangle with the given size s, according to the specified mode.
        fn scaled(self: &QSizeF, s: &QSizeF, mode: AspectRatioMode) -> QSizeF;

        /// Sets the height to the given finite height.
        #[rust_name = "set_height"]
        fn setHeight(self: &mut QSizeF, height: f64);

        /// Sets the width to the given finite width.
        #[rust_name = "set_width"]
        fn setWidth(self: &mut QSizeF, width: f64);

        /// Returns the size that results from shrinking this size by margins.
        #[rust_name = "shrunk_by"]
        fn shrunkBy(self: &QSizeF, margins: QMarginsF) -> QSizeF;

        /// Returns an integer based copy of this size.
        ///
        /// Note that the coordinates in the returned size will be rounded to the nearest integer.
        #[rust_name = "to_size"]
        fn toSize(self: &QSizeF) -> QSize;

        /// Swaps the width and height values.
        fn transpose(self: &mut QSizeF);

        /// Returns the size with width and height values swapped.
        fn transposed(self: &QSizeF) -> QSizeF;

        /// Returns the width.
        fn width(self: &QSizeF) -> f64;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[doc(hidden)]
        #[rust_name = "qsizef_init_default"]
        fn construct() -> QSizeF;
        #[doc(hidden)]
        #[rust_name = "qsizef_init"]
        fn construct(w: f64, h: f64) -> QSizeF;
        #[doc(hidden)]
        #[rust_name = "qsizef_from_qsize"]
        fn construct(size: &QSize) -> QSizeF;
        #[doc(hidden)]
        #[rust_name = "qsizef_to_qstring"]
        fn toQString(value: &QSizeF) -> QString;
    }
}

/// The QSizeF class defines the size of a two-dimensional object using floating point precision.
#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct QSizeF {
    width: f64,
    height: f64,
}

impl QSizeF {
    /// Constructs a size with the given width and height.
    pub fn new(w: f64, h: f64) -> Self {
        ffi::qsizef_init(w, h)
    }
}

impl Default for QSizeF {
    /// Constructs an invalid size.
    fn default() -> Self {
        ffi::qsizef_init_default()
    }
}

impl fmt::Display for QSizeF {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", ffi::qsizef_to_qstring(self))
    }
}

impl From<&ffi::QSize> for QSizeF {
    /// Constructs a size with floating point accuracy from the given size.
    fn from(size: &ffi::QSize) -> Self {
        ffi::qsizef_from_qsize(size)
    }
}

impl From<QSizeF> for ffi::QSize {
    /// Returns an integer based copy of this size.
    ///
    /// Note that the coordinates in the returned size will be rounded to the nearest integer.
    fn from(size: QSizeF) -> Self {
        size.to_size()
    }
}

// Safety:
//
// Static checks on the C++ side ensure that QSizeF is trivial.
unsafe impl ExternType for QSizeF {
    type Id = type_id!("QSizeF");
    type Kind = cxx::kind::Trivial;
}

#[cfg(feature = "serde")]
use serde::ser::SerializeMap;

#[cfg(feature = "serde")]
struct QSizeFVisitor;

#[cfg(feature = "serde")]
impl<'de> serde::de::Visitor<'de> for QSizeFVisitor {
    type Value = QSizeF;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("QSizeF")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let mut width = None;
        let mut height = None;

        while let Some((key, value)) = map.next_entry()? {
            match key {
                "width" => width = Some(value),
                "height" => height = Some(value),
                others => {
                    return Err(serde::de::Error::invalid_value(
                        serde::de::Unexpected::Str(others),
                        &"expected either width or height as a key",
                    ));
                }
            }
        }

        if let (Some(width), Some(height)) = (width, height) {
            Ok(QSizeF::new(width, height))
        } else {
            Err(serde::de::Error::missing_field(
                "missing width or height as key",
            ))
        }
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QSizeF {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_map(QSizeFVisitor)
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for QSizeF {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut map = serializer.serialize_map(Some(2))?;
        map.serialize_entry("width", &self.width())?;
        map.serialize_entry("height", &self.height())?;
        map.end()
    }
}

#[cfg(feature = "serde")]
#[cfg(test)]
mod serde_tests {
    use super::*;

    #[test]
    fn test_serde_deserialize() {
        let test_data: QSizeF = serde_json::from_str(r#"{"width":1.0,"height":2.0}"#).unwrap();
        assert_eq!(test_data, QSizeF::new(1.0, 2.0));
    }

    #[test]
    fn test_serde_serialize() {
        let test_data = QSizeF::new(1.0, 2.0);
        let data_string = serde_json::to_string(&test_data).unwrap();
        assert_eq!(data_string, r#"{"width":1.0,"height":2.0}"#);
    }
}
