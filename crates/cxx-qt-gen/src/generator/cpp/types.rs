// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use std::collections::BTreeMap;
use syn::{
    spanned::Spanned, Error, GenericArgument, PathArguments, PathSegment, Result, Type,
    TypeReference,
};

/// A helper for describing a C++ type
///
/// If a cxx_type has been specified in the attribute then use it
/// otherwise parse the Rust type as the C++ type.
///
/// This is useful where you have UniquePtr<T> as the Rust type but
/// need T as the C++ type.
pub struct CppType {
    cxx_type: Option<String>,
    ty: String,
}

impl CppType {
    /// Retrieve either the cxx_type attribute value or the Rust type
    pub fn as_cxx_ty(&self) -> &str {
        if let Some(cxx_type) = &self.cxx_type {
            cxx_type.as_str()
        } else {
            self.as_rust_ty()
        }
    }

    /// Retrieve the Rust type in C++ form
    pub fn as_rust_ty(&self) -> &str {
        &self.ty
    }

    /// Construct a [CppType] from a given [syn::Type] and the contents of the cxx_type attribute
    pub fn from(
        ty: &Type,
        cxx_type: &Option<String>,
        cxx_names_map: &BTreeMap<String, String>,
    ) -> Result<CppType> {
        Ok(CppType {
            cxx_type: cxx_type.clone(),
            ty: to_cpp_string(ty, cxx_names_map)?,
        })
    }
}

/// For a given Rust type attempt to generate a C++ string
fn to_cpp_string(ty: &Type, cxx_names_map: &BTreeMap<String, String>) -> Result<String> {
    match ty {
        Type::Path(ty_path) => {
            let ty_strings = ty_path
                .path
                .segments
                .iter()
                .map(|generic| path_segment_to_string(generic, cxx_names_map))
                .collect::<Result<Vec<String>>>()?;
            if ty_strings.len() == 1 {
                let first = ty_strings.first().unwrap();
                Ok(cxx_names_map
                    .get(first)
                    .cloned()
                    .unwrap_or_else(|| possible_built_in(first)))
            } else {
                Ok(ty_strings.join("::"))
            }
        }
        Type::Reference(TypeReference {
            mutability, elem, ..
        }) => Ok(format!(
            "{is_const}{ty}&",
            is_const = if mutability.is_some() { "" } else { "const " },
            ty = to_cpp_string(elem, cxx_names_map)?
        )),
        _others => Err(Error::new(
            ty.span(),
            format!("Unsupported type, needs to be a TypePath: {:?}", _others),
        )),
    }
}

/// Convert any generic arguments to C++, eg A and B in Ty<A, B>
fn generic_argument_to_string(
    generic: &GenericArgument,
    cxx_names_map: &BTreeMap<String, String>,
) -> Result<String> {
    match generic {
        GenericArgument::Type(ty) => to_cpp_string(ty, cxx_names_map),
        _others => Err(Error::new(
            generic.span(),
            "Unsupported GenericArgument type",
        )),
    }
}

/// Convert the arguments for a path to C++, eg this is the whole <T> block
fn path_argument_to_string(
    args: &PathArguments,
    cxx_names_map: &BTreeMap<String, String>,
) -> Result<Option<Vec<String>>> {
    match args {
        PathArguments::AngleBracketed(angled) => Ok(Some(
            angled
                .args
                .iter()
                .map(|generic| generic_argument_to_string(generic, cxx_names_map))
                .collect::<Result<Vec<String>>>()?,
        )),
        PathArguments::Parenthesized(_) => Err(Error::new(
            args.span(),
            "Parenthesized arguments are unsupported",
        )),
        PathArguments::None => Ok(None),
    }
}

/// Convert a segment of a path to C++
fn path_segment_to_string(
    segment: &PathSegment,
    cxx_names_map: &BTreeMap<String, String>,
) -> Result<String> {
    let mut ident = segment.ident.to_string();

    // If we are a Pin<T> then for C++ it becomes just T
    let args = if ident == "Pin" {
        ident = path_argument_to_string(&segment.arguments, cxx_names_map)?
            .map_or_else(|| "".to_owned(), |values| values.join(", "));

        None
    } else {
        path_argument_to_string(&segment.arguments, cxx_names_map)?.map(|values| values.join(", "))
    };

    // If there are template args check that we aren't a recognised A of A<B>
    if args.is_some() {
        ident = possible_built_in_template_base(&ident);
    }

    Ok(format!(
        "{ident}{args}",
        ident = ident,
        args = args.map_or_else(|| "".to_owned(), |arg| format!("<{arg}>"))
    ))
}

