// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::{
    parser::{
        constructor::Constructor, inherit::ParsedInheritedMethod, invokable::ParsedQInvokable,
        property::ParsedQProperty, signals::ParsedSignal,
    },
    syntax::{
        attribute::{attribute_find_path, attribute_tokens_to_map, AttributeDefault},
        path::path_compare_str,
    },
};
use syn::{
    spanned::Spanned, Attribute, Error, Ident, ImplItem, Item, ItemImpl, ItemStruct, LitStr,
    Result, Visibility,
};

/// Metadata for registering QML element
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct QmlElementMetadata {
    pub uri: String,
    pub name: String,
    pub version_major: usize,
    pub version_minor: usize,
    pub uncreatable: bool,
    pub singleton: bool,
}

/// A representation of a QObject within a CXX-Qt [syn::ItemMod]
///
/// This has initial splitting of [syn::Item]'s into relevant blocks, other phases will
/// then mutate these [syn::Item]'s for generation purposes.
pub struct ParsedQObject {
    /// The base class of the struct
    pub base_class: Option<String>,
    /// QObject struct that stores the invokables for the QObject
    pub qobject_struct: ItemStruct,
    /// The namespace of the QObject. If one isn't specified for the QObject,
    /// this will be the same as the module
    pub namespace: String,
    /// Representation of the Q_SIGNALS for the QObject
    pub signals: Vec<ParsedSignal>,
    /// List of invokables that need to be implemented on the C++ object in Rust
    ///
    /// These will also be exposed as Q_INVOKABLE on the C++ object
    pub invokables: Vec<ParsedQInvokable>,
    /// List of inherited methods
    pub inherited_methods: Vec<ParsedInheritedMethod>,
    /// List of "impl" items that need to be implemented on the C++ object in Rust
    ///
    /// Note that they will only be visible on the Rust side
    pub passthrough_impl_items: Vec<ImplItem>,
    /// Any user-defined constructors
    pub constructors: Vec<Constructor>,
    /// List of properties that need to be implemented on the C++ object
    ///
    /// These will be exposed as Q_PROPERTY on the C++ object
    pub properties: Vec<ParsedQProperty>,
    /// List of specifiers to register with in QML
    pub qml_metadata: Option<QmlElementMetadata>,
    /// Whether locking is enabled for this QObject
    pub locking: bool,
    /// Whether threading has been enabled for this QObject
    pub threading: bool,
    /// Items that we don't need to generate anything for CXX or C++
    /// eg impls on the Rust object or Default implementations
    pub others: Vec<Item>,
}

impl ParsedQObject {
    /// Parse a [syn::ItemStruct] into a [ParsedQObject] with the index of the cxx_qt::qobject specified
    pub fn from_struct(qobject_struct: &ItemStruct, attr_index: usize) -> Result<Self> {
        let qml_metadata = Self::parse_qml_metadata(qobject_struct, attr_index)?;

        let attrs_map = attribute_tokens_to_map::<Ident, LitStr>(
            &qobject_struct.attrs[attr_index],
            AttributeDefault::Some(|span| LitStr::new("", span)),
        )?;

        // Find if there is any base class
        let base_class = attrs_map
            .get(&quote::format_ident!("base"))
            .map(|base| base.value());

        // Load the namespace, if it is empty then the ParsedCxxQtData will inject any global namespace
        let namespace = attrs_map
            .get(&quote::format_ident!("namespace"))
            .map_or_else(|| "".to_owned(), |base| base.value());

        // Remove the macro from the struct
        let mut qobject_struct = qobject_struct.clone();
        qobject_struct.attrs.remove(attr_index);

        // Parse any properties in the struct
        // and remove the #[qproperty] attribute
        let properties = Self::parse_struct_attributes(&mut qobject_struct.attrs)?;

        // Ensure that the QObject is marked as pub otherwise the error is non obvious
        // https://github.com/KDAB/cxx-qt/issues/457
        if !matches!(qobject_struct.vis, Visibility::Public(..)) {
            return Err(Error::new(
                qobject_struct.span(),
                "qobject marked structs must be public",
            ));
        }

        Ok(Self {
            base_class,
            qobject_struct,
            namespace,
            signals: vec![],
            invokables: vec![],
            inherited_methods: vec![],
            passthrough_impl_items: vec![],
            constructors: vec![],
            properties,
            qml_metadata,
            locking: true,
            threading: false,
            others: vec![],
        })
    }

