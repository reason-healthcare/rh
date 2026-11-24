use super::FhirPathEvaluator;
use crate::ast::Expression;
use crate::error::{FhirPathError, FhirPathResult};
use crate::evaluator::operations::collection::CollectionEvaluator;
use crate::evaluator::operations::comparison::ComparisonEvaluator;
use crate::evaluator::{metadata::FhirModel, EvaluationContext, FhirPathValue};
use std::{borrow::Cow, cmp::Ordering};
use std::{collections::HashMap, hash::RandomState};

impl FhirPathEvaluator {
    /// Evaluate where function that filters collections based on boolean criteria
    pub(super) fn evaluate_where_function(
        &self,
        target: &FhirPathValue,
        parameters: &[Expression],
        context: &EvaluationContext,
    ) -> FhirPathResult<FhirPathValue> {
        let [criterion] = parameters else {
            return Err(FhirPathError::FunctionError {
                message: "where() function requires exactly one parameter".to_string(),
            });
        };

        let mut filtered = Vec::new();
        for (index, item) in target.items().iter().enumerate() {
            if self.matches_criterion(criterion, context, item, index, "where() criterion")? {
                filtered.push(item.clone());
            }
        }
        Ok(FhirPathValue::from_items(filtered))
    }

    /// iif(criterion, true-result[, otherwise-result]) with lazy evaluation.
    /// Evaluates criterion first; only evaluates the selected branch.
    pub(super) fn evaluate_iif_lazy(
        &self,
        target: &FhirPathValue,
        parameters: &[Expression],
        context: &EvaluationContext,
    ) -> FhirPathResult<FhirPathValue> {
        if parameters.len() < 2 || parameters.len() > 3 {
            return Err(FhirPathError::FunctionError {
                message: "iif() requires 2 or 3 parameters".to_string(),
            });
        }

        let focus = target
            .singleton("iif() input")?
            .cloned()
            .unwrap_or(FhirPathValue::Empty);
        let iif_ctx = context.with_this_value(focus);
        let criterion = self.evaluate_expression(&parameters[0], &iif_ctx)?;
        let is_truthy = criterion.singleton_boolean("iif() criterion")? == Some(true);

        // Lazy: only evaluate the selected branch.
        if is_truthy {
            self.evaluate_expression(&parameters[1], &iif_ctx)
        } else if let Some(otherwise) = parameters.get(2) {
            self.evaluate_expression(otherwise, &iif_ctx)
        } else {
            Ok(FhirPathValue::Empty)
        }
    }

    pub(super) fn evaluate_coalesce(
        &self,
        target: &FhirPathValue,
        parameters: &[Expression],
        context: &EvaluationContext,
    ) -> FhirPathResult<FhirPathValue> {
        if parameters.is_empty() {
            return Err(FhirPathError::FunctionError {
                message: "coalesce() requires at least one argument".into(),
            });
        }
        let context = context.with_invocation_focus(target.clone());
        for parameter in parameters {
            let result = self.evaluate_expression(parameter, &context)?;
            if !result.is_empty() {
                return Ok(result);
            }
        }
        Ok(FhirPathValue::Empty)
    }

    /// Sort stably using singleton keys. Evaluate later keys only for ties.
    pub(super) fn evaluate_sort_by_expression(
        &self,
        target: &FhirPathValue,
        parameters: &[Expression],
        context: &EvaluationContext,
    ) -> FhirPathResult<FhirPathValue> {
        let items = target.items();
        let key_specs: Vec<_> = parameters
            .iter()
            .map(|parameter| match parameter {
                Expression::SortKey { key, descending } => (key.as_ref(), *descending),
                parameter => (parameter, false),
            })
            .collect();

        let evaluate_key = |item: &FhirPathValue, expression: &Expression| {
            let item_context = context.with_unindexed_focus(item.clone());
            let result = self.evaluate_expression(expression, &item_context)?;
            Ok::<_, FhirPathError>(
                result
                    .singleton("sort() key")?
                    .cloned()
                    .unwrap_or(FhirPathValue::Empty),
            )
        };

        let mut keys = items
            .iter()
            .map(|item| {
                let key = if let Some((expression, _)) = key_specs.first() {
                    evaluate_key(item, expression)?
                } else {
                    item.clone()
                };
                Ok(vec![key])
            })
            .collect::<FhirPathResult<Vec<_>>>()?;

        let mut indices: Vec<usize> = (0..items.len()).collect();
        let mut scratch = indices.clone();
        CollectionEvaluator::sort_indices_by(&mut indices, &mut scratch, &mut |a, b| {
            for key_index in 0..key_specs.len().max(1) {
                if key_index >= keys[a].len() {
                    keys[a].push(evaluate_key(&items[a], key_specs[key_index].0)?);
                }
                if key_index >= keys[b].len() {
                    keys[b].push(evaluate_key(&items[b], key_specs[key_index].0)?);
                }
                let order = CollectionEvaluator::compare_sort_keys(
                    &keys[a][key_index],
                    &keys[b][key_index],
                )?;
                if order != Ordering::Equal {
                    let descending = key_specs.get(key_index).is_some_and(|key| key.1);
                    return Ok(if descending { order.reverse() } else { order });
                }
            }
            Ok(Ordering::Equal)
        })?;
        Ok(FhirPathValue::from_items(
            indices
                .into_iter()
                .map(|index| items[index].clone())
                .collect(),
        ))
    }

