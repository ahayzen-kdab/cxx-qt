// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Be Wilson <be.wilson@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

// ANCHOR: book_cxx_qt_module
// ANCHOR: book_bridge_macro

#[cxx_qt::bridge]
mod my_object {
    // ANCHOR_END: book_bridge_macro

    // ANCHOR: book_qstring_import
    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;

        include!("cxx-qt-lib/qmap.h");
        type QVariantMap = cxx_qt_lib::QMap<cxx_qt_lib::QMapPair_QString_QVariant>;

        include!("cxx-qt-lib/qvariant.h");
        type QVariant = cxx_qt_lib::QVariant;
    }
    // ANCHOR_END: book_qstring_import

    // ANCHOR: book_rustobj_struct
    #[cxx_qt::qobject(qml_uri = "com.kdab.cxx_qt.demo", qml_version = "1.0")]
    pub struct MyObject {
        #[qproperty]
        number: i32,
        #[qproperty]
        string: QString,
        #[qproperty]
        test: QVariantMap,
    }
    // ANCHOR_END: book_rustobj_struct

    // ANCHOR: book_rustobj_default
    impl Default for MyObject {
        fn default() -> Self {
            Self {
                number: 0,
                string: QString::from(""),
                test: QVariantMap::default(),
            }
        }
    }
    // ANCHOR_END: book_rustobj_default

    // ANCHOR: book_rustobj_impl
    impl qobject::MyObject {
        #[qinvokable]
        pub fn increment_number(mut self: Pin<&mut Self>) {
            let mut value = QVariantMap::default();
            value.insert(QString::from("a"), QVariant::from(&1));
            self.as_mut().set_test(value);

            let previous = *self.as_ref().number();
            self.set_number(previous + 1);
        }

        #[qinvokable]
        pub fn say_hi(&self, string: &QString, number: i32) {
            println!("Hi from Rust! String is '{string}' and number is {number}");
        }
    }
    // ANCHOR_END: book_rustobj_impl
}
// ANCHOR_END: book_cxx_qt_module
