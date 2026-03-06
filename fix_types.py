import sys

file_path = "crates/rh-cql/src/emit/types.rs"
with open(file_path, "r") as f:
    text = f.read()

# fix 1 and 2
old1 = """            let elements = tuple.elements.iter().map(|e| {
                elm::TupleElementDefinition {
                    name: e.name.clone(),
                    element_type: Some(emit_type_specifier(&e.element_type)),
                }
            }).collect();"""
new1 = """            let elements = tuple.elements.iter().map(|e| {
                elm::TupleElementDefinition {
                    name: e.name.clone(),
                    element_type: Some(Box::new(emit_type_specifier(&e.element_type))),
                    type_specifier: None, // Or we provide it here instead since error says missing
                }
            }).collect();"""
# wait, actually let's check TupleElementDefinition in ts.txt. Oh ts.txt didn't include TupleElementDefinition! It was cut off!

# fix 3
text = text.replace("choice.choices.iter()", "choice.types.iter()")

# fix 4
old_match = """        ast::TypeOperator::As => {
            elm::Expression::As(elm::AsExpr {
                element,
                operand: Some(operand_elm),
                as_type: qname,
                as_type_specifier: Some(ts),
                strict: Some(false),
            })
        }"""
new_match = """        ast::TypeOperator::As => {
            elm::Expression::As(elm::AsExpr {
                element,
                operand: Some(operand_elm),
                as_type: qname,
                as_type_specifier: Some(ts),
                strict: Some(false),
            })
        }
        ast::TypeOperator::Cast => {
            elm::Expression::As(elm::AsExpr {
                element,
                operand: Some(operand_elm),
                as_type: qname.clone(),
                as_type_specifier: Some(ts.clone()),
                strict: Some(true),
            })
        }"""
text = text.replace(old_match, new_match)

with open(file_path, "w") as f:
    f.write(text)