    /// exists(criterion) — returns true if any element satisfies the criterion.
    pub(super) fn evaluate_exists_with_criterion(
        &self,
        target: &FhirPathValue,
        parameters: &[Expression],
        context: &EvaluationContext,
    ) -> FhirPathResult<FhirPathValue> {
        let [criterion] = parameters else {
            return Err(FhirPathError::FunctionError {
                message: "exists() accepts at most one parameter (criterion)".to_string(),
            });
        };
        for (index, item) in target.items().iter().enumerate() {
            if self.matches_criterion(criterion, context, item, index, "exists() criterion")? {
                return Ok(FhirPathValue::Boolean(true));
            }
        }
        Ok(FhirPathValue::Boolean(false))
    }

    /// all(criterion) — returns true if every element satisfies the criterion.
    pub(super) fn evaluate_all_with_criterion(
        &self,
        target: &FhirPathValue,
        parameters: &[Expression],
        context: &EvaluationContext,
    ) -> FhirPathResult<FhirPathValue> {
        let [criterion] = parameters else {
            return Err(FhirPathError::FunctionError {
                message: "all() requires exactly one parameter (criterion)".to_string(),
            });
        };
        for (index, item) in target.items().iter().enumerate() {
            if !self.matches_criterion(criterion, context, item, index, "all() criterion")? {
                return Ok(FhirPathValue::Boolean(false));
            }
        }
        Ok(FhirPathValue::Boolean(true))
    }

    pub(super) fn evaluate_select_function(
        &self,
        target: &FhirPathValue,
        parameters: &[Expression],
        context: &EvaluationContext,
    ) -> FhirPathResult<FhirPathValue> {
        let [projection] = parameters else {
            return Err(FhirPathError::function_error(
                "select() function requires exactly one parameter",
            ));
        };
        self.project_items(target, projection, context)
    }

    fn project_items(
        &self,
        target: &FhirPathValue,
        projection: &Expression,
        context: &EvaluationContext,
    ) -> FhirPathResult<FhirPathValue> {
        let mut selected = Vec::new();
        for (index, item) in target.items().iter().enumerate() {
            selected.extend(
                self.evaluate_indexed(projection, context, item, index)?
                    .into_items(),
            );
        }
        Ok(FhirPathValue::from_items(selected))
    }

    fn evaluate_indexed(
        &self,
        expression: &Expression,
        context: &EvaluationContext,
        item: &FhirPathValue,
        index: usize,
    ) -> FhirPathResult<FhirPathValue> {
        let item_context = context.with_this_and_index(item.clone(), index as i64);
        self.evaluate_expression(expression, &item_context)
    }

    fn matches_criterion(
        &self,
        criterion: &Expression,
        context: &EvaluationContext,
        item: &FhirPathValue,
        index: usize,
        operation: &str,
    ) -> FhirPathResult<bool> {
        let result = self.evaluate_indexed(criterion, context, item, index)?;
        Ok(result.singleton_boolean(operation)? == Some(true))
    }

