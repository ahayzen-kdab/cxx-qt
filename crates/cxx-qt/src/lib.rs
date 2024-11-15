// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#![deny(missing_docs)]

//! This crate and its associated crates provide a framework for generating QObjects from Rust.
//!
//! See the [book](https://kdab.github.io/cxx-qt/book/) for more information.

use std::{fs::File, io::Write, path::Path};

mod connection;
mod connectionguard;
#[doc(hidden)]
pub mod signalhandler;
mod threading;

pub use cxx_qt_macro::bridge;
pub use cxx_qt_macro::qobject;

pub use connection::{ConnectionType, QMetaObjectConnection};
pub use connectionguard::QMetaObjectConnectionGuard;
pub use threading::CxxQtThread;

// Export static assertions that can then be used in cxx-qt-gen generation
//
// These are currently used to ensure that CxxQtSignalHandler has the right size
#[doc(hidden)]
pub use static_assertions;

/// This trait is automatically implemented for all QObject types generated by CXX-Qt.
/// It provides information about the inner Rust struct that is wrapped by the QObject, as well as the methods
/// that Cxx-Qt will generate for the QObject.
pub trait CxxQtType {
    /// The Rust type that this QObject is wrapping.
    type Rust;

    /// Retrieve an immutable reference to the Rust struct backing this C++ object
    fn rust(&self) -> &Self::Rust;

    /// Retrieve a mutable reference to the Rust struct backing this C++ object
    fn rust_mut(self: core::pin::Pin<&mut Self>) -> core::pin::Pin<&mut Self::Rust>;
}

/// This trait indicates that the object implements threading and has a method which returns a [CxxQtThread].
///
/// The QObjects generated by CXX-Qt are neither [`Send`](https://doc.rust-lang.org/std/marker/trait.Send.html) nor [`Sync`](https://doc.rust-lang.org/std/marker/trait.Sync.html).
/// Therefore they may not be passed between threads nor accessed from multiple threads.
///
/// To achieve safe multi-threading on the Rust side we use an [CxxQtThread].
/// A [CxxQtThread] represents a reference to the Qt thread that the QObject lives in.
/// When a new Rust thread is started (e.g. in an invokable) the [CxxQtThread] can be moved into the thread to later update the QObject in a thread safe manner.
///
/// # Example
///
/// ```rust,ignore
/// # // FIXME: test doesn't link correctly on Windows
/// #[cxx_qt::bridge]
/// mod qobject {
///     unsafe extern "RustQt" {
///         #[qobject]
///         type MyStruct = super::MyStructRust;
///
///        #[qinvokable]
///         fn say_hello(self: Pin<&mut MyStruct>);
///     }
///
///     impl cxx_qt::Threading for MyStruct {}
/// }
///
/// use cxx_qt::Threading;
///
/// #[derive(Default)]
/// pub struct MyStructRust;
///
/// impl qobject::MyStruct {
///     pub fn say_hello(self: core::pin::Pin<&mut Self>) {
///         let qt_thread = self.qt_thread();
///
///         // Start a background thread that doesn't block the invokable
///         std::thread::spawn(move || {
///             std::thread::sleep(std::time::Duration::from_secs(1));
///
///             // Say hello on the Qt event loop
///             qt_thread.queue(|_| {
///                 println!("Hello");
///             }).unwrap();
///         });
///     }
/// }
///
/// # // Note that we need a fake main function for doc tests to build.
/// # fn main() {}
/// ```
pub trait Threading: Sized {
    #[doc(hidden)]
    type BoxedQueuedFn;
    #[doc(hidden)]
    type ThreadingTypeId;

    /// Create an instance of a [CxxQtThread]
    ///
    /// This allows for queueing closures onto the Qt event loop from a background thread.
    fn qt_thread(&self) -> CxxQtThread<Self>;

    #[doc(hidden)]
    fn is_destroyed(cxx_qt_thread: &CxxQtThread<Self>) -> bool;

    #[doc(hidden)]
    fn queue<F>(cxx_qt_thread: &CxxQtThread<Self>, f: F) -> Result<(), cxx::Exception>
    where
        F: FnOnce(core::pin::Pin<&mut Self>),
        F: Send + 'static;

    #[doc(hidden)]
    fn threading_clone(cxx_qt_thread: &CxxQtThread<Self>) -> CxxQtThread<Self>;