    fn parse_qml_metadata(
        qobject_struct: &ItemStruct,
        attr_index: usize,
    ) -> Result<Option<QmlElementMetadata>> {
        let attrs_map = attribute_tokens_to_map::<Ident, LitStr>(
            &qobject_struct.attrs[attr_index],
            AttributeDefault::Some(|span| LitStr::new("", span)),
        )?;
        let qml_uri = attrs_map.get(&quote::format_ident!("qml_uri"));
        let qml_version = attrs_map.get(&quote::format_ident!("qml_version"));
        let qml_name = attrs_map.get(&quote::format_ident!("qml_name"));
        let qml_uncreatable = attrs_map.get(&quote::format_ident!("qml_uncreatable"));
        let qml_singleton = attrs_map.get(&quote::format_ident!("qml_singleton"));
        match (qml_uri, qml_version) {
            (Some(qml_uri), Some(qml_version)) => {
                let qml_version = qml_version.value();
                let version_parts: Vec<_> = qml_version.split('.').collect();
                let version_major = version_parts[0]
                    .parse()
                    .expect("Could not parse major version from qml_version");
                let version_minor = version_parts.get(1).unwrap_or(&"0").parse().unwrap_or(0);

                let name = match qml_name {
                    Some(qml_name) => qml_name.value(),
                    None => qobject_struct.ident.to_string(),
                };

                Ok(Some(QmlElementMetadata {
                    uri: qml_uri.value(),
                    name,
                    version_major,
                    version_minor,
                    uncreatable: qml_uncreatable.is_some(),
                    singleton: qml_singleton.is_some(),
                }))
            }
            (Some(uri), None) => Err(Error::new(
                uri.span(),
                "qml_uri specified but no qml_version specified",
            )),
            (None, Some(version)) => Err(Error::new(
                version.span(),
                "qml_version specified but no qml_uri specified",
            )),
            (None, None) => {
                if let Some(qml_name) = qml_name {
                    return Err(Error::new(
                        qml_name.span(),
                        "qml_name specified but qml_uri and qml_version unspecified",
                    ));
                }
                if let Some(qml_uncreatable) = qml_uncreatable {
                    return Err(Error::new(
                        qml_uncreatable.span(),
                        "qml_uncreatable specified but qml_uri and qml_version unspecified",
                    ));
                }
                if let Some(qml_singleton) = qml_singleton {
                    return Err(Error::new(
                        qml_singleton.span(),
                        "qml_singleton specified but qml_uri and qml_version unspecified",
                    ));
                }
                Ok(None)
            }
        }
    }

    pub fn parse_trait_impl(&mut self, imp: ItemImpl) -> Result<()> {
        let (not, trait_path, _) = &imp
            .trait_
            .as_ref()
            .ok_or_else(|| Error::new_spanned(imp.clone(), "Expected trait impl!"))?;

        if let Some(attr) = imp.attrs.first() {
            return Err(Error::new_spanned(
                attr,
                "Attributes are not allowed on trait impls in cxx_qt::bridge",
            ));
        }

        if path_compare_str(trait_path, &["cxx_qt", "Locking"]) {
            if imp.unsafety.is_none() {
                return Err(Error::new_spanned(
                    trait_path,
                    "cxx_qt::Locking must be an unsafe impl",
                ));
            }

            if not.is_none() {
                return Err(Error::new_spanned(
                    trait_path,
                    "cxx_qt::Locking is enabled by default, it can only be negated.",
                ));
            }

            // Check that cxx_qt::Threading is not enabled
            if self.threading {
                return Err(Error::new_spanned(
                    trait_path,
                    "cxx_qt::Locking must be enabled if cxx_qt::Threading is enabled",
                ));
            }

            self.locking = false;
            Ok(())
        } else if path_compare_str(trait_path, &["cxx_qt", "Threading"]) {
            if not.is_some() {
                return Err(Error::new_spanned(
                    trait_path,
                    "Negative impls for cxx_qt::Threading are not allowed",
                ));
            }

            // Check that cxx_qt::Locking is not disabled
            if !self.locking {
                return Err(Error::new_spanned(
                    trait_path,
                    "cxx_qt::Locking must be enabled if cxx_qt::Threading is enabled",
                ));
            }

            self.threading = true;
            Ok(())
        } else if path_compare_str(trait_path, &["cxx_qt", "Constructor"]) {
            self.constructors.push(Constructor::parse(imp)?);
            Ok(())
        } else {
            // TODO: Give suggestions on which trait might have been meant
            Err(Error::new_spanned(
                trait_path,
                "Unsupported trait!\nCXX-Qt currently only supports:\n- cxx_qt::Threading\n- cxx_qt::Constructor\n- cxx_qt::Locking\nNote that the trait must always be fully-qualified."
            ))
        }
    }

