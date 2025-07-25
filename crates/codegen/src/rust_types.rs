//! Rust-specific data structures for code generation
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
}

impl RustStruct {
    pub fn new(name: String) -> Self {
        Self {
            name,
            doc_comment: None,
            fields: Vec::new(),
            derives: vec!["Debug".to_string(), "Clone".to_string()],
            is_public: true,
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
        }
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
        self.serde_attributes.push(format!("rename = \"{}\"", name));
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
    Custom(String),
    Reference(String),
}

impl RustType {
    pub fn to_string(&self) -> String {
        match self {
            RustType::String => "String".to_string(),
            RustType::Integer => "i32".to_string(),
            RustType::Boolean => "bool".to_string(),
            RustType::Float => "f64".to_string(),
            RustType::Option(inner) => format!("Option<{}>", inner.to_string()),
            RustType::Vec(inner) => format!("Vec<{}>", inner.to_string()),
            RustType::Custom(name) => name.clone(),
            RustType::Reference(name) => format!("&{}", name),
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
}

impl RustEnumVariant {
    pub fn new(name: String) -> Self {
        Self {
            name,
            doc_comment: None,
            data: None,
        }
    }

    pub fn with_data(mut self, data: RustType) -> Self {
        self.data = Some(data);
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
