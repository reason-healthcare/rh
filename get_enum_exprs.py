import re
with open("crates/rh-cql/src/elm/expression.rs", "r") as f:
    text = f.read()

structs = ["IsExpr", "AsExpr", "ConvertExpr", "IntervalExpr", "ListExpr", "TupleExpr", "Instance"]
for s in structs:
    pattern = r"pub struct " + s + r" \{(.*?)\}"
    match = re.search(pattern, text, re.DOTALL | re.MULTILINE)
    if match:
        print(f"\n--- {s} ---")
        print(match.group(1))

