#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        // Includes the type
        // Defines the IsRelocatable
        include!("test-qt-types/include/qstring.h");
        type QString = super::QString;

        // TODO: remove _test once this doesn't collide with cxx-qt-lib
        #[rust_name = "prepend_test"]
        fn prepend<'a>(&'a mut self, str: &QString) -> &'a mut QString;

        #[rust_name = "is_empty_test"]
        fn isEmpty(&self) -> bool;
    }

    unsafe extern "C++" {
        // Implementation details
        include!("test-qt-types/include/qstringimpl.h");

        #[rust_name = "qstring_new_default"]
        fn construct() -> QString;

        #[rust_name = "qstring_drop"]
        fn drop(value: &mut QString);

        #[rust_name = "qstring_eq"]
        fn operatorEq(a: &QString, b: &QString) -> bool;

        #[rust_name = "qstring_as_slice"]
        fn qstringAsSlice(string: &QString) -> &[u16];
    }
}

use core::fmt::Write;
use core::mem::MaybeUninit;
use cxx::{type_id, ExternType};

/// TODO
#[repr(C)]
pub struct QString {
    #[cfg(cxxqt_qt_version_major = "5")]
    _space: MaybeUninit<usize>,
    #[cfg(cxxqt_qt_version_major = "6")]
    _space: MaybeUninit<[usize; 3]>,
}

impl Default for QString {
    fn default() -> Self {
        ffi::qstring_new_default()
    }
}

impl Drop for QString {
    fn drop(&mut self) {
        ffi::qstring_drop(self);
    }
}

impl core::fmt::Display for QString {
    /// Format the `QString` as a Rust string.
    ///
    /// Note that this converts from UTF-16 to UTF-8.
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        if f.width().is_some() || f.precision().is_some() {
            return f.pad(&String::from(self));
        }
        for c in char::decode_utf16(self.as_slice().iter().copied()) {
            f.write_char(c.unwrap_or(char::REPLACEMENT_CHARACTER))?;
        }
        Ok(())
    }
}

impl core::fmt::Debug for QString {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        String::from(self).fmt(f)
    }
}

impl PartialEq for QString {
    fn eq(&self, other: &Self) -> bool {
        ffi::qstring_eq(self, other)
    }
}

impl Eq for QString {}

impl QString {
    pub fn as_slice(&self) -> &[u16] {
        ffi::qstring_as_slice(self)
    }
}

impl From<&QString> for String {
    /// Constructs a Rust `String` from a `QString` reference.
    ///
    /// Note that this converts from UTF-16 to UTF-8.
    fn from(qstring: &QString) -> Self {
        String::from_utf16_lossy(qstring.as_slice())
    }
}

unsafe impl ExternType for QString {
    type Id = type_id!("QString");
    type Kind = cxx::kind::Trivial;
}
