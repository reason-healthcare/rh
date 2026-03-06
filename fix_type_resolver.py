import sys

with open('crates/rh-cql/src/semantics/analyzer.rs', 'r') as f:
    text = f.read()

# Replace any lingering type_resolver calls
text = text.replace('.type_resolver\n            .resolve_type_specifier', '.resolve_type_specifier')
text = text.replace('.type_resolver\n                .resolve_type_specifier', '.resolve_type_specifier')

# Fix resolve_type_specifier and resolve_qualified_name signatures
text = text.replace('Result<DataType, String>', 'Result<DataType, crate::types::TypeError>')

with open('crates/rh-cql/src/semantics/analyzer.rs', 'w') as f:
    f.write(text)

