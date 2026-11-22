//! Context-free registry entry points for FHIR functions. Model-aware callers
//! use EvaluationContext with the evaluator to preserve the source node.
use super::host;
use crate::error::FhirPathResult;
use crate::evaluator::{EvaluationContext, FhirPathValue};
use std::collections::HashMap;

type ExtensionFunction =
    Box<dyn Fn(&FhirPathValue, &[FhirPathValue]) -> FhirPathResult<FhirPathValue> + Send + Sync>;

pub fn register_functions(functions: &mut HashMap<String, ExtensionFunction>) {
    for name in ["extension", "hasValue", "getValue"] {
        functions.insert(
            name.to_owned(),
            Box::new(move |target, params| {
                let context = EvaluationContext::new(target.to_json());
                host::invoke(name, target, params, &context)
            }),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::evaluator::metadata::{FhirModel, FhirNode};
    use serde_json::json;

    #[test]
    fn registry_uses_the_same_exact_urls_and_primitive_rules_as_the_evaluator() {
        let mut functions = HashMap::new();
        register_functions(&mut functions);
        let target =
            FhirPathValue::from_json(&json!({"extension":[{"url":"urn:x","valueString":"yes"}]}));
        assert!(
            functions["extension"](&target, &[FhirPathValue::String("urn".into())])
                .unwrap()
                .is_empty()
        );
        let ext =
            functions["extension"](&target, &[FhirPathValue::String("urn:x".into())]).unwrap();
        assert_eq!(ext.items().len(), 1);
        assert_eq!(
            functions["hasValue"](&ext, &[]).unwrap(),
            FhirPathValue::Boolean(false)
        );
        assert!(functions["getValue"](&ext, &[]).unwrap().is_empty());
        let code = FhirPathValue::Node(Box::new(FhirNode::primitive(
            "code",
            &json!("abc"),
            None,
            FhirModel::R4,
        )));
        assert_eq!(
            functions["hasValue"](&code, &[]).unwrap(),
            FhirPathValue::Boolean(true)
        );
        assert_eq!(
            functions["getValue"](&code, &[]).unwrap(),
            FhirPathValue::String("abc".into())
        );
    }
}
