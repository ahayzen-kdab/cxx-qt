// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Be Wilson <be.wilson@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

// ANCHOR: book_cargo_executable_build_rs
use cxx_qt_build::{CxxQtBuilder, QmlModule};

fn include_dirs() -> Vec<String> {
    let mut out = vec![];
    for (k, v) in std::env::vars_os() {
        let k = k.to_string_lossy().into_owned();
        if !k.starts_with("DEP_") {
            continue;
        }

        if k.ends_with("_CXXBRIDGE_PREFIX") {
            continue;
        }

        if k.ends_with("_CXXBRIDGE_LINKS") {
            continue;
        }

        // likely a DIR look at the url for more info
        // https://github.com/dtolnay/cxx/blob/master/gen/build/src/deps.rs
        let path = v.into_string().unwrap();
        if std::path::Path::new(&path).is_dir() {
            println!("Found: {:?}", path);
            out.push(path);
        }
    }

    out
}

fn main() {
    CxxQtBuilder::new()
        // Link Qt's Network library
        // - Qt Core is always linked
        // - Qt Gui is linked by enabling the qt_gui Cargo feature of cxx-qt-lib.
        // - Qt Qml is linked by enabling the qt_qml Cargo feature of cxx-qt-lib.
        // - Qt Qml requires linking Qt Network on macOS
        .qt_module("Network")
        .qml_module(QmlModule {
            uri: "com.kdab.cxx_qt.demo",
            rust_files: &["src/cxxqt_object.rs"],
            qml_files: &["qml/main.qml"],
            ..Default::default()
        })
        // Add the includes from cxx_build
        .cc_builder(|cc| {
            cc.includes(include_dirs());
        })
        .build();
}
// ANCHOR_END: book_cargo_executable_build_rs