    pub(super) fn evaluate_trace_function(
        &self,
        target: FhirPathValue,
        parameters: &[Expression],
        context: &EvaluationContext,
    ) -> FhirPathResult<FhirPathValue> {
        let (name, projection) = match parameters {
            [name] => (name, None),
            [name, projection] => (name, Some(projection)),
            _ => {
                return Err(FhirPathError::function_error(
                    "trace() function requires 1 or 2 parameters (name, [projection])",
                ));
            }
        };

        let name = self.evaluate_expression(name, context)?;
        let name = name
            .singleton("trace() name")?
            .and_then(|value| value.system_value().as_str())
            .ok_or_else(|| {
                FhirPathError::function_error("trace() name parameter must evaluate to a string")
            })?;
        let value = match projection {
            Some(projection) => Cow::Owned(self.project_items(&target, projection, context)?),
            None => Cow::Borrowed(&target),
        };
        let debug_value = format!("{:?}", value.as_ref());
        if std::env::var_os("RH_FHIRPATH_TRACE_STDERR").is_some() {
            eprintln!("[TRACE:{name}] {debug_value}");
        }
        context.add_trace_log(name.to_owned(), debug_value);
        Ok(target)
    }

    /// Project each generation, returning only the projected items.
    pub(super) fn evaluate_repeat_function(
        &self,
        target: &FhirPathValue,
        name: &str,
        parameters: &[Expression],
        context: &EvaluationContext,
    ) -> FhirPathResult<FhirPathValue> {
        let [projection] = parameters else {
            return Err(FhirPathError::FunctionError {
                message: format!("{name}() function requires exactly one parameter"),
            });
        };
        if target.is_empty() {
            return Ok(FhirPathValue::Empty);
        }
        let mut seen = (name != "repeatAll").then(RepeatSeen::default);
        let mut result = Vec::new();
        let mut pending: std::collections::VecDeque<_> = target
            .items()
            .iter()
            .cloned()
            .map(|item| (item, 0))
            .collect();
        let mut item_context = context.clone();
        while let Some((item, depth)) = pending.pop_front() {
            item_context.set_unindexed_focus(item);
            let projected = self.evaluate_expression(projection, &item_context)?;
            for value in projected.into_items() {
                if seen
                    .as_mut()
                    .is_some_and(|seen| !seen.insert(&result, &value, context.model))
                {
                    continue;
                }
                if depth >= 1_000 || result.len() >= 100_000 {
                    return Err(FhirPathError::EvaluationError {
                        message: format!("{name}() exceeded its recursive projection limit"),
                    });
                }
                pending.push_back((value.clone(), depth + 1));
                result.push(value);
            }
        }
        Ok(FhirPathValue::from_items(result))
    }

    /// `aggregate(expression [, init])` — fold a collection using `$this`
    /// (current item) and `$total` (accumulator).
    pub(super) fn evaluate_aggregate_function(
        &self,
        target: &FhirPathValue,
        parameters: &[Expression],
        context: &EvaluationContext,
    ) -> FhirPathResult<FhirPathValue> {
        if parameters.is_empty() || parameters.len() > 2 {
            return Err(FhirPathError::FunctionError {
                message: "aggregate() requires 1–2 parameters: expression [, init]".to_string(),
            });
        }
        let expr = &parameters[0];
        let init = if parameters.len() == 2 {
            self.evaluate_expression(&parameters[1], context)?
        } else {
            FhirPathValue::Empty
        };

        let mut total = init;
        for (idx, item) in target.items().iter().cloned().enumerate() {
            let iter_ctx = context.with_aggregate_vars_and_index(item, total, idx as i64);
            total = self.evaluate_expression(expr, &iter_ctx)?;
        }
        Ok(total)
    }
}

#[derive(Default)]
struct RepeatSeen {
    hash: RandomState,
    buckets: HashMap<u64, Vec<usize>>,
    unindexed: Vec<usize>,
}

impl RepeatSeen {
    fn insert(
        &mut self,
        result: &[FhirPathValue],
        value: &FhirPathValue,
        model: FhirModel,
    ) -> bool {
        let Some((system, logical)) =
            ComparisonEvaluator::equality_fingerprints(value, &self.hash, model)
        else {
            if result
                .iter()
                .any(|item| FhirPathValue::equals_static(item, value))
            {
                return false;
            }
            self.unindexed.push(result.len());
            return true;
        };
        if self
            .unindexed
            .iter()
            .chain(
                [Some(system), logical]
                    .into_iter()
                    .flatten()
                    .filter_map(|key| self.buckets.get(&key))
                    .flatten(),
            )
            .any(|&index| FhirPathValue::equals_static(&result[index], value))
        {
            return false;
        }
        self.buckets.entry(system).or_default().push(result.len());
        if let Some(logical) = logical {
            self.buckets.entry(logical).or_default().push(result.len());
        }
        true
    }
}
