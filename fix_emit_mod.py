content = open('crates/rh-cql/src/emit/mod.rs').read()

# 1. Fix module declarations: remove functions/statements, add references + tests
old_mods = (
    "pub mod clinical;\n"
    "pub mod conditionals;\n"
    "pub mod functions;\n"
    "pub mod literals;\n"
    "pub mod operators;\n"
    "pub mod queries;\n"
    "pub mod statements;\n"
    "pub mod types;"
)
new_mods = (
    "pub mod clinical;\n"
    "pub mod conditionals;\n"
    "pub mod literals;\n"
    "pub mod operators;\n"
    "pub mod queries;\n"
    "pub mod references;\n"
    "pub mod types;\n"
    "\n"
    "#[cfg(test)]\n"
    "mod tests;"
)
assert old_mods in content, "module decls not found"
content = content.replace(old_mods, new_mods, 1)

# 2. Remove the test block
test_marker = '#[cfg(test)]\nmod tests {'
test_start = content.find(test_marker)
if test_start != -1:
    # find matching closing brace
    block_start = content.index('{', test_start + len('#[cfg(test)]\nmod tests')) + 1
    depth = 1
    i = block_start
    while i < len(content) and depth > 0:
        if content[i] == '{': depth += 1
        elif content[i] == '}': depth -= 1
        i += 1
    content = content[:test_start].rstrip() + '\n'
    print("Tests block removed")
else:
    print("WARNING: tests block not found!")

# 3. Replace IdentifierRef handler
old = (
    "            TE::IdentifierRef(id_ref) => {\n"
    "                let element = self.element_fields(node);\n"
    "                elm::Expression::ExpressionRef(elm::ExpressionRef {\n"
    "                    element,\n"
    "                    name: Some(id_ref.name.clone()),\n"
    "                    library_name: None,\n"
    "                })\n"
    "            }"
)
new = "            TE::IdentifierRef(id_ref) => references::emit_identifier_ref(id_ref, node, self),"
assert old in content, f"IdentifierRef not found"
content = content.replace(old, new, 1)

# 4. Replace QualifiedIdentifierRef handler
old = (
    "            TE::QualifiedIdentifierRef(qid) => {\n"
    "                let element = self.element_fields(node);\n"
    "                elm::Expression::ExpressionRef(elm::ExpressionRef {\n"
    "                    element,\n"
    "                    name: Some(qid.name.clone()),\n"
    "                    library_name: Some(qid.qualifier.clone()),\n"
    "                })\n"
    "            }"
)
new = "            TE::QualifiedIdentifierRef(qid) => references::emit_qualified_identifier_ref(qid, node, self),"
assert old in content, "QualifiedIdentifierRef not found"
content = content.replace(old, new, 1)

# 5. Replace FunctionInvocation handler
old = (
    "            TE::FunctionInvocation(fi) => {\n"
    "                let element = self.element_fields(node);\n"
    "                // First try built-in system functions; fall through to FunctionRef\n"
    "                if let Some(expr) = operators::emit_system_function(\n"
    "                    &fi.function,\n"
    "                    &fi.arguments,\n"
    "                    node,\n"
    "                    self,\n"
    "                    &|n, ctx| ctx.emit_expression(n),\n"
    "                ) {\n"
    "                    return expr;\n"
    "                }\n"
    "                let operand = fi\n"
    "                    .arguments\n"
    "                    .iter()\n"
    "                    .map(|a| self.emit_expression(a))\n"
    "                    .collect();\n"
    "                elm::Expression::FunctionRef(elm::FunctionRef {\n"
    "                    element,\n"
    "                    name: Some(fi.function.clone()),\n"
    "                    library_name: None,\n"
    "                    operand,\n"
    "                    signature: Vec::new(),\n"
    "                })\n"
    "            }"
)
new = "            TE::FunctionInvocation(fi) => references::emit_function_invocation(fi, node, self, |n, ctx| ctx.emit_expression(n)),"
if old in content:
    content = content.replace(old, new, 1)
    print("FunctionInvocation replaced")
else:
    print("WARNING: FunctionInvocation not found, searching...")
    idx = content.find("TE::FunctionInvocation")
    print(repr(content[idx:idx+300]))

# 6. Replace MemberInvocation handler
old = (
    "            TE::MemberInvocation(mi) => {\n"
    "                let element = self.element_fields(node);\n"
    "                let source = Box::new(self.emit_expression(&mi.expression));\n"
    "                elm::Expression::Property(elm::Property {\n"
    "                    element,\n"
    "                    path: Some(mi.member.clone()),\n"
    "                    source: Some(source),\n"
    "                    scope: None,\n"
    "                })\n"
    "            }"
)
new = "            TE::MemberInvocation(mi) => references::emit_member_invocation(mi, node, self, |n, ctx| ctx.emit_expression(n)),"
assert old in content, "MemberInvocation not found"
content = content.replace(old, new, 1)

