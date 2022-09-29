// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

// ANCHOR: book_macro_code
#[cxx_qt::bridge]
pub mod ffi {
    #[cxx_qt::qobject]
    pub struct Invokables {
        #[qproperty]
        number: i32,

        rust_only_field: i32,
    }

    impl Default for Invokables {
        fn default() -> Self {
            Self {
                number: 0,
                rust_only_field: 1,
            }
        }
    }

    impl cxx_qt::QObject<Invokables> {
        // ANCHOR: book_cpp_obj
        #[qinvokable]
        pub fn invokable_mutate_cpp(self: Pin<&mut Self>) {
            let new_number = self.number() * 2;
            self.set_number(new_number);
        }
        // ANCHOR_END: book_cpp_obj

        #[qinvokable]
        pub fn invokable_return(&self) -> i32 {
            self.rust().rust_only_field
        }

        #[qinvokable]
        pub fn invokable_multiply(mut self: Pin<&mut Self>, factor: i32) -> i32 {
            unsafe {
                self.as_mut().rust_mut().rust_only_method(factor);
            }
            self.rust().rust_only_field
        }

        #[qinvokable]
        pub fn invokable_mutate_no_signal(mut self: Pin<&mut Self>, factor: i32) -> i32 {
            // Safety: This does not trigger a signal change for the property
            unsafe {
                *self.as_mut().number_mut() *= factor;
            }
            *self.number()
        }
    }

    impl Invokables {
        fn rust_only_method(&mut self, factor: i32) {
            self.rust_only_field *= factor;
        }
    }
}
// ANCHOR_END: book_macro_code