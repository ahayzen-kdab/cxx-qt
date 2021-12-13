mod my_object {
    #[derive(Default)]
    struct RustObj;

    impl RustObj {
        #[invokable]
        fn sub_test(&self, _cpp: Cpp<&mut my_object>, sub: Cpp<&mut crate::sub_object>) {
            println!("Bye from Rust!");
        }
    }
}