/// Convert any built in types to known C++ equivalents
///
/// This is similar to the methods in CXX
/// https://github.com/dtolnay/cxx/blob/9c1737feff7208cd4825984614beaf09a27aefcf/syntax/atom.rs#L30
/// https://github.com/dtolnay/cxx/blob/a6e1cd1e8d9d6df20e88e7443963dc4c5c8c4875/gen/src/write.rs#L1311
fn possible_built_in(ty: &str) -> String {
    match ty {
        "bool" => "bool",
        "c_char" => "char",
        "u8" => "::std::uint8_t",
        "u16" => "::std::uint16_t",
        "u32" => "::std::uint32_t",
        "u64" => "::std::uint64_t",
        "usize" => "::std::size_t",
        "i8" => "::std::int8_t",
        "i16" => "::std::int16_t",
        "i32" => "::std::int32_t",
        "i64" => "::std::int64_t",
        "isize" => "::rust::isize",
        "f32" => "float",
        "f64" => "double",
        "CxxString" => "::std::string",
        "String" => "::rust::String",
        // TODO: handle reference
        // TODO: handle pointer
        // TODO: need to handle Type::Reference for &str ?
        "str" => "::rust::Str",
        others => others,
    }
    .to_owned()
}

/// Convert any templated bases to known C++ equivalents
///
/// This is similar to the method in CXX
/// https://github.com/dtolnay/cxx/blob/a6e1cd1e8d9d6df20e88e7443963dc4c5c8c4875/gen/src/write.rs#L1213
fn possible_built_in_template_base(ty: &str) -> String {
    match ty {
        "Box" => "::rust::Box",
        "Vec" => "::rust::Vec",
        "UniquePtr" => "::std::unique_ptr",
        "SharedPtr" => "::std::shared_ptr",
        "WeakPtr" => "::std::weak_ptr",
        "CxxVector" => "::std::vector",
        // TODO: handle Slice
        // TODO: handle Fn pointer
        // TODO: handle Array
        others => others,
        // TODO: what happens with Result<T> ?
    }
    .to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::tests::tokens_to_syn;
    use quote::quote;

    fn cxx_names_map_default() -> BTreeMap<String, String> {
        BTreeMap::default()
    }

    #[test]
    fn test_cxx_type_with_attribute() {
        let ty = tokens_to_syn(quote! { UniquePtr<QColor> });
        let cxx_ty =
            CppType::from(&ty, &Some("QColor".to_owned()), &cxx_names_map_default()).unwrap();
        assert_eq!(cxx_ty.as_cxx_ty(), "QColor");
        assert_eq!(cxx_ty.as_rust_ty(), "::std::unique_ptr<QColor>");
    }

    #[test]
    fn test_cxx_type_without_attribute() {
        let ty = tokens_to_syn(quote! { UniquePtr<QColor> });
        let cxx_ty = CppType::from(&ty, &None, &cxx_names_map_default()).unwrap();
        assert_eq!(cxx_ty.as_cxx_ty(), "::std::unique_ptr<QColor>");
        assert_eq!(cxx_ty.as_rust_ty(), "::std::unique_ptr<QColor>");
    }

    #[test]
    fn test_cxx_type_mapped() {
        let ty = tokens_to_syn(quote! { A });
        let mut cxx_names_map = BTreeMap::new();
        cxx_names_map.insert("A".to_owned(), "A1".to_owned());
        let cxx_ty = CppType::from(&ty, &None, &cxx_names_map).unwrap();
        assert_eq!(cxx_ty.as_cxx_ty(), "A1");
        assert_eq!(cxx_ty.as_rust_ty(), "A1");
    }

    #[test]
    fn test_cxx_type_mapped_with_attribute() {
        let ty = tokens_to_syn(quote! { A });
        let mut cxx_names_map = BTreeMap::new();
        cxx_names_map.insert("A".to_owned(), "A1".to_owned());
        let cxx_ty = CppType::from(&ty, &Some("B1".to_owned()), &cxx_names_map).unwrap();
        assert_eq!(cxx_ty.as_cxx_ty(), "B1");
        assert_eq!(cxx_ty.as_rust_ty(), "A1");
    }

    #[test]
    fn test_to_cpp_string_built_in_one_part() {
        let ty = tokens_to_syn(quote! { i32 });
        assert_eq!(
            to_cpp_string(&ty, &cxx_names_map_default()).unwrap(),
            "::std::int32_t"
        );
    }

    #[test]
    fn test_to_cpp_string_unknown_one_part() {
        let ty = tokens_to_syn(quote! { QPoint });
        assert_eq!(
            to_cpp_string(&ty, &cxx_names_map_default()).unwrap(),
            "QPoint"
        );
    }

    #[test]
    fn test_to_cpp_string_ref_const_one_part() {
        let ty = tokens_to_syn(quote! { &QPoint });
        assert_eq!(
            to_cpp_string(&ty, &cxx_names_map_default()).unwrap(),
            "const QPoint&"
        );
    }

    #[test]
    fn test_to_cpp_string_ref_mut_one_part() {
        let ty = tokens_to_syn(quote! { &mut QPoint });
        assert_eq!(
            to_cpp_string(&ty, &cxx_names_map_default()).unwrap(),
            "QPoint&"
        );
    }

    #[test]
    fn test_to_cpp_string_templated_built_in() {
        let ty = tokens_to_syn(quote! { Vec<f64> });
        assert_eq!(
            to_cpp_string(&ty, &cxx_names_map_default()).unwrap(),
            "::rust::Vec<double>"
        );
    }

    #[test]
    fn test_to_cpp_string_templated_unknown() {
        let ty = tokens_to_syn(quote! { UniquePtr<QColor> });
        assert_eq!(
            to_cpp_string(&ty, &cxx_names_map_default()).unwrap(),
            "::std::unique_ptr<QColor>"
        );
    }

    #[test]
    fn test_to_cpp_string_templated_built_in_ref_const() {
        let ty = tokens_to_syn(quote! { &Vec<f64> });
        assert_eq!(
            to_cpp_string(&ty, &cxx_names_map_default()).unwrap(),
            "const ::rust::Vec<double>&"
        );
    }

    #[test]
    fn test_to_cpp_string_templated_unknown_ref_mut() {
        let ty = tokens_to_syn(quote! { &mut UniquePtr<QColor> });
        assert_eq!(
            to_cpp_string(&ty, &cxx_names_map_default()).unwrap(),
            "::std::unique_ptr<QColor>&"
        );
    }

    #[test]
    fn test_to_cpp_string_mapped() {
        let ty = tokens_to_syn(quote! { A });
        let mut cxx_names_map = BTreeMap::new();
        cxx_names_map.insert("A".to_owned(), "A1".to_owned());
        assert_eq!(to_cpp_string(&ty, &cxx_names_map).unwrap(), "A1");
    }

    #[test]
    fn test_to_cpp_string_pin() {
        let ty = tokens_to_syn(quote! { Pin<T> });
        assert_eq!(to_cpp_string(&ty, &cxx_names_map_default()).unwrap(), "T");
    }

    #[test]
    fn test_to_cpp_string_pin_ref() {
        let ty = tokens_to_syn(quote! { Pin<&T> });
        assert_eq!(
            to_cpp_string(&ty, &cxx_names_map_default()).unwrap(),
            "const T&"
        );
    }

    #[test]
    fn test_to_cpp_string_pin_ref_mut() {
        let ty = tokens_to_syn(quote! { Pin<&mut T> });
        assert_eq!(to_cpp_string(&ty, &cxx_names_map_default()).unwrap(), "T&");
    }

    #[test]
    fn test_to_cpp_string_pin_template() {
        let ty = tokens_to_syn(quote! { Pin<UniquePtr<T>> });
        assert_eq!(
            to_cpp_string(&ty, &cxx_names_map_default()).unwrap(),
            "::std::unique_ptr<T>"
        );
    }

    #[test]
    fn test_to_cpp_string_pin_template_ref() {
        let ty = tokens_to_syn(quote! { Pin<&UniquePtr<T>> });
        assert_eq!(
            to_cpp_string(&ty, &cxx_names_map_default()).unwrap(),
            "const ::std::unique_ptr<T>&"
        );
    }

    #[test]
    fn test_to_cpp_string_pin_template_ref_mut() {
        let ty = tokens_to_syn(quote! { Pin<&mut UniquePtr<T>> });
        assert_eq!(
            to_cpp_string(&ty, &cxx_names_map_default()).unwrap(),
            "::std::unique_ptr<T>&"
        );
    }
}
