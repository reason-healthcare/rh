import sys

with open('crates/rh-cql/src/semantics/analyzer.rs', 'r') as f:
    text = f.read()

old_member = """                                        if let Some(spec) = &el.element_type_specifier {
                                            if let Ok(res_dt) = self.resolve_type_specifier(spec) {
                                                dt = res_dt;
                                            }
                                        } else if let Some(type_str) = &el.element_type {"""

new_member = """                                        // TODO: support full modelinfo::TypeSpecifier
                                        if let Some(type_str) = &el.element_type {"""

text = text.replace(old_member, new_member)

with open('crates/rh-cql/src/semantics/analyzer.rs', 'w') as f:
    f.write(text)

