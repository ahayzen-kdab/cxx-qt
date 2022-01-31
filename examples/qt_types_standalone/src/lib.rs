// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use core::pin::Pin;
use cxx_qt_lib::{
    let_qstring, Color, MapQtValue, QColor, QPoint, QPointF, QRect, QRectF, QSize, QSizeF, QString,
    QVariant, Variant, VariantEnum,
};

#[cxx::bridge]
mod ffi {
    enum ColorTest {
        Rgb_Red,
        Rgb_Green,
        Rgb_Blue,
        Rgb_Transparent,
    }

    enum VariantTest {
        Bool,
        F32,
        F64,
        I8,
        I16,
        I32,
        String,
        U8,
        U16,
        U32,
    }

    unsafe extern "C++" {
        include!("cxx-qt-gen/statics/rust/cxx_qt.h");
        include!("bridge.h");

        type QColor = cxx_qt_lib::QColor;
        type QString = cxx_qt_lib::QString;
        type QVariant = cxx_qt_lib::QVariant;
        type QSize = cxx_qt_lib::QSize;
        type QSizeF = cxx_qt_lib::QSizeF;
        type QPoint = cxx_qt_lib::QPoint;
        type QPointF = cxx_qt_lib::QPointF;
        type QRectF = cxx_qt_lib::QRectF;
        type QRect = cxx_qt_lib::QRect;

        fn test_constructed_qstring(s: &QString) -> bool;
        fn assign_to_qstring(s: Pin<&mut QString>, v: &QString);

        fn test_constructed_qcolor(c: &QColor, test: ColorTest) -> bool;

        fn test_constructed_qvariant(s: &QVariant, test: VariantTest) -> bool;
    }

    extern "Rust" {
        fn can_construct_qstring(slice: bool) -> bool;
        fn can_read_qstring(s: &QString) -> bool;
        fn modify_qstring(s: Pin<&mut QString>);
        fn can_map_to_qstring() -> bool;
        fn can_handle_qstring_change() -> bool;

        fn make_color(test: ColorTest) -> UniquePtr<QColor>;
        fn can_construct_qcolor(test: ColorTest) -> bool;
        fn can_read_qcolor(c: &QColor, test: ColorTest) -> bool;

        fn make_variant(test: VariantTest) -> UniquePtr<QVariant>;
        fn can_construct_qvariant(test: VariantTest) -> bool;
        fn can_read_qvariant(v: &QVariant, test: VariantTest) -> bool;

        fn construct_qpoint() -> QPoint;
        fn read_qpoint(p: &QPoint) -> bool;
        fn copy_qpoint(p: &QPoint) -> QPoint;
        fn copy_value_qpoint(p: QPoint) -> QPoint;

        fn construct_qpointf() -> QPointF;
        fn read_qpointf(p: &QPointF) -> bool;
        fn copy_qpointf(p: &QPointF) -> QPointF;
        fn copy_value_qpointf(p: QPointF) -> QPointF;

        fn construct_qrect() -> QRect;
        fn read_qrect(p: &QRect) -> bool;
        fn copy_qrect(p: &QRect) -> QRect;
        fn copy_value_qrect(p: QRect) -> QRect;

        fn construct_qrectf() -> QRectF;
        fn read_qrectf(p: &QRectF) -> bool;
        fn copy_qrectf(p: &QRectF) -> QRectF;
        fn copy_value_qrectf(p: QRectF) -> QRectF;

        fn construct_qsize() -> QSize;
        fn read_qsize(p: &QSize) -> bool;
        fn copy_qsize(p: &QSize) -> QSize;
        fn copy_value_qsize(p: QSize) -> QSize;

        fn construct_qsizef() -> QSizeF;
        fn read_qsizef(p: &QSizeF) -> bool;
        fn copy_qsizef(p: &QSizeF) -> QSizeF;
        fn copy_value_qsizef(p: QSizeF) -> QSizeF;
    }
}

use ffi::ColorTest;
use ffi::VariantTest;

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

fn make_color(test: ColorTest) -> cxx::UniquePtr<QColor> {
    match test {
        ColorTest::Rgb_Red => Color::from_rgba(255, 0, 0, 255).to_unique_ptr(),
        ColorTest::Rgb_Green => Color::from_rgba(0, 255, 0, 255).to_unique_ptr(),
        ColorTest::Rgb_Blue => Color::from_rgba(0, 0, 255, 255).to_unique_ptr(),
        ColorTest::Rgb_Transparent => Color::from_rgba(0, 0, 0, 0).to_unique_ptr(),
        _others => panic!("Unsupported test: {}", test.repr),
    }
}

fn can_construct_qcolor(test: ColorTest) -> bool {
    let color = make_color(test);
    ffi::test_constructed_qcolor(&color, test)
}

fn can_read_qcolor(c: &QColor, test: ColorTest) -> bool {
    let color = c.to_rust();
    match test {
        ColorTest::Rgb_Red => {
            color.alpha() == 255 && color.red() == 255 && color.green() == 0 && color.blue() == 0
        }
        ColorTest::Rgb_Green => {
            color.alpha() == 255 && color.red() == 0 && color.green() == 255 && color.blue() == 0
        }
        ColorTest::Rgb_Blue => {
            color.alpha() == 255 && color.red() == 0 && color.green() == 0 && color.blue() == 255
        }
        ColorTest::Rgb_Transparent => {
            color.alpha() == 0 && color.red() == 0 && color.green() == 0 && color.blue() == 0
        }
        _others => panic!("Unsupported test: {}", test.repr),
    }
}

