use crate::parser::span::SourceLocation;

#[derive(thiserror::Error, Debug)]
pub enum FshError {
    #[error("Parse error at {location:?}: {message}")]
    Parse {
        message: String,
        location: SourceLocation,
    },
    #[error("Duplicate entity '{name}' at {location:?}")]
    DuplicateEntity {
        name: String,
        location: SourceLocation,
    },
    #[error("Unresolved alias '{name}' at {location:?}")]
    UnresolvedAlias {
        name: String,
        location: SourceLocation,
    },
    #[error("RuleSet cycle: {names:?}")]
    RuleSetCycle { names: Vec<String> },
    #[error("Unknown parent '{name}' at {location:?}")]
    UnknownParent {
        name: String,
        location: SourceLocation,
    },
    #[error("Export error: {message}")]
    Export { message: String },
    #[error(transparent)]
    Foundation(#[from] rh_foundation::FoundationError),
}
