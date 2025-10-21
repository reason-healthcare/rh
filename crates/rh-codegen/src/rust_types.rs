//! Rust-specific types and structures for code generation
//!
//! This module contains data structures that represent generated Rust code elements
//! such as structs, fields, enums, and their associated metadata.

/// Represents a generated Rust struct
#[derive(Debug, Clone)]
pub struct RustStruct {
    pub name: String,
    pub doc_comment: Option<String>,
    pub fields: Vec<RustField>,
    pub derives: Vec<String>,
    pub is_public: bool,
    pub base_definition: Option<String>,
}

impl RustStruct {
    pub fn new(name: String) -> Self {
        Self {
            name,
            doc_comment: None,
            fields: Vec::new(),
            derives: vec![
                "Debug".to_string(),
                "Clone".to_string(),
                "Deserialize".to_string(),
                "Serialize".to_string(),
            ],
            is_public: true,
            base_definition: None,
        }
    }

    pub fn with_doc(mut self, doc: String) -> Self {
        self.doc_comment = Some(doc);
        self
    }

    pub fn add_field(&mut self, field: RustField) {
        self.fields.push(field);
    }
}

/// Represents a field in a Rust struct
#[derive(Debug, Clone)]
pub struct RustField {
    pub name: String,
    pub field_type: RustType,
    pub doc_comment: Option<String>,
    pub is_optional: bool,
    pub is_public: bool,
    pub serde_attributes: Vec<String>,
    /// If set, this field should be rendered as a macro call instead of a regular field
    pub macro_call: Option<String>,
}

impl RustField {
    pub fn new(name: String, field_type: RustType) -> Self {
        Self {
            name,
            field_type,
            doc_comment: None,
            is_optional: false,
            is_public: true,
            serde_attributes: Vec::new(),
            macro_call: None,
        }
    }

    /// Create a new field that represents a macro call
    pub fn new_macro_call(macro_call: String) -> Self {
        // Extract field name from macro call for display purposes
        let field_name = Self::extract_field_name_from_macro_call(&macro_call);

        Self {
            name: field_name,
            field_type: RustType::Custom("MacroCall".to_string()), // Placeholder type
            doc_comment: None,
            is_optional: false,
            is_public: true,
            serde_attributes: Vec::new(),
            macro_call: Some(macro_call),
        }
    }

    /// Extract field name from a macro call string
    fn extract_field_name_from_macro_call(macro_call: &str) -> String {
        // Parse macro call like: primitive_string!("field_name", true)
        if let Some(start) = macro_call.find('(') {
            if let Some(end) = macro_call.find(',') {
                let content = &macro_call[start + 1..end];
                let field_name = content.trim().trim_matches('"');
                return field_name.to_string();
            }
        }
        "unknown_field".to_string()
    }

    pub fn optional(mut self) -> Self {
        self.is_optional = true;
        self
    }

    pub fn with_doc(mut self, doc: String) -> Self {
        self.doc_comment = Some(doc);
        self
    }

    pub fn with_serde_rename(mut self, name: String) -> Self {
        self.serde_attributes.push(format!("rename = \"{name}\""));
        self
    }
}

/// Represents a Rust type
#[derive(Debug, Clone)]
pub enum RustType {
    String,
    Integer,
    Boolean,
    Float,
    Option(Box<RustType>),
    Vec(Box<RustType>),
    Box(Box<RustType>),
    Slice(Box<RustType>),
    Custom(String),
    Reference(String),
}

impl RustType {
    #[allow(clippy::inherent_to_string)]
    pub fn to_string(&self) -> String {
        match self {
            RustType::String => "String".to_string(),
            RustType::Integer => "i32".to_string(),
            RustType::Boolean => "bool".to_string(),
            RustType::Float => "f64".to_string(),
            RustType::Option(inner) => format!("Option<{}>", inner.to_string()),
            RustType::Vec(inner) => format!("Vec<{}>", inner.to_string()),
            RustType::Box(inner) => format!("Box<{}>", inner.to_string()),
            RustType::Slice(inner) => format!("[{}]", inner.to_string()),
            RustType::Custom(name) => name.clone(),
            RustType::Reference(name) => format!("&{name}"),
        }
    }

    pub fn wrap_in_option(self) -> Self {
        match self {
            RustType::Option(_) => self,
            _ => RustType::Option(Box::new(self)),
        }
    }
}