fn make_variant(test: VariantTest) -> cxx::UniquePtr<QVariant> {
    match test {
        VariantTest::Bool => Variant::from_bool(true).to_unique_ptr(),
        VariantTest::F32 => Variant::from_f32(1.23).to_unique_ptr(),
        VariantTest::F64 => Variant::from_f64(1.23).to_unique_ptr(),
        VariantTest::I8 => Variant::from_i8(12).to_unique_ptr(),
        VariantTest::I16 => Variant::from_i16(123).to_unique_ptr(),
        VariantTest::I32 => Variant::from_i32(123).to_unique_ptr(),
        VariantTest::String => Variant::from_string("Rust string".to_owned()).to_unique_ptr(),
        VariantTest::U8 => Variant::from_u8(12).to_unique_ptr(),
        VariantTest::U16 => Variant::from_u16(123).to_unique_ptr(),
        VariantTest::U32 => Variant::from_u32(123).to_unique_ptr(),
        _others => panic!("Unsupported test: {}", test.repr),
    }
}

fn can_construct_qvariant(test: VariantTest) -> bool {
    let variant = make_variant(test);
    ffi::test_constructed_qvariant(&variant, test)
}

fn can_read_qvariant(v: &QVariant, test: VariantTest) -> bool {
    let variant = v.to_rust().value();
    match test {
        VariantTest::Bool => match variant {
            VariantEnum::Bool(b) => !b,
            _others => false,
        },
        VariantTest::F32 => match variant {
            VariantEnum::F32(f) => f == 89.1,
            _others => false,
        },
        VariantTest::F64 => match variant {
            VariantEnum::F64(f) => f == 89.1,
            _others => false,
        },
        VariantTest::I8 => match variant {
            VariantEnum::I8(i) => i == 89,
            _others => false,
        },
        VariantTest::I16 => match variant {
            VariantEnum::I16(i) => i == 8910,
            _others => false,
        },
        VariantTest::I32 => match variant {
            VariantEnum::I32(i) => i == 8910,
            _others => false,
        },
        VariantTest::String => match variant {
            VariantEnum::String(s) => s == "C++ string",
            _others => false,
        },
        VariantTest::U8 => match variant {
            VariantEnum::U8(i) => i == 89,
            _others => false,
        },
        VariantTest::U16 => match variant {
            VariantEnum::U16(i) => i == 8910,
            _others => false,
        },
        VariantTest::U32 => match variant {
            VariantEnum::U32(i) => i == 8910,
            _others => false,
        },
        _others => panic!("Unsupported test: {}", test.repr),
    }
}

fn construct_qpoint() -> QPoint {
    QPoint::new(2, 4)
}

fn read_qpoint(p: &QPoint) -> bool {
    p.x() == 2 && p.y() == 4
}

fn copy_qpoint(p: &QPoint) -> QPoint {
    *p
}

fn copy_value_qpoint(p: QPoint) -> QPoint {
    p
}

fn construct_qpointf() -> QPointF {
    QPointF::new(1.23, 4.56)
}

fn read_qpointf(p: &QPointF) -> bool {
    ((p.x() - 1.23).abs() < f64::EPSILON) && ((p.y() - 4.56).abs() < f64::EPSILON)
}

fn copy_qpointf(p: &QPointF) -> QPointF {
    *p
}

fn copy_value_qpointf(p: QPointF) -> QPointF {
    p
}

fn construct_qrect() -> QRect {
    QRect::new(1, 4, 2, 8)
}

fn read_qrect(r: &QRect) -> bool {
    r.x() == 1 && r.y() == 4 && r.width() == 2 && r.height() == 8
}

fn copy_qrect(r: &QRect) -> QRect {
    *r
}

fn copy_value_qrect(r: QRect) -> QRect {
    r
}

fn construct_qrectf() -> QRectF {
    QRectF::new(1.23, 4.56, 2.46, 9.12)
}

fn read_qrectf(p: &QRectF) -> bool {
    ((p.x() - 1.23).abs() < f64::EPSILON)
        && ((p.y() - 4.56).abs() < f64::EPSILON)
        && ((p.width() - 2.46).abs() < f64::EPSILON)
        && ((p.height() - 9.12).abs() < f64::EPSILON)
}

fn copy_qrectf(p: &QRectF) -> QRectF {
    *p
}

fn copy_value_qrectf(p: QRectF) -> QRectF {
    p
}

fn construct_qsize() -> QSize {
    QSize::new(1, 4)
}

fn read_qsize(s: &QSize) -> bool {
    s.width() == 1 && s.height() == 4
}

fn copy_qsize(s: &QSize) -> QSize {
    *s
}

fn copy_value_qsize(s: QSize) -> QSize {
    s
}

fn construct_qsizef() -> QSizeF {
    QSizeF::new(1.23, 4.56)
}

fn read_qsizef(s: &QSizeF) -> bool {
    ((s.width() - 1.23).abs() < f64::EPSILON) && ((s.height() - 4.56).abs() < f64::EPSILON)
}

fn copy_qsizef(s: &QSizeF) -> QSizeF {
    *s
}

fn copy_value_qsizef(s: QSizeF) -> QSizeF {
    s
}
