mod my_object {
    #[cxx::bridge(namespace = "cxx_qt::my_object")]
    mod ffi {
        enum Property {}

        unsafe extern "C++" {
            include!("cxx-qt-gen/include/my_object.h");

            type MyObject;
            #[namespace = ""]
            type QPointF = cxx_qt_lib::QPointF;
            #[namespace = ""]
            type QString = cxx_qt_lib::QString;
            #[namespace = ""]
            type QVariant = cxx_qt_lib::QVariant;

            #[namespace = "CxxQt"]
            type Variant = cxx_qt_lib::Variant;

            #[rust_name = "new_cpp_object"]
            fn newCppObject() -> UniquePtr<MyObject>;
        }

        extern "Rust" {
            type RustObj;

            #[cxx_name = "sayHiWrapper"]
            fn say_hi_wrapper(
                self: &RustObj,
                _cpp: Pin<&mut MyObject>,
                string: &QString,
                number: i32,
            );
            #[cxx_name = "sayByeWrapper"]
            fn say_bye_wrapper(self: &RustObj, _cpp: Pin<&mut MyObject>);

            #[cxx_name = "createRs"]
            fn create_rs() -> Box<RustObj>;

            #[cxx_name = "initialiseCpp"]
            fn initialise_cpp(cpp: Pin<&mut MyObject>);
        }
    }

    pub type FFICppObj = ffi::MyObject;
    pub type Property = ffi::Property;

    #[derive(Default)]
    struct RustObj;

    impl RustObj {
        fn say_hi_wrapper(
            &self,
            _cpp: std::pin::Pin<&mut FFICppObj>,
            string: &cxx_qt_lib::QString,
            number: i32,
        ) {
            let mut _cpp = CppObj::new(_cpp);
            return self.say_hi(&mut _cpp, string, number);
        }

        fn say_bye_wrapper(&self, _cpp: std::pin::Pin<&mut FFICppObj>) {
            let mut _cpp = CppObj::new(_cpp);
            return self.say_bye(&mut _cpp);
        }

        fn say_hi(&self, _cpp: &mut CppObj, string: &cxx_qt_lib::QString, number: i32) {
            println!(
                "Hi from Rust! String is {} and number is {}",
                string, number
            );
        }

        fn say_bye(&self, _cpp: &mut CppObj) {
            println!("Bye from Rust!");
        }
    }

    pub struct CppObj<'a> {
        cpp: std::pin::Pin<&'a mut FFICppObj>,
    }

    impl<'a> CppObj<'a> {
        pub fn new(cpp: std::pin::Pin<&'a mut FFICppObj>) -> Self {
            Self { cpp }
        }

        pub fn update_requester(&self) -> cxx_qt_lib::update_requester::UpdateRequester {
            use cxx_qt_lib::update_requester::{CxxQObject, UpdateRequester};

            let ptr: *const FFICppObj = unsafe { &*self.cpp.as_ref() };
            unsafe { UpdateRequester::new(ptr as *mut CxxQObject) }
        }

        pub fn grab_values_from_data(&mut self, data: &Data) {
            use cxx_qt_lib::MapQtValue;
        }
    }

    struct Data;

    impl<'a> From<&CppObj<'a>> for Data {
        fn from(_value: &CppObj<'a>) -> Self {
            Self {}
        }
    }

    fn create_rs() -> std::boxed::Box<RustObj> {
        std::default::Default::default()
    }

    fn initialise_cpp(cpp: std::pin::Pin<&mut FFICppObj>) {
        let mut wrapper = CppObj::new(cpp);
        wrapper.grab_values_from_data(&Data::default());
    }
}
