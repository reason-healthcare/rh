import re

content = open('crates/rh-cql/src/semantics/analyzer.rs').read()

# Fix unused err mappings
content = content.replace("            Err(err) => {", "            Err(_err) => {")

open('crates/rh-cql/src/semantics/analyzer.rs', 'w').write(content)
