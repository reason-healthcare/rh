//! CQL query evaluation — single-source `from/where/let/return/sort`.
//!
//! Provides [`eval_query`] which is called by the engine when it encounters
//! a [`crate::elm::expression::Query`] node.

use std::collections::BTreeMap;

use super::context::EvalError;
use super::lists::distinct;
use super::operators::cql_compare;
use super::value::Value;
use crate::elm::{Query, SortClause};

// ---------------------------------------------------------------------------
// Public entry point
// ---------------------------------------------------------------------------

/// Evaluates a single-source ELM [`Query`] node, returning a `Value::List`.
///
/// `eval_expr` is a callback that evaluates a single ELM expression in the
/// context of the given binding environment.
pub fn eval_query<F>(
    query: &Query,
    bindings: &BTreeMap<String, Value>,
    eval_expr: &mut F,
) -> Result<Value, EvalError>
where
    F: FnMut(&crate::elm::Expression, &BTreeMap<String, Value>) -> Result<Value, EvalError>,
{
    // Evaluate sources — currently supports a single aliased source.
    let source = query
        .source
        .first()
        .ok_or_else(|| EvalError::General("Query requires at least one source".to_string()))?;

    let source_alias = source.alias.as_deref().unwrap_or("$this");
    let source_expr = source
        .expression
        .as_ref()
        .ok_or_else(|| EvalError::General("Query source has no expression".to_string()))?;

    let source_value = eval_expr(source_expr, bindings)?;

    let rows = match source_value {
        Value::List(v) => v,
        Value::Null => return Ok(Value::List(vec![])),
        other => vec![other], // single-element source is wrapped in a list
    };

    let mut result: Vec<Value> = Vec::new();

    'row: for row in &rows {
        // Build per-row bindings: add source alias.
        let mut row_bindings = bindings.clone();
        row_bindings.insert(source_alias.to_string(), row.clone());

        // Evaluate let clauses and add them to bindings.
        for let_clause in &query.let_clause {
            if let Some(ref id) = let_clause.identifier {
                if let Some(ref expr) = let_clause.expression {
                    let val = eval_expr(expr, &row_bindings)?;
                    row_bindings.insert(id.clone(), val);
                }
            }
        }

        // Evaluate relationship clauses (with/without).
        for rel in &query.relationship {
            let rel_type = rel.relationship_type.as_deref().unwrap_or("with");
            let rel_alias = rel.alias.as_deref().unwrap_or("$relSource");
            let rel_source_expr = match &rel.expression {
                Some(e) => e,
                None => continue,
            };

            let rel_source = eval_expr(rel_source_expr, &row_bindings)?;
            let rel_rows = match rel_source {
                Value::List(v) => v,
                Value::Null => vec![],
                other => vec![other],
            };

            let mut any_match = false;
            for rel_row in &rel_rows {
                let mut rel_bindings = row_bindings.clone();
                rel_bindings.insert(rel_alias.to_string(), rel_row.clone());

                let sat = match &rel.such_that {
                    None => true,
                    Some(st) => eval_expr(st, &rel_bindings)? == Value::Boolean(true),
                };
                if sat {
                    any_match = true;
                    break;
                }
            }

            match rel_type {
                "with"
                    if !any_match => {
                        continue 'row; // no match → skip this row
                    }
                "without"
                    if any_match => {
                        continue 'row; // match found → skip this row
                    }
                _ => {}
            }
        }

        // Evaluate where clause.
        if let Some(ref where_expr) = query.where_clause {
            let keep = eval_expr(where_expr, &row_bindings)?;
            if keep != Value::Boolean(true) {
                continue;
            }
        }

        // Evaluate return clause.
        let return_value = match &query.return_clause {
            None => row.clone(),
            Some(ret) => match &ret.expression {
                None => row.clone(),
                Some(ret_expr) => eval_expr(ret_expr, &row_bindings)?,
            },
        };

        result.push(return_value);
    }

    // Apply distinct if requested.
    let distinct_requested = query
        .return_clause
        .as_ref()
        .and_then(|r| r.distinct)
        .unwrap_or(false);

    let result_value = if distinct_requested {
        match distinct(&Value::List(result))? {
            Value::List(v) => v,
            _ => unreachable!(),
        }
    } else {
        result
    };

    // Apply sort clause.
    let sorted = if let Some(ref sort_clause) = query.sort {
        apply_sort(result_value, sort_clause)?
    } else {
        result_value
    };

    Ok(Value::List(sorted))
}

// ---------------------------------------------------------------------------
// Sort helpers
// ---------------------------------------------------------------------------

fn apply_sort(mut items: Vec<Value>, sort_clause: &SortClause) -> Result<Vec<Value>, EvalError> {
    use crate::elm::SortDirection;
    use std::cmp::Ordering;

    for sort_item in sort_clause.by.iter().rev() {
        let desc = sort_item.direction == Some(SortDirection::Desc);

        // Path-based sort (ByColumn)
        if let Some(ref _path) = sort_item.path {
            items.sort_by(|a, b| {
                let va = extract_path(a, _path);
                let vb = extract_path(b, _path);
                let ord = match (&va, &vb) {
                    (Some(x), Some(y)) => cql_compare(x, y).unwrap_or(Ordering::Equal),
                    (None, None) => Ordering::Equal,
                    (None, _) => Ordering::Less,
                    (_, None) => Ordering::Greater,
                };
                if desc {
                    ord.reverse()
                } else {
                    ord
                }
            });
        } else {
            // ByValue (sort by the element itself)
            items.sort_by(|a, b| {
                let ord = cql_compare(a, b).unwrap_or(Ordering::Equal);
                if desc {
                    ord.reverse()
                } else {
                    ord
                }
            });
        }
    }

    Ok(items)
}

