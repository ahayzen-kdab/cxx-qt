// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use cxx::{type_id, ExternType};

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = crate::QString;
        include!("cxx-qt-lib/qvariant.h");
        type QVariant = crate::QVariant;

        include!("cxx-qt-lib/qmap.h");
        type QVariantMap = crate::QMap<super::QMapPair_QString_QVariant>;
    }

    unsafe extern "C++" {
        #[rust_name = "cxx_clear"]
        fn clear(self: &mut QVariantMap);
        #[rust_name = "cxx_contains"]
        fn contains(self: &QVariantMap, key: &QString) -> bool;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "qmap_clone_QString_QVariant"]
        fn construct(_: &QVariantMap) -> QVariantMap;
        #[rust_name = "qmap_default_QString_QVariant"]
        fn construct() -> QVariantMap;
        #[rust_name = "qmap_drop_QString_QVariant"]
        fn drop(_: &mut QVariantMap);
    }

    #[namespace = "rust::cxxqtlib1::qmap"]
    unsafe extern "C++" {
        #[rust_name = "get_or_default_QString_QVariant"]
        fn qmapGetOrDefault(_: &QVariantMap, key: &QString) -> QVariant;
        #[rust_name = "get_unchecked_key_QString_QVariant"]
        unsafe fn qmapGetUncheckedKey(_: &QVariantMap, pos: isize) -> &QString;
        #[rust_name = "get_unchecked_value_QString_QVariant"]
        unsafe fn qmapGetUncheckedValue(_: &QVariantMap, pos: isize) -> &QVariant;
        #[rust_name = "insert_QString_QVariant"]
        fn qmapInsert(_: &mut QVariantMap, key: &QString, value: &QVariant);
        #[rust_name = "len_QString_QVariant"]
        fn qmapLen(_: &QVariantMap) -> isize;
        #[rust_name = "remove_QString_QVariant"]
        fn qmapRemove(_: &mut QVariantMap, key: &QString) -> bool;
    }
}

pub(crate) fn clone(map: &ffi::QVariantMap) -> ffi::QVariantMap {
    ffi::qmap_clone_QString_QVariant(map)
}

pub(crate) fn default() -> ffi::QVariantMap {
    ffi::qmap_default_QString_QVariant()
}

pub(crate) fn drop(map: &mut ffi::QVariantMap) {
    ffi::qmap_drop_QString_QVariant(map);
}

pub(crate) fn get_or_default(
    map: &ffi::QVariantMap,
    key: &ffi::QString,
) -> ffi::QVariant {
    ffi::get_or_default_QString_QVariant(map, key)
}

pub(crate) unsafe fn get_unchecked_key(
    map: &ffi::QVariantMap,
    pos: isize,
) -> &ffi::QString {
    ffi::get_unchecked_key_QString_QVariant(map, pos)
}

pub(crate) unsafe fn get_unchecked_value(
    map: &ffi::QVariantMap,
    pos: isize,
) -> &ffi::QVariant {
    ffi::get_unchecked_value_QString_QVariant(map, pos)
}

pub(crate) fn insert(
    map: &mut ffi::QVariantMap,
    key: &ffi::QString,
    value: &ffi::QVariant,
) {
    ffi::insert_QString_QVariant(map, key, value);
}

pub(crate) fn len(map: &ffi::QVariantMap) -> isize {
    ffi::len_QString_QVariant(map)
}

pub(crate) fn remove(map: &mut ffi::QVariantMap, key: &ffi::QString) -> bool {
    ffi::remove_QString_QVariant(map, key)
}

#[allow(non_camel_case_types)]
pub struct QMapPair_QString_QVariant;

unsafe impl ExternType for QMapPair_QString_QVariant {
    type Id = type_id!("QMapPair_QString_QVariant");
    type Kind = cxx::kind::Trivial;
}
