// SPDX-FileCopyrightText: CXX Authors
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: David Tolnay <dtolnay@gmail.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::syntax::cfg::{parse_attribute, CfgExpr};
use cxx_gen::{CfgEvaluator, CfgResult};
use quote::quote;
use syn::{Attribute, Error, LitStr};

pub(crate) fn try_eval_attributes(
    cfg_evaluator: &dyn CfgEvaluator,
    attrs: &[Attribute],
) -> Result<bool, Error> {
    // Build a single CfgExpr from the Attributes
    let cfg_expr = attrs
        .iter()
        .map(parse_attribute)
        .collect::<Result<Vec<CfgExpr>, Error>>()?
        .into_iter()
        .reduce(|mut acc, e| {
            acc.merge(e);
            acc
        });

    // Evaluate the CfgExpr against the CfgEvaluator
    if let Some(cfg_expr) = cfg_expr {
        try_eval(cfg_evaluator, &cfg_expr).map_err(|errs| {
            errs.into_iter()
                .reduce(|mut acc, e| {
                    acc.combine(e);
                    acc
                })
                .expect("There should be at least one error")
        })
    } else {
        Ok(true)
    }
}

fn try_eval(cfg_evaluator: &dyn CfgEvaluator, expr: &CfgExpr) -> Result<bool, Vec<Error>> {
    match expr {
        CfgExpr::Unconditional => Ok(true),
        CfgExpr::Eq(ident, string) => {
            let key = ident.to_string();
            let value = string.as_ref().map(LitStr::value);
            match cfg_evaluator.eval(&key, value.as_deref()) {
                CfgResult::True => Ok(true),
                CfgResult::False => Ok(false),
                CfgResult::Undetermined { msg } => {
                    let span = quote!(#ident #string);
                    Err(vec![Error::new_spanned(span, msg)])
                }
            }
        }
        CfgExpr::All(list) => {
            let mut all_errors = Vec::new();
            for subexpr in list {
                match try_eval(cfg_evaluator, subexpr) {
                    Ok(true) => {}
                    Ok(false) => return Ok(false),
                    Err(errors) => all_errors.extend(errors),
                }
            }
            if all_errors.is_empty() {
                Ok(true)
            } else {
                Err(all_errors)
            }
        }
        CfgExpr::Any(list) => {
            let mut all_errors = Vec::new();
            for subexpr in list {
                match try_eval(cfg_evaluator, subexpr) {
                    Ok(true) => return Ok(true),
                    Ok(false) => {}
                    Err(errors) => all_errors.extend(errors),
                }
            }
            if all_errors.is_empty() {
                Ok(false)
            } else {
                Err(all_errors)
            }
        }
        CfgExpr::Not(subexpr) => match try_eval(cfg_evaluator, subexpr) {
            Ok(value) => Ok(!value),
            Err(errors) => Err(errors),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::collections::HashMap;
    use syn::{parse_quote, ItemMod};

    #[derive(Default)]
    struct CfgEvaluatorTest<'a> {
        cfgs: HashMap<&'a str, Option<&'a str>>,
    }

    impl<'a> CfgEvaluator for CfgEvaluatorTest<'a> {
        fn eval(&self, name: &str, query_value: Option<&str>) -> CfgResult {
            if self.cfgs.get(name) == Some(&query_value) {
                CfgResult::True
            } else {
                CfgResult::False
            }
        }
    }

    #[test]
    fn test_try_eval_attributes_eq() {
        let module: ItemMod = parse_quote! {
            #[cfg(a = "1")]
            #[cfg(b = "2")]
            mod module;
        };
        let mut cfg_evaluator = Box::new(CfgEvaluatorTest::default());
        assert_eq!(
            try_eval_attributes(cfg_evaluator.as_ref(), &module.attrs).unwrap(),
            false
        );

        // Insert cfg into map
        cfg_evaluator.cfgs.insert("a", Some("1"));
        cfg_evaluator.cfgs.insert("b", Some("2"));
        assert_eq!(
            try_eval_attributes(cfg_evaluator.as_ref(), &module.attrs).unwrap(),
            true
        );
    }

    #[test]
    fn test_try_eval_attributes_any() {
        let module: ItemMod = parse_quote! {
            #[cfg(any(a = "1", b = "2"))]
            mod module;
        };
        let mut cfg_evaluator = Box::new(CfgEvaluatorTest::default());
        assert_eq!(
            try_eval_attributes(cfg_evaluator.as_ref(), &module.attrs).unwrap(),
            false
        );

        // Insert cfg into map
        cfg_evaluator.cfgs.insert("a", Some("1"));
        assert_eq!(
            try_eval_attributes(cfg_evaluator.as_ref(), &module.attrs).unwrap(),
            true
        );
    }

    #[test]
    fn test_try_eval_attributes_all() {
        let module: ItemMod = parse_quote! {
            #[cfg(all(a = "1", b = "2"))]
            mod module;
        };
        let mut cfg_evaluator = Box::new(CfgEvaluatorTest::default());
        assert_eq!(
            try_eval_attributes(cfg_evaluator.as_ref(), &module.attrs).unwrap(),
            false
        );

        // Insert cfg into map
        cfg_evaluator.cfgs.insert("a", Some("1"));
        cfg_evaluator.cfgs.insert("b", Some("2"));
        assert_eq!(
            try_eval_attributes(cfg_evaluator.as_ref(), &module.attrs).unwrap(),
            true
        );
    }

    #[test]
    fn test_try_eval_attributes_not() {
        let module: ItemMod = parse_quote! {
            #[cfg(not(a = "1"))]
            mod module;
        };
        let mut cfg_evaluator = Box::new(CfgEvaluatorTest::default());
        assert_eq!(
            try_eval_attributes(cfg_evaluator.as_ref(), &module.attrs).unwrap(),
            true
        );

        // Insert cfg into map
        cfg_evaluator.cfgs.insert("a", Some("1"));
        assert_eq!(
            try_eval_attributes(cfg_evaluator.as_ref(), &module.attrs).unwrap(),
            false
        );
    }
}