// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qvariant.h");
        type QVariant = crate::QVariant;
    }

    #[namespace = "rust::cxxqtlib1::qvariant"]
    unsafe extern "C++" {
        #[rust_name = "can_convert_f64"]
        fn qvariantCanConvertF64(variant: &QVariant) -> bool;
        #[rust_name = "construct_f64"]
        fn qvariantConstruct(value: &f64) -> QVariant;
        #[rust_name = "value_f64"]
        fn qvariantValue(variant: &QVariant) -> f64;
    }
}

pub(crate) fn can_convert(variant: &ffi::QVariant) -> bool {
    ffi::can_convert_f64(variant)
}

pub(crate) fn construct(value: &f64) -> ffi::QVariant {
    ffi::construct_f64(value)
}

pub(crate) fn value(variant: &ffi::QVariant) -> f64 {
    ffi::value_f64(variant)
}