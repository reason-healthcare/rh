import sys, re

file_path = "crates/rh-cql/src/emit/queries.rs"
with open(file_path, "r") as f:
    text = f.read()

# Fix emit_sort_clause signature
text = re.sub(
    r'fn emit_sort_clause\([^)]+\)\s*->\s*elm::SortClause',
    'fn emit_sort_clause(sort: &TypedSortClause) -> elm::SortClause',
    text
)

import os
with open(file_path, "w") as f:
    f.write(text)
