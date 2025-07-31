//! Token generation utilities for Rust code emission
//!
//! This module handles the generation of Rust code tokens from the internal
//! representation, including structs, enums, and modules.

use crate::rust_types::{RustEnum, RustEnumVariant, RustField, RustModule, RustStruct, RustType};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

/// Generates Rust code tokens from internal representations
#[derive(Debug)]
pub struct TokenGenerator {
    /// Whether to include serde derives by default
    include_serde: bool,
}

impl TokenGenerator {
    pub fn new() -> Self {
        Self {
            include_serde: true,
        }
    }

    pub fn with_serde(mut self, include_serde: bool) -> Self {
        self.include_serde = include_serde;
        self
    }

    /// Generate tokens for a complete module
    pub fn generate_module(&self, module: &RustModule) -> TokenStream {
        let mut tokens = TokenStream::new();

        // Add module documentation if present
        if let Some(doc) = &module.doc_comment {
            let doc_lines: Vec<_> = doc.lines().collect();
            for line in doc_lines {
                tokens.extend(quote! {
                    #[doc = #line]
                });
            }
        }

        // Add imports
        for import in &module.imports {
            let import_tokens: TokenStream = import.parse().expect("Invalid import statement");
            tokens.extend(quote! {
                use #import_tokens;
            });
        }

        // Add structs
        for rust_struct in &module.structs {
            tokens.extend(self.generate_struct(rust_struct));
        }

        // Add enums
        for rust_enum in &module.enums {
            tokens.extend(self.generate_enum(rust_enum));
        }

        tokens
    }

