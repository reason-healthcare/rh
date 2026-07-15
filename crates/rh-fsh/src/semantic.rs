//! Semantic operations lowered from resolved FSH assignment rules.

use crate::parser::ast::{
    AssignmentRule, FshPath, FshPathSegment, FshValue, InstanceRule, SdRule, Spanned,
};
use crate::SourceLocation;

/// Explicit repetition behavior carried by a semantic path.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RepetitionSelection {
    Index(u32),
    Named,
    Append,
    Current,
}

/// Location and behavior of one repetition selector in a semantic path.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PathSelection {
    pub segment_index: usize,
    pub selection: RepetitionSelection,
}

/// A fully qualified path with selection semantics preclassified during lowering.
#[derive(Debug, Clone)]
pub struct SemanticPath {
    segments: Vec<FshPathSegment>,
    selections: Vec<PathSelection>,
}

impl SemanticPath {
    pub fn segments(&self) -> &[FshPathSegment] {
        &self.segments
    }

    pub fn selections(&self) -> &[PathSelection] {
        &self.selections
    }

    pub fn has_trailing_append(&self) -> bool {
        matches!(
            self.segments.last(),
            Some(FshPathSegment::Slice { slice, .. }) if slice == "+"
        )
    }

    fn qualify(parent: Option<&Self>, path: &FshPath) -> Self {
        let mut segments = parent.map_or_else(Vec::new, |parent| parent.segments.clone());
        segments.extend(path.segments.clone());
        Self::from_segments(segments)
    }

    fn trailing_append_as_current(&self) -> Self {
        let mut segments = self.segments.clone();
        if let Some(FshPathSegment::Slice { slice, .. }) = segments.last_mut() {
            if slice == "+" {
                *slice = "=".to_string();
            }
        }
        Self::from_segments(segments)
    }

    fn from_segments(segments: Vec<FshPathSegment>) -> Self {
        let selections = segments
            .iter()
            .enumerate()
            .filter_map(|(segment_index, segment)| match segment {
                FshPathSegment::Index(index) => Some(PathSelection {
                    segment_index,
                    selection: RepetitionSelection::Index(*index),
                }),
                FshPathSegment::Slice { slice, .. } => Some(PathSelection {
                    segment_index,
                    selection: match slice.as_str() {
                        "+" => RepetitionSelection::Append,
                        "=" => RepetitionSelection::Current,
                        _ => RepetitionSelection::Named,
                    },
                }),
                _ => None,
            })
            .collect();
        Self {
            segments,
            selections,
        }
    }
}

/// One resolved assignment ready for typed-tree application.
#[derive(Debug, Clone)]
pub struct SemanticAssignment {
    pub path: SemanticPath,
    pub value: serde_json::Value,
    pub exactly: bool,
    pub location: SourceLocation,
}

/// Ordered semantic operation emitted from resolved FSH rules.
#[derive(Debug, Clone)]
pub enum SemanticOperation {
    EstablishContext {
        path: SemanticPath,
        location: SourceLocation,
    },
    Assign(SemanticAssignment),
}

/// Lowered assignment program for one instance or local profile.
#[derive(Debug, Clone, Default)]
pub struct SemanticProgram {
    operations: Vec<SemanticOperation>,
}

impl SemanticProgram {
    pub fn lower_instance<F>(rules: &[Spanned<InstanceRule>], resolve_value: F) -> Self
    where
        F: FnMut(&FshValue) -> serde_json::Value,
    {
        lower_rules(
            rules.iter().map(|rule| {
                let lowered = match &rule.value {
                    InstanceRule::Assignment(assignment) => LowerableRule::Assign(assignment),
                    InstanceRule::Path(path) => LowerableRule::Context(&path.path),
                    InstanceRule::Insert(_) => LowerableRule::Ignored,
                };
                (rule.location, lowered)
            }),
            resolve_value,
            true,
        )
    }

    pub fn lower_profile<F>(rules: &[Spanned<SdRule>], resolve_value: F) -> Self
    where
        F: FnMut(&FshValue) -> serde_json::Value,
    {
        lower_rules(
            rules.iter().map(|rule| {
                let lowered = match &rule.value {
                    SdRule::Assignment(assignment) => LowerableRule::Assign(assignment),
                    SdRule::Path(path) => LowerableRule::Context(&path.path),
                    _ => LowerableRule::Ignored,
                };
                (rule.location, lowered)
            }),
            resolve_value,
            false,
        )
    }