    #[doc(hidden)]
    fn threading_drop(cxx_qt_thread: &mut CxxQtThread<Self>);
}

/// Placeholder for upcasting objects, suppresses dead code warning
#[allow(dead_code)]
#[doc(hidden)]
pub trait Upcast<T> {}

/// This trait can be implemented on any [CxxQtType] to define a
/// custom constructor in C++ for the QObject.
///
/// The `Arguments` must be a tuple of CXX types that will be the arguments to the constructor in C++.
///
/// If this trait is implemented for a given [CxxQtType], it must also be declared inside the
/// [cxx_qt::bridge](bridge) macro.
/// See the example below.
///
/// Note that declaring an implementation of this trait will stop CXX-Qt from generating a default constructor.
/// Therefore an implementation of [Default] is no longer required for the Rust type.
///
/// # Minimal Example
///
/// ```rust
/// #[cxx_qt::bridge]
/// mod qobject {
///     extern "RustQt" {
///         #[qobject]
///         type MyStruct = super::MyStructRust;
///     }
///
///     // Declare that we want to use a custom constructor
///     // Note that the arguments must be a tuple of CXX types.
///     // Any associated types that aren't included here are assumed to be `()`.
///     impl cxx_qt::Constructor<(i32, String), NewArguments=(i32, String)> for MyStruct {}
/// }
///
/// // Struct without `Default` implementation
/// pub struct MyStructRust {
///     pub integer: i32,
///     pub string: String
/// }
///
/// impl cxx_qt::Constructor<(i32, String)> for qobject::MyStruct {
///     type BaseArguments = (); // Will be passed to the base class constructor
///     type InitializeArguments = (); // Will be passed to the "initialize" function
///     type NewArguments = (i32, String); // Will be passed to the "new" function
///
///     fn route_arguments(args: (i32, String)) -> (
///         Self::NewArguments,
///         Self::BaseArguments,
///         Self::InitializeArguments
///     ) {
///         (args, (), ())
///     }
///
///     fn new((integer, string): (i32, String)) -> MyStructRust {
///         MyStructRust {
///             integer,
///             string
///         }
///     }
/// }
///
/// # // Note that we need a fake main function for doc tests to build.
/// # fn main() {}
/// ```
///
/// # Pseudo Code for generated C++ Constructor
/// You can imagine this trait as creating a constructor roughly like this:
/// ```cpp
/// class MyCxxQtType : public QObject {
///     public:
///         MyCxxQtType(Arguments... args)
///             : QObject(Constructor::route_arguments(args).BaseArguments)
///             , m_rust(Constructor::new(Constructor::route_arguments(args).NewArguments))
///         {
///             Constructor::initialize(*this, Constructor::route_arguments(args).InitializeArguments);
///         }
/// }
/// ```
/// Note that in reality, `route_arguments` will only be called once and all arguments
/// will be moved, never copied.
///
/// # Initializing the QObject
///
/// In addition to running code before constructing the inner Rust struct, it may be useful to run code from the context of the QObject itself (i.e. inside the Constructor implementation).
///
/// The `initialize` function can be used to run code inside a constructor.
/// It is given a pinned mutable self reference to the QObject and the list of `InitializeArguments`.
///
/// ## Using the `Initialize` trait
///
/// The QML engine creates QML elements using their default constructor, so for most QML types only the `initialize` part of the constructor is of interest.
/// To reduce the boilerplate of this use-case, CXX-Qt provides the [Initialize] trait.
///
/// If a QObject implements the `Initialize` trait, and the inner Rust struct is [Default]-constructible it will automatically implement `cxx_qt::Constructor<()>`.
pub trait Constructor<Arguments>: CxxQtType {
    /// The arguments that are passed to the [`new()`](Self::new) function to construct the inner Rust struct.
    /// This must be a tuple of CXX compatible types.
    ///
    /// This way QObjects can be constructed that need additional arguments for constructing the
    /// inner Rust type.
    type NewArguments;
    /// The arguments that should be passed to the constructor of the base class.
    /// This must be a tuple of CXX compatible types.
    type BaseArguments;
    /// The arguments that should be used to initialize the QObject in the [`initialize()`](Self::initialize) function.
    /// This must be a tuple of CXX compatible types.
    type InitializeArguments;