    /// Generate tokens for a Rust struct
    pub fn generate_struct(&self, rust_struct: &RustStruct) -> TokenStream {
        let name = format_ident!("{}", rust_struct.name);

        // Generate documentation
        let doc_attrs = if let Some(doc) = &rust_struct.doc_comment {
            let doc_lines: Vec<_> = doc.lines().collect();
            let attrs: Vec<TokenStream> = doc_lines
                .iter()
                .map(|line| quote! { #[doc = #line] })
                .collect();
            quote! { #(#attrs)* }
        } else {
            quote! {}
        };

        // Generate derive attributes
        let mut derives = rust_struct.derives.clone();
        if self.include_serde {
            if !derives.contains(&"Serialize".to_string()) {
                derives.push("Serialize".to_string());
            }
            if !derives.contains(&"Deserialize".to_string()) {
                derives.push("Deserialize".to_string());
            }
        }

        let derive_idents: Vec<_> = derives.iter().map(|d| format_ident!("{}", d)).collect();

        // Generate fields
        let mut fields: Vec<TokenStream> = Vec::new();

        // Add base definition as flattened field if present
        if let Some(base_def) = &rust_struct.base_definition {
            // Extract the base type name from the URL (e.g., "http://hl7.org/fhir/StructureDefinition/Element" -> "Element")
            let base_type = base_def.split('/').next_back().unwrap_or(base_def);
            let base_field_name = format_ident!("base");
            let base_type_ident = format_ident!("{}", base_type);

            fields.push(quote! {
                #[doc = "Base definition inherited from FHIR specification"]
                #[serde(flatten)]
                pub #base_field_name: #base_type_ident
            });
        }

        // Add regular fields
        let regular_fields: Vec<_> = rust_struct
            .fields
            .iter()
            .map(|field| self.generate_field(field))
            .collect();
        fields.extend(regular_fields);

        // Generate visibility
        let vis = if rust_struct.is_public {
            quote! { pub }
        } else {
            quote! {}
        };

        quote! {
            #doc_attrs
            #[derive(#(#derive_idents),*)]
            #vis struct #name {
                #(#fields),*
            }
        }
    }

    /// Generate tokens for a struct field
    fn generate_field(&self, field: &RustField) -> TokenStream {
        let name = format_ident!("{}", field.name);
        let field_type = self.generate_type(&field.field_type, field.is_optional);

        // Generate documentation
        let doc_attrs = if let Some(doc) = &field.doc_comment {
            let doc_lines: Vec<_> = doc.lines().collect();
            let attrs: Vec<TokenStream> = doc_lines
                .iter()
                .map(|line| quote! { #[doc = #line] })
                .collect();
            quote! { #(#attrs)* }
        } else {
            quote! {}
        };

        // Generate serde attributes
        let serde_attrs: Vec<TokenStream> = field
            .serde_attributes
            .iter()
            .map(|attr| {
                let attr_tokens: TokenStream = format!("serde({attr})")
                    .parse()
                    .expect("Invalid serde attribute");
                quote! { #[#attr_tokens] }
            })
            .collect();

        // Generate visibility
        let vis = if field.is_public {
            quote! { pub }
        } else {
            quote! {}
        };

        quote! {
            #doc_attrs
            #(#serde_attrs)*
            #vis #name: #field_type
        }
    }

    /// Generate tokens for a Rust enum
    pub fn generate_enum(&self, rust_enum: &RustEnum) -> TokenStream {
        let name = format_ident!("{}", rust_enum.name);

        // Generate documentation
        let doc_attrs = if let Some(doc) = &rust_enum.doc_comment {
            let doc_lines: Vec<_> = doc.lines().collect();
            let attrs: Vec<TokenStream> = doc_lines
                .iter()
                .map(|line| quote! { #[doc = #line] })
                .collect();
            quote! { #(#attrs)* }
        } else {
            quote! {}
        };

        // Generate derive attributes
        let mut derives = rust_enum.derives.clone();
        if self.include_serde {
            if !derives.contains(&"Serialize".to_string()) {
                derives.push("Serialize".to_string());
            }
            if !derives.contains(&"Deserialize".to_string()) {
                derives.push("Deserialize".to_string());
            }
        }

        let derive_idents: Vec<_> = derives.iter().map(|d| format_ident!("{}", d)).collect();

        // Generate variants
        let variants: Vec<_> = rust_enum
            .variants
            .iter()
            .map(|variant| self.generate_enum_variant(variant))
            .collect();

        // Generate visibility
        let vis = if rust_enum.is_public {
            quote! { pub }
        } else {
            quote! {}
        };

        quote! {
            #doc_attrs
            #[derive(#(#derive_idents),*)]
            #vis enum #name {
                #(#variants),*
            }
        }
    }

    /// Generate tokens for an enum variant
    fn generate_enum_variant(&self, variant: &RustEnumVariant) -> TokenStream {
        let name = format_ident!("{}", variant.name);

        // Generate documentation
        let doc_attrs = if let Some(doc) = &variant.doc_comment {
            let doc_lines: Vec<_> = doc.lines().collect();
            let attrs: Vec<TokenStream> = doc_lines
                .iter()
                .map(|line| quote! { #[doc = #line] })
                .collect();
            quote! { #(#attrs)* }
        } else {
            quote! {}
        };

        // Generate serde rename attribute if needed
        let serde_attrs = if let Some(rename) = &variant.serde_rename {
            quote! { #[serde(rename = #rename)] }
        } else {
            quote! {}
        };

        // Generate variant with optional data
        if let Some(data_type) = &variant.data {
            let data_tokens = self.generate_type(data_type, false);
            quote! {
                #doc_attrs
                #serde_attrs
                #name(#data_tokens)
            }
        } else {
            quote! {
                #doc_attrs
                #serde_attrs
                #name
            }
        }
    }

    /// Generate tokens for a Rust type
    #[allow(clippy::only_used_in_recursion)]
    fn generate_type(&self, rust_type: &RustType, wrap_optional: bool) -> TokenStream {
        let base_type = match rust_type {
            RustType::String => quote! { String },
            RustType::Integer => quote! { i32 },
            RustType::Boolean => quote! { bool },
            RustType::Float => quote! { f64 },
            RustType::Option(inner) => {
                let inner_tokens = self.generate_type(inner, false);
                quote! { Option<#inner_tokens> }
            }
            RustType::Vec(inner) => {
                let inner_tokens = self.generate_type(inner, false);
                quote! { Vec<#inner_tokens> }
            }
            RustType::Custom(name) => {
                let ident = format_ident!("{}", name);
                quote! { #ident }
            }
            RustType::Reference(name) => {
                let ident = format_ident!("{}", name);
                quote! { &#ident }
            }
        };

        if wrap_optional && !matches!(rust_type, RustType::Option(_)) {
            quote! { Option<#base_type> }
        } else {
            base_type
        }
    }
}

impl Default for TokenGenerator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rust_types::*;

    #[test]
    fn test_generate_simple_struct() {
        let generator = TokenGenerator::new();

        let mut rust_struct = RustStruct::new("TestStruct".to_string());
        rust_struct.add_field(RustField::new("name".to_string(), RustType::String));
        rust_struct.add_field(RustField::new("age".to_string(), RustType::Integer).optional());

        let tokens = generator.generate_struct(&rust_struct);
        let code = tokens.to_string();

        assert!(code.contains("struct TestStruct"));
        assert!(code.contains("pub name : String"));
        assert!(code.contains("pub age : Option < i32 >"));
    }

    #[test]
    fn test_generate_simple_enum() {
        let generator = TokenGenerator::new();

        let mut rust_enum = RustEnum::new("TestEnum".to_string());
        rust_enum.add_variant(RustEnumVariant::new("Variant1".to_string()));
        rust_enum
            .add_variant(RustEnumVariant::new("Variant2".to_string()).with_data(RustType::String));

        let tokens = generator.generate_enum(&rust_enum);
        let code = tokens.to_string();

        // Print the actual generated code for debugging
        println!("Generated enum code: {code}");

        assert!(code.contains("enum TestEnum"));
        assert!(code.contains("Variant1"));
        assert!(code.contains("Variant2"));
        assert!(code.contains("String"));
    }
}
