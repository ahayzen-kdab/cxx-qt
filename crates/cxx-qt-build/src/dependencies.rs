// SPDX-FileCopyrightText: 2024 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! This modules contains utilities for specifying dependencies with cxx-qt-build.

use serde::{Deserialize, Serialize};

use std::{collections::BTreeMap, path::PathBuf};

#[derive(Clone, Default, Serialize, Deserialize)]
/// This struct is used by cxx-qt-build internally to propagate data through to downstream
/// dependencies
pub(crate) struct Manifest {
    pub(crate) name: String,
    pub(crate) link_name: String,
    pub(crate) qt_modules: Vec<String>,
    pub(crate) initializers: Vec<qt_build_utils::Initializer>,
    pub(crate) exported_include_prefixes: Vec<String>,
}

pub(crate) struct DependencyCXX {
    // TODO: do we need to pass on prefix or link information too?
    pub(crate) header_dirs: Vec<PathBuf>,
    // TODO: do we need this for reexporting?
    #[allow(dead_code)]
    pub(crate) link_name: String,
}

impl DependencyCXX {
    pub(crate) fn find_all() -> Vec<DependencyCXX> {
        // See CXXBRIDGE_DIR env vars in the following file
        // https://github.com/dtolnay/cxx/blob/master/gen/build/src/deps.rs
        // and thise code has been directly copied from there
        let mut exported_header_dirs: BTreeMap<String, Vec<(usize, PathBuf)>> = BTreeMap::new();

        for (k, v) in std::env::vars_os() {
            let mut k = k.to_string_lossy().into_owned();
            if !k.starts_with("DEP_") {
                continue;
            }

            if k.ends_with("_CXXBRIDGE_PREFIX") {
                // k.truncate(k.len() - "_CXXBRIDGE_PREFIX".len());
                // crates.entry(k).or_default().include_prefix = Some(PathBuf::from(v));
                continue;
            }

            if k.ends_with("_CXXBRIDGE_LINKS") {
                // k.truncate(k.len() - "_CXXBRIDGE_LINKS".len());
                // crates.entry(k).or_default().links = Some(v);
                continue;
            }

            let without_counter = k.trim_end_matches(|ch: char| ch.is_ascii_digit());
            let counter_len = k.len() - without_counter.len();
            if counter_len == 0 || !without_counter.ends_with("_CXXBRIDGE_DIR") {
                continue;
            }

            let sort_key = k[k.len() - counter_len..]
                .parse::<usize>()
                .unwrap_or(usize::MAX);
            k.truncate(k.len() - counter_len - "_CXXBRIDGE_DIR".len());
            exported_header_dirs
                .entry(k)
                .or_default()
                .push((sort_key, PathBuf::from(v)));
        }

        exported_header_dirs
            .into_iter()
            .map(|(link_name, mut header_dirs)| {
                header_dirs.sort_by_key(|(sort_key, _dir)| *sort_key);
                let header_dirs = header_dirs
                    .into_iter()
                    .map(|(_sort_key, dir)| dir)
                    .collect();
                DependencyCXX {
                    header_dirs,
                    link_name: link_name.clone(),
                }
            })
            .collect()
    }
}

#[derive(Clone)]
/// A dependency that has been set up with [crate::CxxQtBuilder::library] and is available to
/// the crate that is currently being built.
pub(crate) struct Dependency {
    /// The path of the dependencies export directory
    pub(crate) path: PathBuf,
    /// The deserialized manifest of the dependency
    pub(crate) manifest: Manifest,
}

impl Dependency {
    /// This function will search the environment for all direct dependencies that have exported
    /// their Interface via [crate::Interface::export].
    /// They export their manifest paths as metadata, which will be exposed to us as an environment
    /// variable.
    /// We extract those paths here, parse the manifest and make sure to set it up correctly as a
    /// dependency.
    ///
    /// See also the internals "build system" section of our book.
    pub(crate) fn find_all() -> Vec<Dependency> {
        std::env::vars_os()
            .filter(|(var, _)| {
                let var = var.to_string_lossy();
                var.starts_with("DEP_") && var.ends_with("_CXX_QT_MANIFEST_PATH")
            })
            .map(|(_, manifest_path)| {
                let manifest_path = PathBuf::from(manifest_path);
                let manifest: Manifest = serde_json::from_str(
                    &std::fs::read_to_string(&manifest_path)
                        .expect("Could not read dependency manifest file!"),
                )
                .expect("Could not deserialize dependency manifest file!");

                println!(
                    "cxx-qt-build: Discovered dependency `{}` at: {}",
                    manifest.name,
                    manifest_path.display(),
                );
                Dependency {
                    path: manifest_path.parent().unwrap().to_owned(),
                    manifest,
                }
            })
            .collect()
    }
}

pub(crate) fn initializers(dependencies: &[Dependency]) -> Vec<qt_build_utils::Initializer> {
    dependencies
        .iter()
        .flat_map(|dep| dep.manifest.initializers.iter().cloned())
        .collect()
}
