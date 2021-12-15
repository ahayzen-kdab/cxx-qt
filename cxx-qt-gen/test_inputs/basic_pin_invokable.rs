mod my_object {
    #[derive(Default)]
    struct RustObj;

    impl RustObj {
        #[invokable]
        fn say_hi(&self, _cpp: Cpp<&mut my_object>, string: &QString, number: i32) {
            println!(
                "Hi from Rust! String is {} and number is {}",
                string, number
            );
        }

        #[invokable]
        fn say_bye(&self, _cpp: Cpp<&mut my_object>) {
            println!("Bye from Rust!");
        }
    }
}
