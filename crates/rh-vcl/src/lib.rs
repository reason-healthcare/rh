//! # rh-vcl - ValueSet Compose Language Parser
//!
//! This crate provides a parser for the ValueSet Compose Language (VCL),
//! a domain-specific language for expressing ValueSet CLDs in a compact syntax.
//!
//! VCL is inspired by SNOMED CT's Expression Constraint Language (ECL) and enables
//! a new family of implicit ValueSet URIs that are usable across all code systems.
//!
//! ## Example
//!
//! ```rust,ignore
//! use rh_vcl::{parse_vcl, VclExpression};
//!
//! let vcl_str = "(http://hl7.org/fhir/sid/icd-10){Z51.1*}";
//! let expr = parse_vcl(vcl_str)?;
//! ```

pub mod ast;
pub mod error;
pub mod explainer;
pub mod fhir;
pub mod parser;
pub mod translator;

#[cfg(target_arch = "wasm32")]
pub mod wasm;

pub use ast::{Code, Filter, Operation, SimpleExpression, VclExpression};
pub use error::VclError;
pub use explainer::{ComponentExplanation, ExplanationResult, ExpressionType, VclExplainer};
pub use fhir::{ValueSetCompose, ValueSetFilter, ValueSetInclude};
pub use parser::parse_vcl;
pub use translator::{translate_vcl_string_to_fhir, translate_vcl_to_fhir, VclTranslator};

/// Re-export commonly used types
pub type VclResult<T> = Result<T, VclError>;

/// Check if a VCL expression contains explicit system URIs
/// This is used to determine if a default system should be applied
pub fn has_explicit_systems(expr: &VclExpression) -> bool {
    has_explicit_systems_in_expr(&expr.expr)
}

fn has_explicit_systems_in_expr(expr: &ast::Expression) -> bool {
    // Check the main sub-expression
    if has_explicit_systems_in_subexpr(&expr.sub_expr) {
        return true;
    }

    // Check any operations (conjunctions, disjunctions, exclusions)
    if let Some(operation) = &expr.operation {
        match operation {
            ast::Operation::Conjunction(sub_exprs) | ast::Operation::Disjunction(sub_exprs) => {
                for sub_expr in sub_exprs {
                    if has_explicit_systems_in_subexpr(sub_expr) {
                        return true;
                    }
                }
            }
            ast::Operation::Exclusion(sub_expr) => {
                if has_explicit_systems_in_subexpr(sub_expr) {
                    return true;
                }
            }
        }
    }

    false
}

fn has_explicit_systems_in_subexpr(sub_expr: &ast::SubExpression) -> bool {
    // Check if this sub-expression has a system URI
    if sub_expr.system_uri.is_some() {
        return true;
    }

    // Recursively check nested expressions
    match &sub_expr.content {
        ast::SubExpressionContent::Nested(nested_expr) => has_explicit_systems_in_expr(nested_expr),
        ast::SubExpressionContent::Simple(_) => false,
    }
}

// Re-export WASM functions when targeting WASM
#[cfg(target_arch = "wasm32")]
pub use wasm::{
    explain_vcl_simple, explain_vcl_with_system, get_version, parse_vcl_expression,
    parse_vcl_simple, translate_vcl_expression, translate_vcl_simple, translate_vcl_with_system,
    validate_vcl_expression, ParseOptions, TranslateOptions, WasmResult,
};
