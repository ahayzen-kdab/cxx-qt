fn main() {
    let qt_modules = vec!["Core".to_string()];
    // TODO: We might need different Qt installations
    // how do we sanely pick one for all the depends crates
    // Can we use features to default to one?
    let qt = qt_build_utils::QtBuild::new(qt_modules).expect("installation found");

    // TODO: need to inject cfg variables for the current Qt version
    // something from the QtInstallation helper?
    qt_build_utils::CfgGenerator::new(qt.version())
        // TODO: should we have a prefix "cxxqt_" as a parameter?
        .prefix("cxxqt_".to_string())
        .range_major(5..=7)
        .build();

    cxx_build::bridge("src/lib.rs")
        // Tell CXX about the Qt include paths
        .includes(qt.include_paths())
        .file("src/qstringimpl.cpp")
        // Configure the builder
        // TODO: do we need the other flags from cxx-qt-build?
        .std("c++17")
        .compile("test-qt-types");

    // TODO: do we need to link to Qt at this point?
    // likely yes, but this works as cxx-qt-build links at the end
    // qt.link_modules(&mut builder, &qt_modules);

    // TODO: we need to export this header somewhere?
    // actually we can import this instead from the env var it is set to
    // DEP_TEST_QT_TYPES_CXXBRIDGE_DIR{N}
    println!("cargo::rerun-if-changed=include/qstring.h");

    println!("cargo::rerun-if-changed=include/qstringimpl.h");
    println!("cargo::rerun-if-changed=src/qstringimpl.cpp");
}
