#[cxx_qt::bridge(namespace = "cxx_qt::my_object")]
mod my_object {
    #[namespace = ""]
    unsafe extern "C++" {
        include!("cxx-qt-lib/include/qt_types.h");
        type QColor = cxx_qt_lib::QColor;
    }

    #[derive(Default)]
    pub struct Data {
        primitive: i32,
        opaque: UniquePtr<QColor>,
    }

    #[cxx_qt::qobject]
    #[derive(Default)]
    pub struct RustObj;
}
