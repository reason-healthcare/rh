import re
with open('crates/rh-cql/src/semantics/analyzer.rs', 'r') as f:
    text = f.read()
print(f"Len: {len(text)}")