    pub fn operations(&self) -> &[SemanticOperation] {
        &self.operations
    }

    pub fn into_operations(self) -> impl Iterator<Item = SemanticOperation> {
        self.operations.into_iter()
    }
}

enum LowerableRule<'a> {
    Assign(&'a AssignmentRule),
    Context(&'a FshPath),
    Ignored,
}

fn lower_rules<'a, I, F>(
    rules: I,
    mut resolve_value: F,
    stabilize_append_context: bool,
) -> SemanticProgram
where
    I: IntoIterator<Item = (SourceLocation, LowerableRule<'a>)>,
    F: FnMut(&FshValue) -> serde_json::Value,
{
    let mut operations = Vec::new();
    let mut contexts: Vec<(usize, SemanticPath)> = Vec::new();

    for (location, rule) in rules {
        let indent = location.column;
        while contexts
            .last()
            .is_some_and(|(context_indent, _)| *context_indent >= indent)
        {
            contexts.pop();
        }
        let parent = contexts
            .iter()
            .rev()
            .find(|(context_indent, _)| *context_indent < indent)
            .map(|(_, path)| path);
        match rule {
            LowerableRule::Assign(assignment) => {
                operations.push(SemanticOperation::Assign(SemanticAssignment {
                    path: SemanticPath::qualify(parent, &assignment.path),
                    value: resolve_value(&assignment.value),
                    exactly: assignment.exactly,
                    location,
                }));
            }
            LowerableRule::Context(path) => {
                let path = SemanticPath::qualify(parent, path);
                operations.push(SemanticOperation::EstablishContext {
                    path: path.clone(),
                    location,
                });
                let active_path = if stabilize_append_context {
                    path.trailing_append_as_current()
                } else {
                    path
                };
                contexts.push((indent, active_path));
            }
            LowerableRule::Ignored => {}
        }
    }

    SemanticProgram { operations }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::FshEntity;
    use crate::FshParser;

    fn parsed_instance_rules(source: &str) -> Vec<Spanned<InstanceRule>> {
        let document = FshParser::parse(source, "semantic.fsh").expect("FSH parses");
        document
            .entities
            .into_iter()
            .find_map(|entity| match entity.value {
                FshEntity::Instance(instance) => Some(instance.rules),
                _ => None,
            })
            .expect("instance exists")
    }

    #[test]
    fn lowers_indented_assignments_to_fully_qualified_paths() {
        let rules = parsed_instance_rules(
            "Instance: example\nInstanceOf: Observation\n* component[+]\n  * code.text = \"A\"\n  * valueString = \"B\" (exactly)\n",
        );

        let program = SemanticProgram::lower_instance(&rules, |value| match value {
            FshValue::Str(value) => serde_json::Value::String(value.clone()),
            _ => unreachable!(),
        });
        let assignments = program
            .operations()
            .iter()
            .filter_map(|operation| match operation {
                SemanticOperation::Assign(assignment) => Some(assignment),
                _ => None,
            })
            .collect::<Vec<_>>();

        assert_eq!(assignments.len(), 2);
        assert_eq!(
            assignments[0].path.segments(),
            &[
                FshPathSegment::Slice {
                    element: "component".to_string(),
                    slice: "=".to_string(),
                },
                FshPathSegment::Name("code".to_string()),
                FshPathSegment::Name("text".to_string()),
            ]
        );
        assert!(matches!(
            assignments[0].path.selections()[0].selection,
            RepetitionSelection::Current
        ));
        assert_eq!(assignments[0].location.line, 4);
        assert!(assignments[1].exactly);
    }

    #[test]
    fn resolves_assignment_values_once_during_lowering() {
        let rules = parsed_instance_rules(
            "Instance: example\nInstanceOf: Patient\n* active = true\n* gender = #female\n",
        );
        let mut resolved = 0;

        let program = SemanticProgram::lower_instance(&rules, |_| {
            resolved += 1;
            serde_json::Value::Null
        });

        assert_eq!(resolved, 2);
        assert_eq!(program.operations().len(), 2);
    }
}