    /// This function is called by CXX-Qt to route the arguments to the correct places.
    ///
    /// Using this function, you can split up the arguments required by the QObject constructor
    /// without additional copies.
    ///
    #[allow(unused_variables)]
    fn route_arguments(
        arguments: Arguments,
    ) -> (
        Self::NewArguments,
        Self::BaseArguments,
        Self::InitializeArguments,
    );

    /// This function is called to construct the inner Rust struct of the CXX-Qt QObject.
    /// You can use this to construct Rust structs that do not provide a [Default] implementation.
    fn new(arguments: Self::NewArguments) -> <Self as CxxQtType>::Rust;

    /// This function is called to initialize the QObject.
    /// After the members of the QObject is initialized, this function is called.
    /// This is equivalent to the body of the constructor in C++.
    ///
    /// # Default
    /// By default, this function does nothing
    #[allow(unused_variables)]
    fn initialize(self: core::pin::Pin<&mut Self>, arguments: Self::InitializeArguments) {
        // By default, do nothing
    }
}

/// This trait can be implemented on any [CxxQtType] to automatically define a default constructor
/// that calls the `initialize` function after constructing a default Rust struct.
///
/// Ensure that the `impl cxx_qt::Constructor<()> for ... {}` is declared inside the CXX-Qt bridge.
///
/// # Example
///
/// ```rust,ignore
/// # // FIXME: test doesn't link correctly on Windows
/// #[cxx_qt::bridge]
/// mod qobject {
///     extern "RustQt" {
///         #[qobject]
///         #[qproperty(i32, integer)]
///         type MyStruct = super::MyStructRust;
///     }
///
///     // Remember to tell the bridge about the default constructor
///     impl cxx_qt::Constructor<()> for MyStruct {}
/// }
///
/// // Make sure the inner Rust struct implements `Default`
/// #[derive(Default)]
/// pub struct MyStructRust {
///     integer: i32,
/// }
///
/// impl cxx_qt::Initialize for qobject::MyStruct {
///     fn initialize(self: core::pin::Pin<&mut Self>) {
///         self.on_integer_changed(|qobject| {
///             println!("New integer value: {}", qobject.integer);
///         }).release();
///     }
/// }
///
/// # // Note that we need a fake main function for doc tests to build.
/// # fn main() {}
/// ```
// TODO: Once the QObject type is available in the cxx-qt crate, also auto-generate a default
// constructor that takes QObject and passes it to the parent.
pub trait Initialize: CxxQtType {
    /// This function is called to initialize the QObject after construction.
    fn initialize(self: core::pin::Pin<&mut Self>);
}

impl<T> Constructor<()> for T
where
    T: Initialize,
    T::Rust: Default,
{
    type NewArguments = ();
    type BaseArguments = ();
    type InitializeArguments = ();

    fn new(_arguments: ()) -> <Self as CxxQtType>::Rust {
        <Self as CxxQtType>::Rust::default()
    }

    fn route_arguments(
        _arguments: (),
    ) -> (
        Self::NewArguments,
        Self::BaseArguments,
        Self::InitializeArguments,
    ) {
        ((), (), ())
    }

    fn initialize(self: core::pin::Pin<&mut Self>, _arguments: Self::InitializeArguments) {
        Self::initialize(self);
    }
}

#[doc(hidden)]
// Write the cxx-qt headers to the specified directory.
pub fn write_headers(directory: impl AsRef<Path>) {
    let directory = directory.as_ref();
    std::fs::create_dir_all(directory).expect("Could not create cxx-qt header directory");
    // Note ensure that the build script is consistent with files that are copied
    for (file_contents, file_name) in [
        (include_str!("../include/connection.h"), "connection.h"),
        (
            include_str!("../include/signalhandler.h"),
            "signalhandler.h",
        ),
        (include_str!("../include/thread.h"), "thread.h"),
        (include_str!("../include/threading.h"), "threading.h"),
        (include_str!("../include/trycatch.h"), "trycatch.h"),
        (include_str!("../include/type.h"), "type.h"),
    ] {
        // Note that we do not need rerun-if-changed for these files
        // as include_str causes a rerun when the header changes
        // and the files are always written to the target.
        let h_path = format!("{}/{file_name}", directory.display());
        let mut header = File::create(h_path).expect("Could not create cxx-qt header");
        write!(header, "{file_contents}").expect("Could not write cxx-qt header");
    }
}
