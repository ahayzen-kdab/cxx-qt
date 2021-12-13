mod my_object {
    #[derive(Default)]
    struct Data {
        number: i32,
        string: String,
    }

    #[derive(Default)]
    struct RustObj;

    impl PropertyChangeHandler<FFICppObj, Property> for RustObj {
        fn handle_property_change(&mut self, _cpp: Cpp<&mut my_object>, _property: Property) {
            println!("change")
        }
    }
}