/// Represents a generated Rust enum
#[derive(Debug, Clone)]
pub struct RustEnum {
    pub name: String,
    pub doc_comment: Option<String>,
    pub variants: Vec<RustEnumVariant>,
    pub derives: Vec<String>,
    pub is_public: bool,
}

impl RustEnum {
    pub fn new(name: String) -> Self {
        Self {
            name,
            doc_comment: None,
            variants: Vec::new(),
            derives: vec!["Debug".to_string(), "Clone".to_string()],
            is_public: true,
        }
    }

    pub fn add_variant(&mut self, variant: RustEnumVariant) {
        self.variants.push(variant);
    }
}

/// Represents a variant in a Rust enum
#[derive(Debug, Clone)]
pub struct RustEnumVariant {
    pub name: String,
    pub doc_comment: Option<String>,
    pub data: Option<RustType>,
    pub serde_rename: Option<String>,
}

impl RustEnumVariant {
    pub fn new(name: String) -> Self {
        Self {
            name,
            doc_comment: None,
            data: None,
            serde_rename: None,
        }
    }

    pub fn with_data(mut self, data: RustType) -> Self {
        self.data = Some(data);
        self
    }

    pub fn with_serde_rename(mut self, rename: String) -> Self {
        self.serde_rename = Some(rename);
        self
    }
}

/// Represents a Rust type alias
#[derive(Debug, Clone)]
pub struct RustTypeAlias {
    pub name: String,
    pub target_type: RustType,
    pub doc_comment: Option<String>,
    pub is_public: bool,
}

impl RustTypeAlias {
    pub fn new(name: String, target_type: RustType) -> Self {
        Self {
            name,
            target_type,
            doc_comment: None,
            is_public: true,
        }
    }

    pub fn with_doc(mut self, doc: String) -> Self {
        self.doc_comment = Some(doc);
        self
    }
}

/// Represents a generated code module
#[derive(Debug, Clone)]
pub struct RustModule {
    pub name: String,
    pub structs: Vec<RustStruct>,
    pub enums: Vec<RustEnum>,
    pub imports: Vec<String>,
    pub doc_comment: Option<String>,
}

impl RustModule {
    pub fn new(name: String) -> Self {
        Self {
            name,
            structs: Vec::new(),
            enums: Vec::new(),
            imports: vec!["serde::{Deserialize, Serialize}".to_string()],
            doc_comment: None,
        }
    }

    pub fn add_struct(&mut self, rust_struct: RustStruct) {
        self.structs.push(rust_struct);
    }

    pub fn add_enum(&mut self, rust_enum: RustEnum) {
        self.enums.push(rust_enum);
    }

    pub fn add_import(&mut self, import: String) {
        if !self.imports.contains(&import) {
            self.imports.push(import);
        }
    }
}

/// Represents a parameter in a Rust method
#[derive(Debug, Clone)]
pub struct RustMethodParam {
    pub name: String,
    pub param_type: RustType,
    pub is_mut: bool,
    pub is_ref: bool,
}

impl RustMethodParam {
    pub fn new(name: String, param_type: RustType) -> Self {
        Self {
            name,
            param_type,
            is_mut: false,
            is_ref: false,
        }
    }

    pub fn with_mut(mut self) -> Self {
        self.is_mut = true;
        self
    }

    pub fn with_ref(mut self) -> Self {
        self.is_ref = true;
        self
    }
}

/// Represents a Rust trait definition
#[derive(Debug, Clone)]
pub struct RustTrait {
    pub name: String,
    pub doc_comment: Option<String>,
    pub methods: Vec<RustTraitMethod>,
    pub is_public: bool,
    pub super_traits: Vec<String>,
}

impl RustTrait {
    pub fn new(name: String) -> Self {
        Self {
            name,
            doc_comment: None,
            methods: Vec::new(),
            is_public: true,
            super_traits: Vec::new(),
        }
    }

    pub fn with_doc(mut self, doc: String) -> Self {
        self.doc_comment = Some(doc);
        self
    }

    pub fn add_method(&mut self, method: RustTraitMethod) {
        // Check if a method with this name already exists
        if !self
            .methods
            .iter()
            .any(|existing| existing.name == method.name)
        {
            self.methods.push(method);
        }
    }

    pub fn with_super_trait(mut self, super_trait: String) -> Self {
        self.super_traits.push(super_trait);
        self
    }
}

