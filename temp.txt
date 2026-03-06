pub mod functions;
pub mod literals;
pub mod operators;
pub mod queries;
pub mod statements;

use crate::elm;
use crate::options::CompilerOptions;
use crate::semantics::typed_ast::TypedLibrary;

pub struct ElmEmitter {
    _local_id_counter: usize,
    _options: CompilerOptions,
    // optional source-map builder integration could go here
}

impl ElmEmitter {
    pub fn new(options: CompilerOptions) -> Self {
        Self {
            _local_id_counter: 1,
            _options: options,
        }
    }

    #[allow(dead_code)]

    pub fn element_fields<T>(&mut self, node: &crate::semantics::typed_ast::TypedNode<T>) -> crate::elm::ElementFields {
        let mut fields = crate::elm::ElementFields::default();
        if self._options.annotations_enabled() {
            fields.local_id = Some(self.generate_local_id());
        }
        // TODO: mapping to result_type_name or result_type_specifier based on self._options.result_types_enabled()
        fields
    }
    fn generate_local_id(&mut self) -> String {
        let id = self._local_id_counter;
        self._local_id_counter += 1;
        id.to_string()
    }

    pub fn emit(&mut self, typed_library: TypedLibrary) -> elm::Library {
        // Create an empty ELM library for now

        let mut usings = Vec::new();
        for u in typed_library.usings {
            usings.push(elm::UsingDef {
                local_identifier: Some(u.model_name.clone()),
                uri: None,
                version: u.version.clone(),
            });
        }
        let usings = if usings.is_empty() {
            None
        } else {
            Some(elm::UsingDefs { defs: usings })
        };

        let mut parameters = Vec::new();
        for p in typed_library.parameters {
            parameters.push(elm::ParameterDef {
                name: Some(p.name),
                access_level: Some(elm::AccessModifier::Public),
                parameter_type_name: None,
                parameter_type_specifier: None,
                default_value: None,
            });
        }
        let parameters = if parameters.is_empty() {
            None
        } else {
            Some(elm::ParameterDefs { defs: parameters })
        };

        let mut code_systems = Vec::new();
        for cs in typed_library.codesystems {
            code_systems.push(elm::CodeSystemDef {
                name: Some(cs.name),
                id: Some(cs.id),
                version: cs.version,
                access_level: Some(elm::AccessModifier::Public),
            });
        }
        let code_systems = if code_systems.is_empty() {
            None
        } else {
            Some(elm::CodeSystemDefs { defs: code_systems })
        };

        let mut value_sets = Vec::new();
        for vs in typed_library.valuesets {
            value_sets.push(elm::ValueSetDef {
                name: Some(vs.name),
                id: Some(vs.id),
                version: vs.version,
                access_level: Some(elm::AccessModifier::Public),
                code_system: Vec::new(),
            });
        }
        let value_sets = if value_sets.is_empty() {
            None
        } else {
            Some(elm::ValueSetDefs { defs: value_sets })
        };

        let mut contexts = Vec::new();
        for ctx in typed_library.contexts {
            contexts.push(elm::ContextDef {
                name: Some(ctx.name),
            });
        }
        let contexts = if contexts.is_empty() {
            None
        } else {
            Some(elm::ContextDefs { defs: contexts })
        };

        let mut codes = Vec::new();
        for c in typed_library.codes {
            codes.push(elm::CodeDef {
                name: Some(c.name),
                id: Some(c.code),
                display: c.display,
                code_system: Some(elm::CodeSystemDefRef {
                    name: Some(c.codesystem),
                    library_name: None,
                }),
                access_level: Some(elm::AccessModifier::Public),
            });
        }
        let codes = if codes.is_empty() {
            None
        } else {
            Some(elm::CodeDefs { defs: codes })
        };

        let mut concepts = Vec::new();
        for c in typed_library.concepts {
            concepts.push(elm::ConceptDef {
                name: Some(c.name),
                display: c.display,
                code: Vec::new(),
                access_level: Some(elm::AccessModifier::Public),
            });
        }
        let concepts = if concepts.is_empty() {
            None
        } else {
            Some(elm::ConceptDefs { defs: concepts })
        };

        let mut statements = Vec::new();
        for _s in typed_library.statements {
            // Need actual statement translation
            statements.push(elm::StatementDef::Expression(elm::ExpressionDef {
                name: Some("TodoStatement".to_string()),
                context: None,
                access_level: Some(elm::AccessModifier::Public),
                expression: Some(Box::new(elm::Expression::default())),
                local_id: None,
                locator: None,
                result_type_name: None,
                result_type_specifier: None,
                annotation: vec![],
            }));
        }
        let statements = if statements.is_empty() {
            None
        } else {
            Some(elm::ExpressionDefs { defs: statements })
        };

        elm::Library {
            local_id: None,
            annotation: vec![],
            identifier: Some(elm::VersionedIdentifier {
                id: Some(
                    typed_library
                        .identifier
                        .as_ref()
                        .map(|i| i.name.clone())
                        .unwrap_or_else(|| "Anonymous".to_string()),
                ),
                system: None,
                version: typed_library
                    .identifier
                    .as_ref()
                    .and_then(|i| i.version.clone()),
            }),
            schema_identifier: Some(elm::VersionedIdentifier {
                id: Some("urn:hl7-org:elm".to_string()),
                system: None,
                version: Some("r1".to_string()),
            }),
            usings,
            includes: None, // TODO mappings
            parameters,
            code_systems,
            value_sets,
            codes,
            concepts,
            contexts,
            statements,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_emit_empty_library() {
        let typed_lib = TypedLibrary {
            identifier: None,
            usings: vec![],
            includes: vec![],
            codesystems: vec![],
            valuesets: vec![],
            codes: vec![],
            concepts: vec![],
            parameters: vec![],
            contexts: vec![],
            statements: vec![],
        };

        let mut emitter = ElmEmitter::new(CompilerOptions::default());
        let elm_lib = emitter.emit(typed_lib);
        assert_eq!(elm_lib.identifier.unwrap().id.unwrap(), "Anonymous");
    }
}
