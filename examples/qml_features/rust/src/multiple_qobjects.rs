// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! This example shows how multiple QObjects can be defined in one module

/// A CXX-Qt bridge which shows multiple QObjects can be defined in one module
#[cxx_qt::bridge(cxx_file_stem = "multiple_qobjects")]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qcolor.h");
        /// QColor from cxx_qt_lib
        type QColor = cxx_qt_lib::QColor;
        include!("cxx-qt-lib/qurl.h");
        /// QUrl from cxx_qt_lib
        type QUrl = cxx_qt_lib::QUrl;
    }

    /// The first QObject
    #[cxx_qt::qobject(qml_uri = "com.kdab.cxx_qt.demo", qml_version = "1.0")]
    pub struct FirstObject {
        #[qproperty]
        counter: i32,
        #[qproperty]
        color: QColor,
    }

    impl Default for FirstObject {
        fn default() -> Self {
            Self {
                counter: 10,
                color: QColor::from_rgb(0, 0, 255),
            }
        }
    }

    /// Signals for the first QObject
    #[cxx_qt::qsignals(FirstObject)]
    pub enum FirstSignals {
        /// Accepted Q_SIGNAL
        Accepted,
        /// Rejected Q_SIGNAL
        Rejected,
    }

    impl qobject::FirstObject {
        /// A Q_INVOKABLE on the first QObject which increments a counter
        #[qinvokable]
        pub fn increment(mut self: Pin<&mut Self>) {
            let new_value = self.as_ref().counter() + 1;
            self.as_mut().set_counter(new_value);

            if new_value % 2 == 0 {
                self.as_mut().set_color(QColor::from_rgb(0, 0, 255));
                self.emit(FirstSignals::Accepted);
            } else {
                self.as_mut().set_color(QColor::from_rgb(255, 0, 0));
                self.emit(FirstSignals::Rejected);
            }
        }
    }

    /// The second QObject
    #[cxx_qt::qobject(qml_uri = "com.kdab.cxx_qt.demo", qml_version = "1.0")]
    pub struct SecondObject {
        #[qproperty]
        counter: i32,
        #[qproperty]
        url: QUrl,
    }

    impl Default for SecondObject {
        fn default() -> Self {
            Self {
                counter: 100,
                url: QUrl::from("https://github.com/kdab/cxx-qt"),
            }
        }
    }

    /// Signals for the second QObject
    #[cxx_qt::qsignals(SecondObject)]
    pub enum SecondSignals {
        /// Accepted Q_SIGNAL
        Accepted,
        /// Rejected Q_SIGNAL
        Rejected,
    }

    impl qobject::SecondObject {
        /// A Q_INVOKABLE on the second QObject which increments a counter
        #[qinvokable]
        pub fn increment(mut self: Pin<&mut Self>) {
            let new_value = self.as_ref().counter() + 1;
            self.as_mut().set_counter(new_value);

            if new_value % 5 == 0 {
                self.as_mut()
                    .set_url(QUrl::from("https://github.com/kdab/cxx-qt"));
                self.emit(SecondSignals::Accepted);
            } else {
                self.as_mut().set_url(QUrl::from("https://kdab.com"));
                self.emit(SecondSignals::Rejected);
            }
        }
    }
}