/// Extract a field from a Tuple or pass through for scalars.
fn extract_path<'a>(v: &'a Value, path: &str) -> Option<&'a Value> {
    match v {
        Value::Tuple(fields) => fields.get(path),
        _ => Some(v),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::elm::{
        AliasedQuerySource, BinaryExpression, Expression, Literal, Property, Query, ReturnClause,
    };

    fn int_list_expr(values: &[i64]) -> Expression {
        Expression::List(crate::elm::ListExpr {
            elements: values
                .iter()
                .map(|&v| {
                    Expression::Literal(Literal {
                        value: Some(v.to_string()),
                        value_type: Some("Integer".to_string()),
                        ..Default::default()
                    })
                })
                .collect(),
            ..Default::default()
        })
    }

    fn eval_expr(
        expr: &Expression,
        bindings: &BTreeMap<String, Value>,
    ) -> Result<Value, EvalError> {
        match expr {
            Expression::Literal(lit) => {
                let raw = lit.value.as_deref().unwrap_or("");
                match lit.value_type.as_deref() {
                    Some("Integer") => Ok(Value::Integer(raw.parse().unwrap_or(0))),
                    Some("Boolean") => Ok(Value::Boolean(raw == "true")),
                    Some("String") => Ok(Value::String(raw.to_string())),
                    _ => Ok(Value::Null),
                }
            }
            Expression::List(list) => {
                let items: Result<Vec<_>, _> = list
                    .elements
                    .iter()
                    .map(|e| eval_expr(e, bindings))
                    .collect();
                Ok(Value::List(items?))
            }
            Expression::Property(prop) => {
                let scope = prop.scope.as_deref().or(prop.source.as_ref().and_then(|s| {
                    if let Expression::IdentifierRef(id) = s.as_ref() {
                        id.name.as_deref()
                    } else {
                        None
                    }
                }));
                let base = match scope {
                    Some(alias) => bindings.get(alias).cloned().unwrap_or(Value::Null),
                    None => Value::Null,
                };
                let path = prop.path.as_deref().unwrap_or("");
                match base {
                    Value::Tuple(fields) => Ok(fields.get(path).cloned().unwrap_or(Value::Null)),
                    _ => Ok(Value::Null),
                }
            }
            Expression::Equal(bin) => {
                if bin.operand.len() == 2 {
                    let a = eval_expr(&bin.operand[0], bindings)?;
                    let b = eval_expr(&bin.operand[1], bindings)?;
                    Ok(Value::Boolean(a == b))
                } else {
                    Ok(Value::Null)
                }
            }
            Expression::IdentifierRef(id) => {
                let name = id.name.as_deref().unwrap_or("");
                Ok(bindings.get(name).cloned().unwrap_or(Value::Null))
            }
            _ => Ok(Value::Null),
        }
    }

    /// Scenario: Simple query with where — only matching rows are returned.
    #[test]
    fn query_where_filters_rows() {
        // from {10, 20, 30} X where X = 20
        let query = Query {
            source: vec![AliasedQuerySource {
                alias: Some("X".to_string()),
                expression: Some(Box::new(int_list_expr(&[10, 20, 30]))),
            }],
            where_clause: Some(Box::new(Expression::Equal(BinaryExpression {
                operand: vec![
                    Expression::IdentifierRef(crate::elm::IdentifierRef {
                        name: Some("X".to_string()),
                        ..Default::default()
                    }),
                    Expression::Literal(Literal {
                        value: Some("20".to_string()),
                        value_type: Some("Integer".to_string()),
                        ..Default::default()
                    }),
                ],
                ..Default::default()
            }))),
            ..Default::default()
        };

        let bindings = BTreeMap::new();
        let result = eval_query(&query, &bindings, &mut eval_expr).unwrap();
        assert_eq!(result, Value::List(vec![Value::Integer(20)]));
    }

    /// Scenario: Query with return projection — result is derived from each row.
    #[test]
    fn query_return_projects_field() {
        // from {{status: 'active'}, {status: 'inactive'}} O return O.status
        use std::collections::BTreeMap as TupleMap;

        let mut active = TupleMap::new();
        active.insert("status".to_string(), Value::String("active".to_string()));
        let mut inactive = TupleMap::new();
        inactive.insert("status".to_string(), Value::String("inactive".to_string()));

        // Inject source as a pre-evaluated list binding "$src".
        let mut bindings = BTreeMap::new();
        bindings.insert(
            "$src".to_string(),
            Value::List(vec![
                Value::Tuple(active.clone()),
                Value::Tuple(inactive.clone()),
            ]),
        );

        let query = Query {
            source: vec![AliasedQuerySource {
                alias: Some("O".to_string()),
                expression: Some(Box::new(Expression::IdentifierRef(
                    crate::elm::IdentifierRef {
                        name: Some("$src".to_string()),
                        ..Default::default()
                    },
                ))),
            }],
            return_clause: Some(ReturnClause {
                distinct: None,
                expression: Some(Box::new(Expression::Property(Property {
                    path: Some("status".to_string()),
                    scope: Some("O".to_string()),
                    ..Default::default()
                }))),
            }),
            ..Default::default()
        };

        let result = eval_query(&query, &bindings, &mut eval_expr).unwrap();
        assert_eq!(
            result,
            Value::List(vec![
                Value::String("active".to_string()),
                Value::String("inactive".to_string()),
            ])
        );
    }
}
