// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::{QByteArray, QString, QStringList, QVector};
use std::pin::Pin;

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qbytearray.h");
        type QByteArray = crate::QByteArray;
        include!("cxx-qt-lib/qstring.h");
        type QString = crate::QString;
        include!("cxx-qt-lib/qstringlist.h");
        type QStringList = crate::QStringList;
        include!("cxx-qt-lib/qvector.h");
        type QVector_QByteArray = crate::QVector<QByteArray>;

        include!("cxx-qt-lib/qguiapplication.h");
        type QGuiApplication;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        #[doc(hidden)]
        #[rust_name = "qguiapplication_new"]
        fn qguiapplicationNew(args: &QVector_QByteArray) -> UniquePtr<QGuiApplication>;

        // These are all static, so we need to create bindings until CXX supports statics
        #[doc(hidden)]
        #[rust_name = "qguiapplication_add_library_path"]
        fn qguiapplicationAddLibraryPath(app: Pin<&mut QGuiApplication>, path: &QString);
        #[doc(hidden)]
        #[rust_name = "qguiapplication_application_name"]
        fn qguiapplicationApplicationName(app: &QGuiApplication) -> QString;
        #[doc(hidden)]
        #[rust_name = "qguiapplication_application_version"]
        fn qguiapplicationApplicationVersion(app: &QGuiApplication) -> QString;
        #[doc(hidden)]
        #[rust_name = "qguiapplication_exec"]
        fn qguiapplicationExec(app: Pin<&mut QGuiApplication>) -> i32;
        #[doc(hidden)]
        #[rust_name = "qguiapplication_library_paths"]
        fn qguiapplicationLibraryPaths(app: &QGuiApplication) -> QStringList;
        #[doc(hidden)]
        #[rust_name = "qguiapplication_organization_domain"]
        fn qguiapplicationOrganizationDomain(app: &QGuiApplication) -> QString;
        #[doc(hidden)]
        #[rust_name = "qguiapplication_organization_name"]
        fn qguiapplicationOrganizationName(app: &QGuiApplication) -> QString;
        #[doc(hidden)]
        #[rust_name = "qguiapplication_set_application_name"]
        fn qguiapplicationSetApplicationName(app: Pin<&mut QGuiApplication>, name: &QString);
        #[doc(hidden)]
        #[rust_name = "qguiapplication_set_application_version"]
        fn qguiapplicationSetApplicationVersion(app: Pin<&mut QGuiApplication>, version: &QString);
        #[doc(hidden)]
        #[rust_name = "qguiapplication_set_library_paths"]
        fn qguiapplicationSetLibraryPaths(app: Pin<&mut QGuiApplication>, paths: &QStringList);
        #[doc(hidden)]
        #[rust_name = "qguiapplication_set_organization_domain"]
        fn qguiapplicationSetOrganizationDomain(app: Pin<&mut QGuiApplication>, domain: &QString);
        #[doc(hidden)]
        #[rust_name = "qguiapplication_set_organization_name"]
        fn qguiapplicationSetOrganizationName(app: Pin<&mut QGuiApplication>, name: &QString);
    }

    // QGuiApplication is not a trivial to CXX and is not relocatable in Qt
    // as the following fails in C++. So we cannot mark it as a trivial type
    // and need to use references or pointers.
    // static_assert(QTypeInfo<QGuiApplication>::isRelocatable);
    impl UniquePtr<QGuiApplication> {}
}

pub use ffi::QGuiApplication;

impl QGuiApplication {
    /// Prepends path to the beginning of the library path list,
    /// ensuring that it is searched for libraries first.
    /// If path is empty or already in the path list, the path list is not changed.
    pub fn add_library_path(self: Pin<&mut Self>, path: &QString) {
        ffi::qguiapplication_add_library_path(self, path);
    }

    /// The name of this application
    pub fn application_name(&self) -> QString {
        ffi::qguiapplication_application_name(self)
    }

    /// The version of this application
    pub fn application_version(&self) -> QString {
        ffi::qguiapplication_application_version(self)
    }

    /// Enters the main event loop and waits until exit() is called,
    /// and then returns the value that was set to exit() (which is 0 if exit() is called via quit()).
    pub fn exec(self: Pin<&mut Self>) -> i32 {
        ffi::qguiapplication_exec(self)
    }

    /// Returns a list of paths that the application will search when dynamically loading libraries.
    pub fn library_paths(&self) -> QStringList {
        ffi::qguiapplication_library_paths(self)
    }

    /// Initializes the window system and constructs an application object.
    /// Standard [Qt command line arguments](https://doc.qt.io/qt-6/qguiapplication.html#supported-command-line-options) are handled automatically.
    pub fn new() -> cxx::UniquePtr<Self> {
        let mut vector = QVector::<QByteArray>::default();

        // Construct an owned QVector of the args
        // as we need the args_os data to outlive this method
        // so we pass a QVector to C++ which is then stored
        for arg in std::env::args_os() {
            // Unix OsStrings can be directly converted to bytes.
            #[cfg(unix)]
            use std::os::unix::ffi::OsStrExt;

            // Windows OsStrings are WTF-8 encoded, so they need to be
            // converted to UTF-8 Strings before being converted to bytes.
            // https://simonsapin.github.io/wtf-8/
            #[cfg(windows)]
            let arg = arg.to_string_lossy();

            vector.append(QByteArray::from(arg.as_bytes()));
        }

        ffi::qguiapplication_new(&vector)
    }

    /// The Internet domain of the organization that wrote this application
    pub fn organization_domain(&self) -> QString {
        ffi::qguiapplication_organization_domain(self)
    }

    /// The name of the organization that wrote this application
    pub fn organization_name(&self) -> QString {
        ffi::qguiapplication_organization_name(self)
    }

    /// Set the name of this application
    pub fn set_application_name(self: Pin<&mut Self>, name: &QString) {
        ffi::qguiapplication_set_application_name(self, name);
    }

    /// Set the version of this application
    pub fn set_application_version(self: Pin<&mut Self>, version: &QString) {
        ffi::qguiapplication_set_application_version(self, version);
    }

    /// Sets the list of directories to search when loading plugins with QLibrary to paths.
    /// All existing paths will be deleted and the path list will consist of the paths given in paths and the path to the application.
    pub fn set_library_paths(self: Pin<&mut Self>, paths: &QStringList) {
        ffi::qguiapplication_set_library_paths(self, paths);
    }

    /// Sets the Internet domain of the organization that wrote this application
    pub fn qguiapplication_set_organization_domain(self: Pin<&mut Self>, domain: &QString) {
        ffi::qguiapplication_set_organization_domain(self, domain);
    }

    /// Sets the name of the organization that wrote this application
    pub fn set_organization_name(self: Pin<&mut Self>, name: &QString) {
        ffi::qguiapplication_set_organization_name(self, name);
    }
}