# 7. Replace IndexInvocation handler
old = (
    "            TE::IndexInvocation(ii) => {\n"
    "                let element = self.element_fields(node);\n"
    "                let operand = vec![\n"
    "                    self.emit_expression(&ii.expression),\n"
    "                    self.emit_expression(&ii.index),\n"
    "                ];\n"
    "                elm::Expression::Indexer(elm::BinaryExpression {\n"
    "                    element,\n"
    "                    operand,\n"
    "                    signature: Vec::new(),\n"
    "                })\n"
    "            }"
)
new = "            TE::IndexInvocation(ii) => references::emit_index_invocation(ii, node, self, |n, ctx| ctx.emit_expression(n)),"
assert old in content, "IndexInvocation not found"
content = content.replace(old, new, 1)

# 8. Replace LetClause handler
old = (
    "            TE::LetClause(name, value) => {\n"
    "                // LetClauses are handled at the query level; if encountered standalone,\n"
    "                // emit as an alias reference.\n"
    "                let element = self.element_fields(node);\n"
    "                let _ = self.emit_expression(value); // ensure value is visited\n"
    "                elm::Expression::AliasRef(elm::AliasRef {\n"
    "                    element,\n"
    "                    name: Some(name.clone()),\n"
    "                })\n"
    "            }"
)
new = "            TE::LetClause(name, value) => references::emit_let_clause_standalone(name, value, node, self, |n, ctx| ctx.emit_expression(n)),"
assert old in content, "LetClause not found"
content = content.replace(old, new, 1)

# 9. Replace MinValue handler
old = (
    "            TE::MinValue(ts) => {\n"
    "                let element = self.element_fields(node);\n"
    "                let value_type = type_specifier_to_qname(ts);\n"
    "                elm::Expression::MinValue(elm::TypedExpression {\n"
    "                    element,\n"
    "                    value_type: Some(value_type),\n"
    "                })\n"
    "            }"
)
new = "            TE::MinValue(ts) => types::emit_min_value(ts, node, self),"
assert old in content, "MinValue not found"
content = content.replace(old, new, 1)

# 10. Replace MaxValue handler
old = (
    "            TE::MaxValue(ts) => {\n"
    "                let element = self.element_fields(node);\n"
    "                let value_type = type_specifier_to_qname(ts);\n"
    "                elm::Expression::MaxValue(elm::TypedExpression {\n"
    "                    element,\n"
    "                    value_type: Some(value_type),\n"
    "                })\n"
    "            }"
)
new = "            TE::MaxValue(ts) => types::emit_max_value(ts, node, self),"
assert old in content, "MaxValue not found"
content = content.replace(old, new, 1)

# 11. Replace TimingExpression handler
old = (
    "            TE::TimingExpression(te) => {\n"
    "                // Timing expressions are complex interval operations; delegate through binary\n"
    "                let element = self.element_fields(node);\n"
    "                // Emit as an Included-In as a placeholder — proper timing handling\n"
    "                // requires precision-aware dispatch which is covered in the timing module.\n"
    "                let left = self.emit_expression(&te.left);\n"
    "                let right = self.emit_expression(&te.right);\n"
    "                elm::Expression::IncludedIn(elm::TimeBinaryExpression {\n"
    "                    element,\n"
    "                    operand: vec![left, right],\n"
    "                    signature: Vec::new(),\n"
    "                    precision: None,\n"
    "                })\n"
    "            }"
)
new = "            TE::TimingExpression(te) => operators::emit_timing_expression(te, node, self, |n, ctx| ctx.emit_expression(n)),"
if old in content:
    content = content.replace(old, new, 1)
    print("TimingExpression replaced")
else:
    print("WARNING: TimingExpression not found")
    idx = content.find("TE::TimingExpression")
    if idx != -1:
        print(repr(content[idx:idx+400]))

# 12. Replace DateTimeComponentFrom handler
old = (
    "            TE::DateTimeComponentFrom(dtc) => {\n"
    "                let element = self.element_fields(node);\n"
    "                let operand = Some(Box::new(self.emit_expression(&dtc.operand)));\n"
    "                elm::Expression::DateTimeComponentFrom(elm::DateTimeComponentFrom {\n"
    "                    element,\n"
    "                    operand,\n"
    "                    precision: None,\n"
    "                })\n"
    "            }"
)
new = "            TE::DateTimeComponentFrom(dtc) => operators::emit_datetime_component_from(dtc, node, self, |n, ctx| ctx.emit_expression(n)),"
assert old in content, "DateTimeComponentFrom not found"
content = content.replace(old, new, 1)

open('crates/rh-cql/src/emit/mod.rs', 'w').write(content)
print(f"Done. Lines: {content.count(chr(10)) + 1}")
