// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

// ANCHOR: book_cargo_qml_bridge
#[cxx::bridge]
pub mod ffi {
    // Include the C++ code generated by rcc from the .qrc file
    // and create a binding to the qInitResources function
    unsafe extern "C++" {
        include!("qml.qrc.cpp");
        #[rust_name = "q_init_resources"]
        fn qInitResources() -> i32;
    }

    unsafe extern "C++" {
        include!("register_types.cpp");
        #[rust_name = "register_types"]
        fn registerTypes();
    }
}
// ANCHOR_END: book_cargo_qml_bridge