/// Represents a method declaration in a trait
#[derive(Debug, Clone)]
pub struct RustTraitMethod {
    pub name: String,
    pub params: Vec<RustMethodParam>,
    pub return_type: Option<RustType>,
    pub doc_comment: Option<String>,
    pub is_default: bool,
    pub default_body: Option<String>,
    /// The self parameter type: None (no self), Some("&self"), Some("&mut self"), Some("self")
    pub self_param: Option<String>,
}

impl RustTraitMethod {
    pub fn new(name: String) -> Self {
        Self {
            name,
            params: Vec::new(),
            return_type: None,
            doc_comment: None,
            is_default: false,
            default_body: None,
            self_param: Some("&self".to_string()), // Default to &self for backward compatibility
        }
    }

    pub fn with_param(mut self, param: RustMethodParam) -> Self {
        self.params.push(param);
        self
    }

    pub fn with_parameter(mut self, name: String, param_type: RustType) -> Self {
        self.params.push(RustMethodParam::new(name, param_type));
        self
    }

    pub fn with_return_type(mut self, return_type: RustType) -> Self {
        self.return_type = Some(return_type);
        self
    }

    pub fn with_doc(mut self, doc: String) -> Self {
        self.doc_comment = Some(doc);
        self
    }

    pub fn with_default_implementation(mut self, body: String) -> Self {
        self.is_default = true;
        self.default_body = Some(body);
        self
    }

    pub fn with_body(mut self, body: String) -> Self {
        self.default_body = Some(body);
        self
    }

    pub fn with_self_param(mut self, self_param: Option<String>) -> Self {
        self.self_param = self_param;
        self
    }
}

/// Represents a Rust trait implementation
#[derive(Debug, Clone)]
pub struct RustTraitImpl {
    /// The name of the trait being implemented
    pub trait_name: String,
    /// The name of the struct implementing the trait  
    pub struct_name: String,
    /// The methods implemented in this trait impl
    pub methods: Vec<RustTraitImplMethod>,
    /// Documentation comment for the impl
    pub doc_comment: Option<String>,
}

impl RustTraitImpl {
    pub fn new(trait_name: String, struct_name: String) -> Self {
        Self {
            trait_name,
            struct_name,
            methods: Vec::new(),
            doc_comment: None,
        }
    }

    pub fn with_doc(mut self, doc: String) -> Self {
        self.doc_comment = Some(doc);
        self
    }

    pub fn add_method(&mut self, method: RustTraitImplMethod) {
        self.methods.push(method);
    }

    pub fn with_method(mut self, method: RustTraitImplMethod) -> Self {
        self.methods.push(method);
        self
    }

    /// Returns true if this trait implementation has no methods
    pub fn is_empty(&self) -> bool {
        self.methods.is_empty()
    }
}

/// Represents a method implementation in a trait impl block
#[derive(Debug, Clone)]
pub struct RustTraitImplMethod {
    /// The name of the method
    pub name: String,
    /// The parameters of the method (excluding self)
    pub params: Vec<RustMethodParam>,
    /// The return type of the method
    pub return_type: String,
    /// The body of the method implementation
    pub body: String,
    /// Documentation comment for the method
    pub doc_comment: Option<String>,
    /// The self parameter type: None (no self), Some("&self"), Some("&mut self"), Some("self")
    pub self_param: Option<String>,
}

impl RustTraitImplMethod {
    pub fn new(name: String) -> Self {
        Self {
            name,
            params: Vec::new(),
            return_type: "()".to_string(),
            body: "todo!()".to_string(),
            doc_comment: None,
            self_param: Some("&self".to_string()), // Default to &self for backward compatibility
        }
    }

    pub fn with_return_type(mut self, return_type: String) -> Self {
        self.return_type = return_type;
        self
    }

    pub fn with_body(mut self, body: String) -> Self {
        self.body = body;
        self
    }

    pub fn with_doc(mut self, doc: String) -> Self {
        self.doc_comment = Some(doc);
        self
    }

    pub fn add_param(&mut self, param: RustMethodParam) {
        self.params.push(param);
    }

    pub fn with_param(mut self, param: RustMethodParam) -> Self {
        self.params.push(param);
        self
    }

    pub fn with_self_param(mut self, self_param: Option<String>) -> Self {
        self.self_param = self_param;
        self
    }
}
