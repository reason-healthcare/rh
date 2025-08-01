//! Token generation utilities for Rust code emission
//!
//! This module handles the generation of Rust code tokens from the internal
//! representation, including structs, enums, and modules.

use crate::rust_types::{
    RustEnum, RustEnumVariant, RustField, RustModule, RustStruct, RustType, RustTypeAlias,
};
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

    /// Generate tokens for a Rust type alias
    pub fn generate_type_alias(&self, type_alias: &RustTypeAlias) -> TokenStream {
        let name = format_ident!("{}", type_alias.name);
        let target_type = self.generate_type(&type_alias.target_type, false);

        // Generate documentation
        let doc_attrs = if let Some(doc) = &type_alias.doc_comment {
            let doc_lines: Vec<_> = doc.lines().collect();
            let attrs: Vec<TokenStream> = doc_lines
                .iter()
                .map(|line| quote! { #[doc = #line] })
                .collect();
            quote! { #(#attrs)* }
        } else {
            quote! {}
        };

        // Generate visibility
        let vis = if type_alias.is_public {
            quote! { pub }
        } else {
            quote! {}
        };

        quote! {
            #doc_attrs
            #vis type #name = #target_type;
        }
    }

    /// Generate tokens for a Rust trait
    pub fn generate_trait(&self, rust_trait: &crate::rust_types::RustTrait) -> TokenStream {
        let trait_name = format_ident!("{}", rust_trait.name);

        // Generate documentation
        let doc = if let Some(doc_comment) = &rust_trait.doc_comment {
            let doc_lines: Vec<_> = doc_comment.lines().collect();
            quote! {
                #(#[doc = #doc_lines])*
            }
        } else {
            quote! {}
        };

        // Generate super traits
        let super_traits = if !rust_trait.super_traits.is_empty() {
            let super_trait_idents: Vec<_> = rust_trait
                .super_traits
                .iter()
                .map(|s| format_ident!("{}", s))
                .collect();
            quote! { : #(#super_trait_idents)+* }
        } else {
            quote! {}
        };

        // Generate trait methods
        let methods: Vec<TokenStream> = rust_trait
            .methods
            .iter()
            .map(|method| self.generate_trait_method(method))
            .collect();

        quote! {
            #doc
            pub trait #trait_name #super_traits {
                #(#methods)*
            }
        }
    }

    /// Generate tokens for a trait method
    fn generate_trait_method(&self, method: &crate::rust_types::RustTraitMethod) -> TokenStream {
        let method_name = format_ident!("{}", method.name);

        // Generate documentation
        let doc = if let Some(doc_comment) = &method.doc_comment {
            let doc_lines: Vec<_> = doc_comment.lines().collect();
            quote! {
                #(#[doc = #doc_lines])*
            }
        } else {
            quote! {}
        };

        // Generate parameters
        let params: Vec<TokenStream> = method
            .params
            .iter()
            .map(|param| {
                let param_name = format_ident!("{}", param.name);
                let param_type = self.generate_type(&param.param_type, false);

                match (param.is_ref, param.is_mut) {
                    (true, true) => quote! { #param_name: &mut #param_type },
                    (true, false) => quote! { #param_name: &#param_type },
                    (false, _) => quote! { #param_name: #param_type },
                }
            })
            .collect();

        // Add implicit &self parameter for trait methods
        let self_param = quote! { &self };
        let all_params = if params.is_empty() {
            vec![self_param]
        } else {
            let mut all = vec![self_param];
            all.extend(params);
            all
        };

        // Generate return type
        let return_type = if let Some(ret_type) = &method.return_type {
            let return_tokens = self.generate_type(ret_type, false);
            quote! { -> #return_tokens }
        } else {
            quote! {}
        };

        // Generate method body for default implementations
        if method.is_default {
            let body = if let Some(body_code) = &method.default_body {
                let body_tokens: TokenStream = body_code
                    .parse()
                    .unwrap_or_else(|_| quote! { unimplemented!() });
                quote! { { #body_tokens } }
            } else {
                quote! { { unimplemented!() } }
            };

            quote! {
                #doc
                fn #method_name(#(#all_params),*) #return_type #body
            }
        } else {
            quote! {
                #doc
                fn #method_name(#(#all_params),*) #return_type;
            }
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

    #[test]
    fn test_generate_type_alias() {
        let generator = TokenGenerator::new();

        let type_alias = RustTypeAlias::new("uri".to_string(), RustType::String)
            .with_doc("FHIR URI primitive type".to_string());

        let tokens = generator.generate_type_alias(&type_alias);
        let code = tokens.to_string();

        // Print the actual generated code for debugging
        println!("Generated type alias code: {code}");

        assert!(code.contains("type uri = String"));
        assert!(code.contains("FHIR URI primitive type"));
    }
}