    fn parse_struct_attributes(attrs: &mut Vec<Attribute>) -> Result<Vec<ParsedQProperty>> {
        let mut properties = vec![];

        // Note that once extract_if is stable, this would allow for comparing all the
        // elements once using path_compare_str and then building ParsedQProperty
        // from the extracted elements.
        // https://doc.rust-lang.org/nightly/std/vec/struct.Vec.html#method.extract_if
        while let Some(index) = attribute_find_path(attrs, &["qproperty"]) {
            properties.push(ParsedQProperty::parse(attrs.remove(index))?);
        }

        Ok(properties)
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    use crate::parser::tests::f64_type;
    use syn::{parse_quote, ItemImpl};

    pub fn create_parsed_qobject() -> ParsedQObject {
        let qobject_struct: ItemStruct = parse_quote! {
            #[cxx_qt::qobject]
            pub struct MyObject;
        };
        ParsedQObject::from_struct(&qobject_struct, 0).unwrap()
    }

    #[test]
    fn test_from_struct_no_base_class() {
        let qobject_struct: ItemStruct = parse_quote! {
            #[cxx_qt::qobject]
            pub struct MyObject;
        };

        let qobject = ParsedQObject::from_struct(&qobject_struct, 0).unwrap();
        assert!(qobject.base_class.is_none());
        assert!(qobject.qml_metadata.is_none());
    }

    #[test]
    fn test_from_struct_base_class() {
        let qobject_struct: ItemStruct = parse_quote! {
            #[cxx_qt::qobject(base = "QStringListModel")]
            pub struct MyObject;
        };

        let qobject = ParsedQObject::from_struct(&qobject_struct, 0).unwrap();
        assert_eq!(qobject.base_class.as_ref().unwrap(), "QStringListModel");
    }

    #[test]
    fn test_from_struct_properties_and_fields() {
        let qobject_struct: ItemStruct = parse_quote! {
            #[cxx_qt::qobject]
            #[qproperty(i32, int_property)]
            #[qproperty(i32, public_property)]
            pub struct MyObject {
                int_property: i32,
                pub public_property: i32,

                field: i32,
            }
        };

        let qobject = ParsedQObject::from_struct(&qobject_struct, 0).unwrap();
        assert_eq!(qobject.properties.len(), 2);
    }

    #[test]
    fn test_from_struct_fields() {
        let qobject_struct: ItemStruct = parse_quote! {
            #[cxx_qt::qobject]
            pub struct MyObject {
                field: i32,
            }
        };

        let qobject = ParsedQObject::from_struct(&qobject_struct, 0).unwrap();
        assert_eq!(qobject.properties.len(), 0);
    }

    #[test]
    fn test_parse_trait_impl_valid() {
        let mut qobject = create_parsed_qobject();
        let item: ItemImpl = parse_quote! {
            impl cxx_qt::Threading for qobject::MyObject {}
        };
        assert!(!qobject.threading);
        assert!(qobject.parse_trait_impl(item).is_ok());
        assert!(qobject.threading);
    }

    #[test]
    fn test_parse_trait_impl_invalid() {
        let mut qobject = create_parsed_qobject();

        // must be a trait
        let item: ItemImpl = parse_quote! {
            impl qobject::T {}
        };
        assert!(qobject.parse_trait_impl(item).is_err());

        // no attribute allowed
        let item: ItemImpl = parse_quote! {
            #[attr]
            impl cxx_qt::Threading for qobject::T {}
        };
        assert!(qobject.parse_trait_impl(item).is_err());

        // Threading cannot be negative
        let item: ItemImpl = parse_quote! {
            impl !cxx_qt::Threading for qobject::T {}
        };
        assert!(qobject.parse_trait_impl(item).is_err());

        // must be a known trait
        let item: ItemImpl = parse_quote! {
            #[attr]
            impl cxx_qt::ABC for qobject::T {}
        };
        assert!(qobject.parse_trait_impl(item).is_err());
    }

    #[test]
    fn test_parse_struct_fields_valid() {
        let item: ItemStruct = parse_quote! {
            #[cxx_qt::qobject]
            #[qproperty(f64, f64_property)]
            #[qproperty(f64, public_property)]
            pub struct T {
                f64_property: f64,
                pub public_property: f64,

                field: f64,
            }
        };
        let properties = ParsedQObject::from_struct(&item, 0).unwrap().properties;
        assert_eq!(properties.len(), 2);

        assert_eq!(properties[0].ident, "f64_property");
        assert_eq!(properties[0].ty, f64_type());

        assert_eq!(properties[1].ident, "public_property");
        assert_eq!(properties[1].ty, f64_type());
    }

    #[test]
    fn test_parse_struct_fields() {
        let item: ItemStruct = parse_quote! {
            #[cxx_qt::qobject]
            pub struct T(f64);
        };
        assert!(ParsedQObject::from_struct(&item, 0).is_ok());
    }

    #[test]
    fn test_qml_metadata() {
        let item: ItemStruct = parse_quote! {
            #[cxx_qt::qobject(qml_uri = "foo.bar", qml_version = "1.0")]
            pub struct MyObject;
        };
        let qobject = ParsedQObject::from_struct(&item, 0).unwrap();
        assert_eq!(
            qobject.qml_metadata,
            Some(QmlElementMetadata {
                uri: "foo.bar".to_owned(),
                name: "MyObject".to_owned(),
                version_major: 1,
                version_minor: 0,
                uncreatable: false,
                singleton: false,
            })
        );
    }

    #[test]
    fn test_qml_metadata_named() {
        let item: ItemStruct = parse_quote! {
            #[cxx_qt::qobject(qml_uri = "foo.bar", qml_version = "1", qml_name = "MyQmlElement")]
            pub struct MyNamedObject;
        };
        let qobject = ParsedQObject::from_struct(&item, 0).unwrap();
        assert_eq!(
            qobject.qml_metadata,
            Some(QmlElementMetadata {
                uri: "foo.bar".to_owned(),
                name: "MyQmlElement".to_owned(),
                version_major: 1,
                version_minor: 0,
                uncreatable: false,
                singleton: false,
            })
        );
    }

    #[test]
    fn test_qml_metadata_singleton() {
        let item: ItemStruct = parse_quote! {
            #[cxx_qt::qobject(qml_uri = "foo.bar", qml_version = "1", qml_singleton)]
            pub struct MyObject;
        };
        let qobject = ParsedQObject::from_struct(&item, 0).unwrap();
        assert_eq!(
            qobject.qml_metadata,
            Some(QmlElementMetadata {
                uri: "foo.bar".to_owned(),
                name: "MyObject".to_owned(),
                version_major: 1,
                version_minor: 0,
                uncreatable: false,
                singleton: true,
            })
        );
    }

    #[test]
    fn test_qml_metadata_uncreatable() {
        let item: ItemStruct = parse_quote! {
            #[cxx_qt::qobject(qml_uri = "foo.bar", qml_version = "1", qml_uncreatable)]
            pub struct MyObject;
        };
        let qobject = ParsedQObject::from_struct(&item, 0).unwrap();
        assert_eq!(
            qobject.qml_metadata,
            Some(QmlElementMetadata {
                uri: "foo.bar".to_owned(),
                name: "MyObject".to_owned(),
                version_major: 1,
                version_minor: 0,
                uncreatable: true,
                singleton: false,
            })
        );
    }

    #[test]
    fn test_qml_metadata_no_version() {
        let item: ItemStruct = parse_quote! {
            #[cxx_qt::qobject(qml_uri = "foo.bar")]
            pub struct MyObject;
        };
        assert!(ParsedQObject::from_struct(&item, 0).is_err());
    }

    #[test]
    fn test_qml_metadata_no_uri() {
        let item: ItemStruct = parse_quote! {
            #[cxx_qt::qobject(qml_version = "1.0")]
            pub struct MyObject;
        };
        assert!(ParsedQObject::from_struct(&item, 0).is_err());
    }

    #[test]
    fn test_qml_metadata_only_name_no_version_no_uri() {
        let item: ItemStruct = parse_quote! {
            #[cxx_qt::qobject(qml_name = "MyQmlElement")]
            pub struct MyObject;
        };
        assert!(ParsedQObject::from_struct(&item, 0).is_err());
    }
}
