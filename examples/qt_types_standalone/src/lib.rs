// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use core::pin::Pin;
use cxx_qt_lib::{let_qstring, MapQtValue, QPointF, QSizeF, QString};

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("bridge.h");

        type QString = cxx_qt_lib::QString;
        type QSizeF = cxx_qt_lib::QSizeF;
        type QPointF = cxx_qt_lib::QPointF;

        fn test_constructed_qstring(s: &QString) -> bool;
        fn assign_to_qstring(s: Pin<&mut QString>, v: &QString);
    }

    extern "Rust" {
        fn can_construct_qstring(slice: bool) -> bool;
        fn can_read_qstring(s: &QString) -> bool;
        fn modify_qstring(s: Pin<&mut QString>);
        fn can_map_to_qstring() -> bool;
        fn can_handle_qstring_change() -> bool;

        fn construct_qpointf() -> QPointF;
        fn read_qpointf(p: &QPointF) -> bool;
        fn copy_qpointf(p: &QPointF) -> QPointF;
        fn copy_value_qpointf(p: QPointF) -> QPointF;

        fn construct_qsizef() -> QSizeF;
        fn read_qsizef(p: &QSizeF) -> bool;
        fn copy_qsizef(p: &QSizeF) -> QSizeF;
        fn copy_value_qsizef(p: QSizeF) -> QSizeF;
    }
}

fn can_construct_qstring(slice: bool) -> bool {
    if slice {
        let_qstring!(s = "String constructed by Rust");
        ffi::test_constructed_qstring(&s)
    } else {
        let rs_string = "String constructed by Rust".to_owned();
        let_qstring!(s = rs_string);
        ffi::test_constructed_qstring(&s)
    }
}

fn can_read_qstring(s: &QString) -> bool {
    let rs = s.to_rust();
    rs == "String constructed by C++"
}

fn modify_qstring(s: Pin<&mut QString>) {
    let_qstring!(v = "Updated string value");
    ffi::assign_to_qstring(s, &v);
}

fn can_map_to_qstring() -> bool {
    "String constructed by Rust".map_qt_value(
        |_, converted| ffi::test_constructed_qstring(converted),
        &mut (),
    )
}

fn can_handle_qstring_change() -> bool {
    let long_s = "Very very long string that is hopefully long enough to allocate and get Valgrind's attention :)";

    let_qstring!(s = "Short string");
    let_qstring!(v = long_s);
    ffi::assign_to_qstring(s.as_mut(), &v);

    let rs = s.to_rust();
    rs == long_s
}

fn construct_qpointf() -> QPointF {
    QPointF::new(1.23, 4.56)
}

fn read_qpointf(p: &QPointF) -> bool {
    ((p.x - 1.23).abs() < f64::EPSILON) && ((p.y - 4.56).abs() < f64::EPSILON)
}

fn copy_qpointf(p: &QPointF) -> QPointF {
    *p
}

fn copy_value_qpointf(p: QPointF) -> QPointF {
    p
}

fn construct_qsizef() -> QSizeF {
    QSizeF::new(1.23, 4.56)
}

fn read_qsizef(s: &QSizeF) -> bool {
    ((s.w - 1.23).abs() < f64::EPSILON) && ((s.h - 4.56).abs() < f64::EPSILON)
}

fn copy_qsizef(s: &QSizeF) -> QSizeF {
    *s
}

fn copy_value_qsizef(s: QSizeF) -> QSizeF {
    s
}
